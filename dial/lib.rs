pub mod builtin;
pub mod env;
pub mod parse;
#[macro_use]
pub mod sexpr;

use anyhow::Result;
use thiserror::Error;

pub use env::Env;
use parse::ParseResult;
use sexpr::DialVal;

pub type EvalResult = Result<DialVal, EvalError>;

// TODO "too many arguments" for macro
// TODO "too few arguments" for macro
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
    match val {
        DialVal::List(l) => {
            if l.is_empty() {
                Ok(DialVal::List(vec![]))
            } else {
                // TODO error handling
                let (first, rest) = l.split_at(1);

                // TODO error handling
                let first = first.get(0).unwrap();

                match first {
                    v if v == &DialVal::Sym("def".into()) => {
                        let sym = match rest.get(0) {
                            Some(val) => val,
                            None => return Err(EvalError::ArityError(0)), // TODO better error
                        };

                        let val = match rest.get(1) {
                            Some(val) => val,
                            None => return Err(EvalError::ArityError(1)),
                        }
                        .clone();

                        let val_res = eval(val, env)?;

                        return match sym {
                            DialVal::Sym(s) => {
                                env.set_value(s.clone(), val_res.clone());
                                Ok(val_res)
                            }
                            _ => {
                                return Err(EvalError::InvalidArgumentError(
                                    "'def' requires binding to symbol".into(),
                                ))
                            }
                        };
                    }
                    v if v == &DialVal::Sym("let".into()) => {
                        // TODO remove need for cloning
                        let mut scope = Env::with_scope(env.clone());

                        let (list_sl, inner) = rest.split_at(1);

                        // TODO error handling
                        // TODO stop with all the cloning
                        return match list_sl.get(0).unwrap().clone() {
                            DialVal::List(l) => {
                                for pair in l.into_iter().collect::<Vec<_>>().chunks(2) {
                                    let sym = pair.get(0).unwrap().clone();
                                    let val = pair.get(1).unwrap().clone();
                                    let val_res = eval(val, &mut scope);

                                    match sym {
                                        DialVal::Sym(s) => {
                                            scope.set_value(s, val_res?);
                                        }
                                        _ => {
                                            return Err(EvalError::TypeError(format!(
                                                "expected symbol in let binding, found {}",
                                                sym
                                            )))
                                        }
                                    }
                                }

                                inner
                                    .iter()
                                    // TODO stop cloning
                                    .map(|val| eval(val.clone(), &mut scope))
                                    .collect::<Result<Vec<DialVal>, EvalError>>()
                                    .map(|v| v.get(0).unwrap().clone())
                            }
                            _ => Err(EvalError::InvalidArgumentError(format!(
                                "let binding expects a list of associations"
                            ))),
                        };
                    }
                    v if v == &DialVal::Sym("if".into()) => {
                        let mut rest = Vec::from(rest);
                        rest.reverse();

                        let cond = match rest.pop() {
                            Some(v) => v,
                            None => return Err(EvalError::ArityError(1)),
                        };

                        let cond_result = eval(cond, env)?;

                        let if_true = match rest.pop() {
                            Some(e) => e,
                            None => return Err(EvalError::ArityError(2)),
                        };

                        // TODO assert this is last item
                        let if_false = match rest.pop() {
                            Some(e) => e,
                            None => return Err(EvalError::ArityError(3)),
                        };

                        if rest.len() > 0 {
                            return Err(EvalError::ArityError(4));
                        }

                        match cond_result {
                            DialVal::Nil | DialVal::Bool(false) => eval(if_false, env),
                            _ => eval(if_true, env),
                        }
                    }
                    v if v == &DialVal::Sym("fn".into()) => {
                        let fn_args = match rest.get(0) {
                            Some(args) => match args {
                                DialVal::List(l) => {
                                    let mut args_sym = vec![];

                                    for arg in l {
                                        match arg {
                                            DialVal::Sym(s) => args_sym.push(s.clone()),
                                            _ => return Err(EvalError::TypeError("symbol".into())),
                                        }
                                    }

                                    args_sym
                                }
                                _ => return Err(EvalError::TypeError("list".into())),
                            },
                            None => return Err(EvalError::ArityError(3)),
                        };

                        let fn_body = match rest.get(1) {
                            Some(body) => body.clone(),
                            None => return Err(EvalError::ArityError(3)),
                        };

                        Ok(DialVal::Lambda {
                            params: fn_args,
                            body: Box::new(fn_body),
                        })
                    }
                    v if v == &DialVal::Sym("do".into()) => {
                        let rest: Result<Vec<DialVal>, EvalError> =
                            rest.iter().map(|val| eval(val.clone(), env)).collect();

                        match rest {
                            Ok(res) => Ok(res.last().unwrap().clone()),
                            Err(e) => Err(e),
                        }
                    }
                    _ => {
                        let rest: Result<Vec<DialVal>, EvalError> =
                            rest.iter().map(|val| eval(val.clone(), env)).collect();

                        match eval(first.clone(), env) {
                            Ok(DialVal::Builtin { func, .. }) => func(rest?.as_slice(), env),
                            Ok(DialVal::Lambda { params, body }) => {
                                let mut lambda_env = Env::with_scope(env.clone());

                                let args = rest?;

                                if params.len() != args.len() {
                                    return Err(EvalError::ArityError(params.len()));
                                }

                                lambda_env.bind(params, args);

                                eval(*body, &mut lambda_env)
                            }
                            _ => Err(EvalError::TypeError(format!("{} is not a function", first))),
                        }
                    }
                }
            }
        }
        _ => eval_form(val, env),
    }
}

