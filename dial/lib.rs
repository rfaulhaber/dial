pub mod builtin;
pub mod parse;
pub mod sexpr;

use std::{cell::RefCell, collections::HashMap};

use anyhow::Result;
use thiserror::Error;

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

#[derive(Clone)]
pub struct Env {
    symbol_map: RefCell<HashMap<String, DialVal>>,
    scope: Option<Box<Env>>,
}

impl Default for Env {
    fn default() -> Self {
        let mut root = HashMap::new();

        root.insert(
            "+".into(),
            DialVal::Atom(Atom::Fn {
                name: "+".into(),
                func: builtin::add,
            }),
        );

        root.insert(
            "-".into(),
            DialVal::Atom(Atom::Fn {
                name: "-".into(),
                func: builtin::sub,
            }),
        );

        root.insert(
            "*".into(),
            DialVal::Atom(Atom::Fn {
                name: "*".into(),
                func: builtin::mul,
            }),
        );

        root.insert(
            "/".into(),
            DialVal::Atom(Atom::Fn {
                name: "/".into(),
                func: builtin::div,
            }),
        );

        Env {
            symbol_map: RefCell::new(root),
            scope: None,
        }
    }
}

impl Env {
    pub fn with_scope(scope: Env) -> Env {
        Env {
            symbol_map: RefCell::new(HashMap::new()),
            scope: Some(Box::new(scope)),
        }
    }

    pub fn get_value(&self, sym: String) -> Option<DialVal> {
        let map = self.symbol_map.borrow();

        let res = map.get(&sym);

        match res {
            Some(val) => Some(val.clone()),
            None => match &self.scope {
                Some(scope) => scope.get_value(sym),
                None => None,
            },
        }
    }

    pub fn set_value(&self, sym: String, val: DialVal) {
        self.symbol_map.borrow_mut().insert(sym, val);
    }
}

pub fn read(input: String) -> ParseResult<Vec<DialVal>> {
    parse::parse_program(input)
}

pub fn eval(val: DialVal, env: &mut Env) -> EvalResult {
    match val {
        DialVal::Atom(a) => match a {
            Atom::Sym(s) => env
                .get_value(s)
                .clone()
                .ok_or_else(|| EvalError::Undefined("no such symbol".into())),
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
                            Atom::Fn { func, .. } => func(rest?.as_slice()),
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
}
