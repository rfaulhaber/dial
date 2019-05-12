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
                        Atom::Lambda(l) => unimplemented!(), // do user-defined lookup
                        _ => Err("invalid form".to_string()),
                    },
                    _ => Err("invalid form".to_string()),
                }
            }
        }
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
