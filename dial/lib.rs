pub mod builtin;
pub mod env;
pub mod parse;
pub mod sexpr;

use anyhow::Result;
use thiserror::Error;

pub use env::Env;
use parse::ParseResult;
use sexpr::{Atom, DialVal};

macro_rules! extract_atom_val {
	($val:ident, $b:block, $($p:path)|+) => {
		match $val {
			DialVal::Atom(a) match a {
				$($p(v))|+ => v,
				_ => $b,
			}
			_ => $b
		}
	}
}

pub type EvalResult = Result<DialVal, EvalError>;

#[derive(Error, Debug, PartialEq)]
pub enum EvalError {
    #[error("undefined value: {0}")]
    Undefined(String),
    #[error("TypeError: expected {0}")]
    TypeError(String),
    #[error("ArityError: wrong number of args ({0})")]
    ArityError(usize),
    #[error("InvalidArgumentError: {0}")]
    InvalidArgumentError(String),
}

pub fn read(input: String) -> ParseResult<Vec<DialVal>> {
    parse::parse_program(input)
}

pub fn eval(val: DialVal, env: &mut Env) -> EvalResult {
    println!("val {:?}", val);
    match val {
        DialVal::Atom(a) => match a {
            Atom::Sym(s) => env
                .get_value(s.clone())
                .clone()
                .ok_or_else(|| EvalError::Undefined(format!("no such symbol {}", s).into())),
            _ => Ok(a.clone().into()),
        },
        DialVal::List(l) => {
            if l.is_empty() {
                Ok(DialVal::List(vec![]))
            } else {
                let (first, rest) = l.split_at(1);

                let first = first.get(0).unwrap();
                let rest: Result<Vec<DialVal>, EvalError> =
                    rest.iter().map(|val| eval(val.clone(), env)).collect();

                match eval(first.clone(), env) {
                    Ok(dv) => match dv {
                        DialVal::Atom(a) => match a {
                            Atom::Builtin { func, .. } => func(rest?.as_slice(), env),
                            _ => Err(EvalError::TypeError(format!("{} is not a function", first))),
                        },
                        _ => Err(EvalError::TypeError(format!("{} is not a function", first))),
                    },
                    Err(e) => Err(e),
                }
            }
        }
        DialVal::Vec(v) => {
            if v.is_empty() {
                Ok(DialVal::Vec(vec![]))
            } else {
                let new_vec: Result<Vec<DialVal>, EvalError> =
                    v.iter().map(|val| eval(val.clone(), env)).collect();

                match new_vec {
                    Ok(v) => Ok(DialVal::Vec(v)),
                    Err(e) => Err(e),
                }
            }
        }
    }
}

pub fn print(val: EvalResult) -> String {
    todo!();
}

#[cfg(test)]
mod mal_tests {
    use super::*;

    #[test]
    fn step_2_eval() {
        let inputs = vec![
            "1",
            "(+ 1 2 3)",
            "(- 5 4 1)",
            "(* 0.5 0.5 0.5)",
            "(/ 1 2 3)",
            "(+ 2 3)",
            "(+ 2 (* 3 4))",
        ];

        let mut env = Env::default();

        let results: Vec<EvalResult> = inputs
            .iter()
            .map(|input| eval(parse::parse_sexpr(input.to_string()).unwrap(), &mut env))
            .collect();

        assert_eq!(
            results,
            vec![
                Ok(DialVal::Atom(Atom::Int(1))),
                Ok(DialVal::Atom(Atom::Float(6.0))),
                Ok(DialVal::Atom(Atom::Float(0.0))),
                Ok(DialVal::Atom(Atom::Float(0.125))),
                Ok(DialVal::Atom(Atom::Float(1.0 / 6.0))),
                Ok(DialVal::Atom(Atom::Float(5.0))),
                Ok(DialVal::Atom(Atom::Float(14.0))),
            ]
        )
    }

    #[test]
    fn step_3_def() {
        let inputs = vec!["(def foo 123)"];
    }
}
