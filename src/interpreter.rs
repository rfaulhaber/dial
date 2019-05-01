use super::core::{add, mul};
use super::env::Env;
use super::parser::{DialParser, Rule, Sexpr};
use super::values::DialValue;
use log::Level;
use std::cell::RefCell;
use std::collections::HashMap;
use std::error;

pub type EvalResult = Result<DialValue, &'static str>;
pub type BuiltinFunc = fn(args: Vec<DialValue>) -> EvalResult;

pub struct Interpreter {
    env: RefCell<Env>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            env: RefCell::new(Env::new()),
        }
    }

    pub fn eval(&self, expr: Sexpr) -> EvalResult {
        let mut stack = Vec::new();
        let mut tmp_stack = Vec::new();
        let mut result_stack = Vec::new();

        for sexpr in expr.into_iter() {
            println!("sexpr: {:?}", sexpr);
            match sexpr {
                Sexpr::Nil => {
                    println!("nil found, unwinding stack");
                    for sexpr in stack.pop() {
                        println!("element in stack: {:?}", sexpr);
                        match sexpr {
                            Sexpr::Symbol(s) => {
                                match (get_builtin(s))(tmp_stack.clone()) {
                                    Ok(val) => result_stack.push(val),
                                    Err(e) => {
                                        return Err(e);
                                    }
                                }

                                tmp_stack.clear();

                                // pass op, vec into handler function
                                break;
                            }
                            Sexpr::Integer(int) => tmp_stack.push(DialValue::Integer(int)),
                            Sexpr::Float(fl) => tmp_stack.push(DialValue::Float(fl)),
                            Sexpr::String(s) => tmp_stack.push(DialValue::String(s)),
                            Sexpr::Boolean(b) => tmp_stack.push(DialValue::Boolean(b)),
                            Sexpr::Identifier(id) => {
                                let result = self.get_symbol(id);

                                match result {
                                    Some(result) => tmp_stack.push(result),
                                    None => {
                                        return Err("variable not defined");
                                    }
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                }

                _ => stack.push(sexpr),
            }
        }

        Ok(result_stack[0].clone())
    }

    fn get_symbol(&self, symbol: String) -> Option<DialValue> {
        self.env.borrow().get(&symbol)
    }
}

fn sexpr_to_value(expr: Sexpr) -> DialValue {
    match expr {
        Sexpr::Integer(int) => DialValue::Integer(int),
        Sexpr::Float(fl) => DialValue::Float(fl),
        Sexpr::Boolean(b) => DialValue::Boolean(b),
        Sexpr::String(s) => DialValue::String(s),
        _ => unreachable!(),
    }
}

fn get_builtin(symbol: String) -> BuiltinFunc {
    match symbol.as_str() {
        "+" => add,
        "*" => mul,
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod interpreter_test {
    use super::*;
    use pest::Parser;

    #[test]
    fn test_function_call() {
        let mut parsed = DialParser::parse(Rule::list, "(* 2 (+ 3 4 5))").unwrap();
        let ast = Sexpr::from_pair(parsed.next().unwrap());

        let int = Interpreter::new();
        let result = int.eval(ast);

        assert!(result.is_ok());
        assert_eq!(DialValue::Integer(24), result.unwrap());
    }
}
