pub mod env;
pub mod parse;
pub mod sexpr;

use std::{cell::RefCell, collections::HashMap};

use anyhow::Result;
use thiserror::Error;

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
}

#[derive(Clone)]
pub struct Env {
    symbol_map: HashMap<String, DialVal>,
    scope: Option<Box<Env>>,
}

impl Default for Env {
    fn default() -> Self {
        let mut root = HashMap::new();

        root.insert(
            "+".into(),
            DialVal::Atom(Atom::Fn {
                name: "+".into(),
                func: env::add,
            }),
        );

        root.insert(
            "-".into(),
            DialVal::Atom(Atom::Fn {
                name: "-".into(),
                func: env::sub,
            }),
        );

        root.insert(
            "*".into(),
            DialVal::Atom(Atom::Fn {
                name: "*".into(),
                func: env::mul,
            }),
        );

        root.insert(
            "/".into(),
            DialVal::Atom(Atom::Fn {
                name: "/".into(),
                func: env::div,
            }),
        );

        Env {
            symbol_map: root,
            scope: None,
        }
    }
}

impl Env {
    pub fn with_scope(scope: Env) -> Env {
        Env {
            symbol_map: HashMap::new(),
            scope: Some(Box::new(scope)),
        }
    }

    pub fn get_value(&self, sym: String) -> Option<&DialVal> {
        self.symbol_map.get(&sym).or_else(|| {
            if let Some(scope) = &self.scope {
                scope.get_value(sym)
            } else {
                None
            }
        })
    }
}

pub fn read(input: String) -> EvalResult {
    unimplemented!();
}

pub fn eval(val: DialVal, env: &mut Env) -> EvalResult {
    match val {
        DialVal::Atom(a) => match a {
            Atom::Sym(s) => env
                .get_value(s)
                .cloned()
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
