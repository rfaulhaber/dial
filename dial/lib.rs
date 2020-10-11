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
        DialVal::Sym(s) => env
            .get_value(s.clone())
            .clone()
            .ok_or_else(|| EvalError::Undefined(format!("no such symbol {}", s).into())),
        DialVal::List(l) => {
            if l.is_empty() {
                Ok(DialVal::List(vec![]))
            } else {
                // TODO error handling
                let (first, rest) = l.split_at(1);

                // TODO error handling
                let first = first.get(0).unwrap();

                // TODO eliminate need for special rules
                if first == &DialVal::Sym("def".into()) {
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
                } else if first == &DialVal::Sym("let".into()) {
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

                let rest: Result<Vec<DialVal>, EvalError> =
                    rest.iter().map(|val| eval(val.clone(), env)).collect();

                match eval(first.clone(), env) {
                    Ok(DialVal::Builtin { func, .. }) => func(rest?.as_slice(), env),
                    _ => Err(EvalError::TypeError(format!("{} is not a function", first))),
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
}
