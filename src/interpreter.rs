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
            Expr::Vector(vec) => {
                if vec.is_empty() {
                    return Ok(Expr::Vector(Vec::new()));
                }

                let vec_eval: Result<Vec<Expr>, String> =
                    vec.iter().map(|expr| self.eval(expr.clone())).collect();

                match vec_eval {
                    Ok(exprs) => Ok(Expr::Vector(exprs)),
                    Err(err) => Err(err),
                }
            }
            Expr::List(list) => {
                if list.is_empty() {
                    return Ok(Expr::List(Vec::new()));
                }

                let first = &list[0];

                // TODO add validation!!!
                if is_list_special(first) {
                    match get_list_special(first).as_str() {
                        "if" => {
                            if list[1..].len() > 3 {
                                return Err("too many arguments for if".to_string());
                            }

                            let cond = self.eval(list[1].clone())?;

                            if is_false_or_nil(&cond) {
                                match list.get(3) {
                                    Some(expr) => self.eval(expr.clone()),
                                    None => Ok(Expr::Atom(Atom::Nil)),
                                }
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
                        "def" => {
                            let first_atom = &list[1].as_atom();
                            let symbol = match first_atom {
                                Some(atom) => match atom {
                                    Atom::Identifier(id) => id,
                                    _ => {
                                        return Err(
                                            "first argument of def must be valid identifier"
                                                .to_string(),
                                        )
                                    }
                                },
                                None => {
                                    return Err("first argument of def must be valid identifier"
                                        .to_string())
                                }
                            };

                            let def = self.eval(list[2].clone())?;

                            self.env.borrow_mut().set(symbol, def.clone());

                            Ok(def)
                        }
                        "let" => unimplemented!(),
                        "fn" => {
                            // TODO validation on list size
                            let params = &list[1];
                            let body = &list[2];
                            unimplemented!();
                        }
                        _ => unreachable!(),
                    }
                } else {
                    let first_result = self.eval(list[0].clone())?;

                    let args = &list[1..];

                    match first_result {
                        Expr::Atom(a) => match a {
                            Atom::Macro(f) => {
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

    fn eval_let(&self, expr: Expr) -> EvalResult {
        unimplemented!();
    }
}

fn is_list_special(expr: &Expr) -> bool {
    match expr {
        Expr::Atom(a) => match a {
            Atom::Symbol(symbol) => match symbol.as_str() {
                "if" | "do" | "fn" | "def" | "let" => true,
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

    #[test]
    fn do_evals_all_returns_last() {
        let parsed = DialParser::parse(Rule::list, "(do (+ 1 2) (+ 3 4) (* 5 6))").unwrap();
        let expr = Expr::from(parsed);

        let int = Interpreter::new();
        let result = int.eval(expr);

        assert!(result.is_ok());
        assert_eq!(Expr::Atom(Atom::Integer(30)), result.unwrap());
    }

    #[test]
    fn if_evals_true_expr_when_true() {
        let parsed = DialParser::parse(Rule::list, r#"(if true "true" "false")"#).unwrap();
        let expr = Expr::from(parsed);

        let int = Interpreter::new();
        let result = int.eval(expr);

        assert!(result.is_ok());
        assert_eq!(
            Expr::Atom(Atom::String(String::from(r#""true""#))),
            result.unwrap()
        );
    }

    #[test]
    fn if_evals_false_when_expr_false() {
        let parsed = DialParser::parse(Rule::list, r#"(if false "true" "false")"#).unwrap();
        let expr = Expr::from(parsed);

        let int = Interpreter::new();
        let result = int.eval(expr);

        assert!(result.is_ok());
        assert_eq!(
            Expr::Atom(Atom::String(String::from(r#""false""#))),
            result.unwrap()
        );
    }

    #[test]
    fn if_evals_false_when_expr_nil() {
        let parsed = DialParser::parse(Rule::list, r#"(if nil "true" "false")"#).unwrap();
        let expr = Expr::from(parsed);

        let int = Interpreter::new();
        let result = int.eval(expr);

        assert!(result.is_ok());
        assert_eq!(
            Expr::Atom(Atom::String(String::from(r#""false""#))),
            result.unwrap()
        );
    }

    #[test]
    fn def_assigns_value_and_persists() {
        let parsed = DialParser::parse(Rule::list, r#"(do (def hello "world") hello)"#).unwrap();
        let expr = Expr::from(parsed);

        let int = Interpreter::new();
        let result = int.eval(expr);

        assert!(result.is_ok());
        assert_eq!(
            Expr::Atom(Atom::String(String::from(r#""world""#))),
            result.unwrap()
        );
    }

    #[test]
    fn let_scopes_value() {}

    #[test]
    fn let_overrides_value() {}
}
