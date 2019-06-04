// use super::core::{get_builtin, BuiltinFunc};
use super::parser::{Atom, Expr, Lambda};
use crate::environment::env::Env;

// TODO remove all direct vector references, safely use .get() instead
// TODO refactor using this: https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#destructuring-nested-structs-and-enums

pub type EvalResult = Result<Expr, String>;

pub struct Interpreter {
    env: Env,
    close_env: bool,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            env: Env::default(),
            close_env: false,
        }
    }

    // TODO please god make this method shorter and fix this right drift
    pub fn eval(&mut self, expr: Expr) -> EvalResult {
        let mut inner_expr = expr;
        let result = loop {
            match inner_expr {
                Expr::Atom(atom) => match atom {
                    Atom::Symbol(symbol) => {
                        let lookup = self.env.get(&symbol);

                        break match lookup {
                            Some(result) => Ok(result),
                            None => Err(format!("cannot resolve symbol: {}", symbol)),
                        };
                    }
                    Atom::Identifier(id) => {
                        let lookup = self.env.get(&id);

                        break match lookup {
                            Some(result) => Ok(result),
                            None => Err(format!("cannot resolve symbol: {}", id)),
                        };
                    }
                    _ => break Ok(atom.into()),
                },
                Expr::Vector(vec) => {
                    if vec.is_empty() {
                        return Ok(Expr::Vector(Vec::new()));
                    }

                    let vec_eval: Result<Vec<Expr>, String> =
                        vec.iter().map(|expr| self.eval(expr.clone())).collect();

                    break match vec_eval {
                        Ok(exprs) => Ok(Expr::Vector(exprs)),
                        Err(err) => Err(err),
                    };
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
                                    inner_expr = match list.get(3) {
                                        Some(expr) => expr.clone(),
                                        None => Atom::Nil.into(),
                                    }
                                } else {
                                    inner_expr = list[2].clone();
                                }
                            }
                            "do" => {
                                let args = &list[1..list.len() - 1];

                                let previous: Result<Vec<Expr>, String> =
                                    args.iter().map(|expr| self.eval(expr.clone())).collect();

                                match previous {
                                    Err(message) => break Err(message),
                                    _ => {
                                        inner_expr = list[list.len() - 1].clone();
                                        continue;
                                    }
                                }
                            }
                            "def" => {
                                let first_atom = &list[1].as_atom();
                                let symbol =
                                    match first_atom {
                                        Some(atom) => match atom {
                                            Atom::Identifier(id) => id,
                                            _ => return Err(
                                                "first argument of def must be valid identifier"
                                                    .to_string(),
                                            ),
                                        },
                                        None => {
                                            return Err(
                                                "first argument of def must be valid identifier"
                                                    .to_string(),
                                            )
                                        }
                                    };

                                let def = self.eval(list[2].clone())?;

                                self.env.set(symbol, def.clone());

                                break Ok(def);
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

                                bindings.clone().chunks(2).for_each(|chunk| {
                                    let symbol_expr = chunk[0].clone();
                                    let value = chunk[1].clone();
                                    self.env
                                        .set(&extract_identifier_from_expr(symbol_expr), value)
                                });

                                // TODO validate
                                inner_expr = list[2].clone();
                                self.close_env = true;
                                continue;
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

                                break Ok(Expr::from(lambda));
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

                                    break f(&args_eval?);
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

                                    break body_eval;
                                }
                                _ => break Err("invalid form".to_string()),
                            },
                            _ => break Err("invalid form".to_string()),
                        }
                    }
                }
            }
        };

        if self.close_env {
            self.close_env = false;
            self.pop_scope();
        }

        result
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

fn extract_identifier_from_expr(expr: Expr) -> String {
    match expr {
        Expr::Atom(a) => match a {
            Atom::Identifier(id) => id,
            _ => unreachable!(),
        },
        _ => unreachable!(),
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
    use crate::parser::{DialParser, Rule};
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
    fn let_scopes_value() {
        let parsed = DialParser::parse(Rule::list, "(do (let [c 2] c) (+ c 2))").unwrap();
        let expr = Expr::from(parsed);

        let mut int = Interpreter::new();
        let result = int.eval(expr);

        assert!(result.is_err());
        assert_eq!(Err("cannot resolve symbol: c".to_string()), result);
    }

    #[test]
    fn let_overrides_value() {
        let parsed = DialParser::parse(Rule::list, "(do (def c 2) (let [c 3] c))").unwrap();
        let expr = Expr::from(parsed);

        let mut int = Interpreter::new();
        let result = int.eval(expr);

        assert!(result.is_ok());
        assert_eq!(Ok(3.into()), result);
    }

    #[test]
    fn fn_creates_closure() {
        let parsed = DialParser::parse(Rule::list, "(fn [a b] (+ a b))").unwrap();
        let expr = Expr::from(parsed);

        let mut int = Interpreter::new();
        let result = int.eval(expr);

        assert!(result.is_ok());
    }

    #[test]
    fn fn_creates_callable_closure() {
        let parsed = DialParser::parse(Rule::list, "((fn [a b] (+ a b)) 2 3)").unwrap();
        let expr = Expr::from(parsed);

        let mut int = Interpreter::new();
        let result = int.eval(expr);

        assert!(result.is_ok());
        assert_eq!(Ok(5.into()), result);
    }
}
