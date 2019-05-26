// use super::core::{get_builtin, BuiltinFunc};
use super::parser::{Atom, DialParser, Expr, Lambda, Rule};
use crate::environment::env::Env;
use log::Level;
use std::cell::RefCell;
use std::error;

// TODO remove all direct vector references, safely use .get() instead
// TODO refactor using this: https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#destructuring-nested-structs-and-enums

pub type EvalResult = Result<Expr, String>;

pub struct Interpreter {
    env: Env,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            env: Env::default(),
        }
    }

    // TODO please god make this method shorter
    pub fn eval(&mut self, expr: Expr) -> EvalResult {
        match expr {
            Expr::Atom(atom) => match atom {
                Atom::Symbol(symbol) => {
                    let lookup = self.env.get(&symbol);

                    match lookup {
                        Some(result) => Ok(result),
                        None => Err(format!("cannot resolve symbol: {}", symbol)),
                    }
                }
                Atom::Identifier(id) => {
                    let lookup = self.env.get(&id);

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

                            self.env.set(symbol, def.clone());

                            Ok(def)
                        }
                        "let" => {
                            // TODO validate
                            let bindings = match &list[1] {
                                Expr::Vector(vec) => vec,
                                _ => return Err("let bindings must be a vector".to_string()),
                            };

                            info!("bindings: {:?}", bindings);

                            if bindings.len() % 2 != 0 {
                                return Err("let bindings must be even".to_string());
                            }

                            info!("pushing scope");

                            self.push_scope();

                            let iter = BindingIterator::from(bindings.clone());

                            for (symbol, value) in iter {
                                info!("binding symbol {:?} to value {:?}", symbol, value);
                                self.env.set(&symbol, value);
                            }

                            // TODO validate
                            let body = list[2].clone();

                            let result = self.eval(body);

                            info!("popping scope");
                            self.pop_scope();

                            result
                        }
                        "fn" => {
                            // TODO validation on list size
                            let bindings_atoms: Result<Vec<Atom>, &'static str> =
                                extract_vec_from_expr(list[1].clone())?
                                    .iter()
                                    .map(|expr| unwrap_atom_from_expr(expr.clone()))
                                    .collect();

                            let bindings_strings: Result<Vec<String>, &'static str> =
                                bindings_atoms?
                                    .iter()
                                    .map(|atom| match atom {
                                        Atom::Identifier(id) => Ok(id.clone()),
                                        _ => Err("invalid identifier"),
                                    })
                                    .collect();

                            let body = Box::new(list[2].clone());

                            let lambda = Lambda::new(bindings_strings?, body);

                            Ok(Expr::from(lambda))
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
                            Atom::Lambda(l) => {
                                let Lambda { params, body } = l;

                                let args_eval: Result<Vec<Expr>, String> = args
                                    .iter()
                                    .map(|arg_expr| self.eval(arg_expr.clone()))
                                    .collect();

                                self.push_scope();

                                for (symbol, arg_eval) in params.iter().zip(args_eval?.iter()) {
                                    self.env.set(&symbol, arg_eval.clone());
                                }

                                let body_eval = self.eval(*body);

                                self.pop_scope();

                                body_eval
                            }
                            _ => Err("invalid form".to_string()),
                        },
                        _ => Err("invalid form".to_string()),
                    }
                }
            }
        }
    }

    fn push_scope(&mut self) {
        self.env = self.env.push_scope();
    }

    fn pop_scope(&mut self) {
        self.env = match self.env.pop_scope() {
            Some(scope) => scope,
            None => Env::default(),
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

fn unwrap_atom_from_expr(expr: Expr) -> Result<Atom, &'static str> {
    match expr {
        Expr::Atom(a) => Ok(a),
        _ => Err("expr is not atom"),
    }
}

fn extract_vec_from_expr(expr: Expr) -> Result<Vec<Expr>, &'static str> {
    match expr {
        Expr::Vector(vec) => Ok(vec),
        _ => Err("expr is not vector"),
    }
}

struct BindingIterator {
    vector: Vec<Expr>,
}

impl From<Vec<Expr>> for BindingIterator {
    fn from(exprs: Vec<Expr>) -> Self {
        BindingIterator {
            vector: exprs.clone(),
        }
    }
}

impl Iterator for BindingIterator {
    type Item = (String, Expr);

    fn next(&mut self) -> Option<Self::Item> {
        // TODO please add some kind of validation!!

        let old_vec = self.vector.clone();
        let mut take = old_vec.iter().take(2);
        let symbol = match take.next() {
            Some(expr) => match expr {
                Expr::Atom(a) => match a {
                    Atom::Identifier(id) => id,
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            },
            None => return None,
        };

        let expr = take.next().unwrap();

        self.vector = self.vector[2..].to_vec();

        Some((symbol.to_string(), expr.clone()))
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

        let mut int = Interpreter::new();
        let result = int.eval(expr);

        assert!(result.is_ok());
        assert_eq!(Expr::Atom(Atom::Integer(24)), result.unwrap());
    }

    #[test]
    fn do_evals_all_returns_last() {
        let parsed = DialParser::parse(Rule::list, "(do (+ 1 2) (+ 3 4) (* 5 6))").unwrap();
        let expr = Expr::from(parsed);

        let mut int = Interpreter::new();
        let result = int.eval(expr);

        assert!(result.is_ok());
        assert_eq!(Expr::Atom(Atom::Integer(30)), result.unwrap());
    }

    #[test]
    fn if_evals_true_expr_when_true() {
        let parsed = DialParser::parse(Rule::list, r#"(if true "true" "false")"#).unwrap();
        let expr = Expr::from(parsed);

        let mut int = Interpreter::new();
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

        let mut int = Interpreter::new();
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

        let mut int = Interpreter::new();
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

        let mut int = Interpreter::new();
        let result = int.eval(expr);

        assert!(result.is_ok());
        assert_eq!(
            Expr::Atom(Atom::String(String::from(r#""world""#))),
            result.unwrap()
        );
    }

    #[test]
    fn let_creates_temporary_binding() {
        let parsed = DialParser::parse(Rule::list, "(let [c 2] c)").unwrap();
        let expr = Expr::from(parsed);

        let mut int = Interpreter::new();
        let result = int.eval(expr);

        assert!(result.is_ok());
        assert_eq!(Expr::Atom(Atom::Integer(2)), result.unwrap());
    }

    #[test]
    fn let_scopes_value() {}

    #[test]
    fn let_overrides_value() {}
}
