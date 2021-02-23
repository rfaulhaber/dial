use super::*;

macro_rules! new_scope {
    ($env:ident, $count:ident) => {
        $env.new_scope();
        $count = $count + 1;
    };
}

pub fn eval(val: DialVal, env: &mut Env) -> EvalResult {
    let mut vals = vec![val];
    let mut scopes_to_drop = 0;

    let ret = 'eval: loop {
        let val = match vals.pop() {
            Some(v) => v,
            None => break Ok(DialVal::Nil),
        };

        match val {
            DialVal::List(l) => {
                if l.is_empty() {
                    break Ok(DialVal::List(vec![]));
                } else {
                    // TODO error handling
                    let (first, rest) = l.split_at(1);

                    // TODO error handling
                    let first = first.get(0).unwrap();

                    match first {
                        v if v == &DialVal::Sym("def".into()) => {
                            let sym = match rest.get(0) {
                                Some(val) => val,
                                None => break Err(EvalError::ArityError(0)), // TODO better error
                            };

                            let val = match rest.get(1) {
                                Some(val) => val,
                                None => break Err(EvalError::ArityError(1)),
                            }
                            .clone();

                            let val_res = eval(val, env)?;

                            match sym {
                                DialVal::Sym(s) => {
                                    env.def_value(s.clone(), val_res.clone());
                                    break Ok(val_res);
                                }
                                _ => {
                                    break Err(EvalError::InvalidArgumentError(
                                        "'def' requires binding to symbol".into(),
                                    ))
                                }
                            }
                        }
                        v if v == &DialVal::Sym("let".into()) => {
                            let (list_sl, inner) = rest.split_at(1);
                            new_scope!(env, scopes_to_drop);

                            // TODO error handling
                            match list_sl.get(0).unwrap().clone() {
                                DialVal::List(l) => {
                                    for pair in l.into_iter().collect::<Vec<_>>().chunks(2) {
                                        let sym = pair.get(0).unwrap().clone();
                                        let val = pair.get(1).unwrap().clone();
                                        let val_res = eval(val, env);

                                        match sym {
                                            DialVal::Sym(s) => {
                                                env.set_value(s, val_res?);
                                            }
                                            _ => {
                                                break 'eval Err(EvalError::TypeError(format!(
                                                    "expected symbol in let binding, found {}",
                                                    sym
                                                )))
                                            }
                                        }
                                    }

                                    vals.append(&mut Vec::from(inner));
                                    continue 'eval;
                                }
                                _ => {
                                    break 'eval Err(EvalError::InvalidArgumentError(format!(
                                        "let binding expects a list of associations"
                                    )))
                                }
                            };
                        }
                        v if v == &DialVal::Sym("if".into()) => {
                            let mut rest = Vec::from(rest);
                            rest.reverse();

                            let cond = match rest.pop() {
                                Some(v) => v,
                                None => break 'eval Err(EvalError::ArityError(1)),
                            };

                            let cond_result = eval(cond, env)?;

                            let if_true = match rest.pop() {
                                Some(e) => e,
                                None => break 'eval Err(EvalError::ArityError(2)),
                            };

                            // TODO assert this is last item
                            let if_false = match rest.pop() {
                                Some(e) => e,
                                None => break 'eval Err(EvalError::ArityError(3)),
                            };

                            if rest.len() > 0 {
                                break 'eval Err(EvalError::ArityError(4));
                            }

                            match cond_result {
                                DialVal::Nil | DialVal::Bool(false) => {
                                    vals.push(if_false);
                                }
                                _ => vals.push(if_true),
                            };

                            continue 'eval;
                        }
                        v if v == &DialVal::Sym("fn".into()) => {
                            let fn_args = match rest.get(0) {
                                Some(args) => match args {
                                    DialVal::List(l) => {
                                        let mut args_sym = vec![];

                                        for arg in l {
                                            match arg {
                                                DialVal::Sym(s) => args_sym.push(s.clone()),
                                                _ => {
                                                    break 'eval Err(EvalError::TypeError(
                                                        "symbol".into(),
                                                    ))
                                                }
                                            }
                                        }

                                        args_sym
                                    }
                                    _ => break 'eval Err(EvalError::TypeError("list".into())),
                                },
                                None => break 'eval Err(EvalError::ArityError(3)),
                            };

                            let fn_body = match rest.get(1) {
                                Some(body) => body.clone(),
                                None => break 'eval Err(EvalError::ArityError(3)),
                            };

                            break 'eval Ok(DialVal::Lambda {
                                params: fn_args,
                                body: Box::new(fn_body),
                                env: env.clone(),
                            });
                        }
                        v if v == &DialVal::Sym("do".into()) => {
                            if rest.is_empty() {
                                break 'eval Ok(DialVal::Nil);
                            }

                            let (head, tail) = rest.split_at(rest.len() - 1);
                            let rest: Result<Vec<DialVal>, EvalError> =
                                head.iter().map(|val| eval(val.clone(), env)).collect();

                            match rest {
                                Ok(_) => {
                                    vals.push(tail.first().unwrap().clone());
                                }
                                Err(e) => break 'eval Err(e),
                            };
                        }
                        _ => {
                            let rest: Result<Vec<DialVal>, EvalError> =
                                rest.iter().map(|val| eval(val.clone(), env)).collect();

                            break 'eval match eval(first.clone(), env) {
                                Ok(DialVal::Builtin { func, .. }) => func(rest?.as_slice(), env),
                                Ok(DialVal::Lambda { params, body, .. }) => {
                                    let args = rest?;
                                    new_scope!(env, scopes_to_drop);

                                    if params.len() != args.len() {
                                        break 'eval Err(EvalError::ArityError(params.len()));
                                    }

                                    env.bind(params, args);

                                    vals.push(*body);

                                    continue 'eval;
                                }
                                _ => Err(EvalError::TypeError(format!(
                                    "{} is not a function",
                                    first
                                ))),
                            };
                        }
                    }
                }
            }
            _ => break 'eval eval_form(val, env),
        }
    };

    env.drop_scopes(scopes_to_drop);

    ret
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
