// use super::core::{get_builtin, BuiltinFunc};
use super::parser::{Atom, DialParser, Expr, Rule};
use crate::environment::env::Env;
use log::Level;
use std::cell::RefCell;
use std::error;

pub type EvalResult = Result<Expr, String>;

pub struct Interpreter {
    env: RefCell<Env>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            env: RefCell::new(Env::default()),
        }
    }

    pub fn eval(&self, expr: Expr) -> EvalResult {
        match expr {
            Expr::Atom(atom) => match atom {
                Atom::Symbol(symbol) => {
                    let lookup = self.env.borrow().get(&symbol);

                    match lookup {
                        Some(result) => Ok(result),
                        None => Err(format!("could not find symbol {}", symbol)),
                    }
                }
                Atom::Identifier(id) => {
                    let lookup = self.env.borrow().get(&id);

                    match lookup {
                        Some(result) => Ok(result),
                        None => Err(format!("cannot resolve symbol: {}", id)),
                    }
                }
                _ => Ok(atom.into()),
            },
            Expr::List(list) => {
                let first = &list[0];

                if is_list_special(first) {
                    match get_list_special(first).as_str() {
                        "if" => {
                            if list[1..].len() > 2 {
                                return Err("too many arguments for if".to_string());
                            }

                            let cond = self.eval(list[1].clone())?;

                            if is_false_or_nil(&cond) {
                                self.eval(list[3].clone())
                            } else {
                                self.eval(list[2].clone())
                            }
                        }
                        "do" => {
                            let args = &list[1..];

                            args.iter()
                                .map(|expr| self.eval(expr.clone()))
                                .last()
                                .unwrap()
                        }
                        "fn" => {
                            unimplemented!();
                        }
                        _ => unreachable!(),
                    }
                } else {
                    let first_result = self.eval(list[0].clone())?;

                    let args = &list[1..];

                    match first_result {
                        Expr::Atom(a) => match a {
                            Atom::Func(f) => {
                                let args_eval: Result<Vec<Expr>, String> = args
                                    .iter()
                                    .map(|arg_expr| self.eval(arg_expr.clone()))
                                    .collect();

                                f(&args_eval?)
                            }
                            Atom::Lambda(l) => Err("lambda lookups are unimplemented".to_string()), // do user-defined lookup
                            _ => Err("invalid form".to_string()),
                        },
                        _ => Err("invalid form".to_string()),
                    }
                }
            }
        }
    }
}

fn is_list_special(expr: &Expr) -> bool {
    match expr {
        Expr::Atom(a) => match a {
            Atom::Symbol(symbol) => match symbol.as_str() {
                "if" | "do" | "fn" => true,
                _ => false,
            },
            _ => false,
        },
        _ => false,
    }
}

fn get_list_special(expr: &Expr) -> String {
    match expr {
        Expr::Atom(a) => match a {
            Atom::Symbol(symbol) => symbol.clone(),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn is_false_or_nil(expr: &Expr) -> bool {
    match expr {
        Expr::Atom(a) => match a {
            Atom::Boolean(b) => !b,
            Atom::Nil => true,
            _ => false,
        },
        _ => false,
    }
}

#[cfg(test)]
mod interpreter_test {
    use super::*;
    use pest::Parser;

    #[test]
    fn test_function_call() {
        let parsed = DialParser::parse(Rule::list, "(* 2 (+ 3 4 5))").unwrap();
        let expr = Expr::from(parsed);

        let int = Interpreter::new();
        let result = int.eval(expr);

        assert!(result.is_ok());
        assert_eq!(Expr::Atom(Atom::Integer(24)), result.unwrap());
    }
}
