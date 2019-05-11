// use super::core::{get_builtin, BuiltinFunc};
use super::parser::{Atom, DialParser, Expr, Rule};
use crate::environment::env::Env;
use log::Level;
use std::cell::RefCell;
use std::error;

pub type EvalResult = Result<Expr, &'static str>;

pub struct Interpreter {
    env: RefCell<Env>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            env: RefCell::new(Env::new()),
        }
    }

    pub fn eval(&self, expr: Expr) -> EvalResult {
        match expr {
            Expr::Atom(_) => Ok(expr),
            Expr::List(list) => {
                let first_result = self.eval(list[0].clone())?;

                let args = &list[1..];

                match first_result {
                    Expr::Atom(a) => match a {
                        Atom::Func(f) => {
                            let args_eval: Result<Vec<Expr>, &str> = args
                                .iter()
                                .map(|arg_expr| self.eval(arg_expr.clone()))
                                .collect();

                            f(&args_eval?)
                        }
                        Atom::Lambda(l) => unimplemented!(), // do user-defined lookup
                        _ => Err("invalid form"),
                    },
                    _ => Err("invalid form"),
                }
            }
        }
    }
}

// #[cfg(test)]
// mod interpreter_test {
//     use super::*;
//     use pest::Parser;

//     #[test]
//     fn test_function_call() {
//         let mut parsed = DialParser::parse(Rule::list, "(* 2 (+ 3 4 5))").unwrap();
//         let ast = Sexpr::from_pair(parsed.next().unwrap());

//         let int = Interpreter::new();
//         let result = int.eval(ast);

//         assert!(result.is_ok());
//         assert_eq!(DialValue::Integer(24), result.unwrap());
//     }
// }