fn eval_form(val: DialVal, env: &mut Env) -> EvalResult {
    match val {
        DialVal::Sym(s) => env
            .get_value(s.clone())
            .clone()
            .ok_or_else(|| EvalError::Undefined(format!("no such symbol {}", s).into())),
        DialVal::List(l) => {
            let vals: Result<Vec<DialVal>, EvalError> =
                l.iter().map(|v| eval(v.clone(), env)).collect();

            match vals {
                Ok(vs) => Ok(DialVal::List(vs)),
                Err(e) => Err(e),
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
        // DialVal::Hash(h) => todo!(),
        _ => Ok(val),
    }
}

pub fn print(val: EvalResult) -> String {
    todo!();
}

#[cfg(test)]
mod mal_tests {
    use super::*;
    use pretty_assertions::assert_eq;

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
                Ok(DialVal::Int(1)),
                Ok(DialVal::Float(6.0)),
                Ok(DialVal::Float(0.0)),
                Ok(DialVal::Float(0.125)),
                Ok(DialVal::Float(1.0 / 6.0)),
                Ok(DialVal::Float(5.0)),
                Ok(DialVal::Float(14.0)),
            ]
        )
    }

    #[test]
    fn step_3_def() {
        let mut env = Env::default();

        let def_input = "(def foo 123)";

        let input_parse = read(def_input.into()).unwrap().pop().unwrap();
        let def_result = eval(input_parse, &mut env);

        assert_eq!(def_result, Ok(DialVal::Int(123)));
    }

    #[test]
    fn step_3_provided_tests() {
        let inputs = vec![
            "(def a 6)",
            "a",
            "(def b (+ a 2))",
            "(+ a b)",
            "(let (c 2) c)",
            "c",
        ];

        let mut env = Env::default();

        let results: Vec<EvalResult> = inputs
            .iter()
            .map(|input| eval(read(input.to_string()).unwrap().pop().unwrap(), &mut env))
            .collect();

        assert_eq!(
            results,
            vec![
                Ok(6.into()),
                Ok(6.into()),
                Ok(8.0.into()),
                Ok(14.0.into()),
                Ok(2.into()),
                Err(EvalError::Undefined("no such symbol c".into()))
            ]
        )
    }

    #[test]
    fn step_4_if() {
        let inputs = vec![
            r#"(if true 1 2)"#,
            r#"(if false 1 2)"#,
            "(if)",
            "(if true foo bar baz)",
        ];

        let mut env = Env::default();

        let results: Vec<EvalResult> = inputs
            .iter()
            .map(|input| eval(read(input.to_string()).unwrap().pop().unwrap(), &mut env))
            .collect();

        assert_eq!(
            results,
            vec![
                Ok(1.into()),
                Ok(2.into()),
                Err(EvalError::ArityError(1)),
                Err(EvalError::ArityError(4)),
            ]
        );
    }

    #[test]
    fn step_4_do() {
        let inputs = vec![
            "(do 1 2 3 4)",
            "(do (+ 1 2) (+ 3 4))",
            "(do (def foo 123) (+ foo 123))",
        ];

        let mut env = Env::default();

        let results: Vec<EvalResult> = inputs
            .iter()
            .map(|input| eval(read(input.to_string()).unwrap().pop().unwrap(), &mut env))
            .collect();

        assert_eq!(
            results,
            vec![Ok(4.into()), Ok(7.0.into()), Ok(246.0.into())]
        );
    }

    #[test]
    fn step_4_fn() {
        let inputs = vec![
            "((fn (a) a) 7)",
            "((fn (a) (+ a 1)) 10)",
            "((fn (a b) (+ a b)) 2 3)",
        ];

        let mut env = Env::default();

        let results: Vec<EvalResult> = inputs
            .iter()
            .map(|input| eval(read(input.to_string()).unwrap().pop().unwrap(), &mut env))
            .collect();

        assert_eq!(results, vec![Ok(7.into()), Ok(11.0.into()), Ok(5.0.into())]);
    }

    #[test]
    fn step_4_list_fn() {
        let inputs = vec!["(list 1 2 3)"];

        let mut env = Env::default();

        let results: Vec<EvalResult> = inputs
            .iter()
            .map(|input| eval(read(input.to_string()).unwrap().pop().unwrap(), &mut env))
            .collect();

        assert_eq!(
            results,
            vec![Ok(DialVal::List(vec![1.into(), 2.into(), 3.into()]))]
        );
    }

    #[test]
    fn step_4_is_list_fn() {
        let inputs = vec!["(list? (list 1 2 3))", "(list? 1)"];

        let mut env = Env::default();

        let results: Vec<EvalResult> = inputs
            .iter()
            .map(|input| eval(read(input.to_string()).unwrap().pop().unwrap(), &mut env))
            .collect();

        assert_eq!(results, vec![Ok(true.into()), Ok(false.into())]);
    }

    #[test]
    fn step_4_is_empty_fn() {
        let inputs = vec!["(empty? (list))", "(empty? (list 1 2 3))"];

        let mut env = Env::default();

        let results: Vec<EvalResult> = inputs
            .iter()
            .map(|input| eval(read(input.to_string()).unwrap().pop().unwrap(), &mut env))
            .collect();

        assert_eq!(results, vec![Ok(true.into()), Ok(false.into())]);
    }

    #[test]
    fn step_4_count_fn() {
        let inputs = vec!["(count (list 1 2 3))", "(count (list))"];

        let mut env = Env::default();

        let results: Vec<EvalResult> = inputs
            .iter()
            .map(|input| eval(read(input.to_string()).unwrap().pop().unwrap(), &mut env))
            .collect();

        assert_eq!(results, vec![Ok(3.into()), Ok(0.into())]);
    }
}
