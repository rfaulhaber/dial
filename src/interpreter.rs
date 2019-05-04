// use super::core::{get_builtin, BuiltinFunc};
use super::env::Env;
use super::parser::{Atom, DialParser, Rule, Sexpr};
use super::values::DialValue;
use log::Level;
use std::cell::RefCell;
use std::error;

pub type EvalResult = Result<DialValue, &'static str>;
pub type BuiltinFunc = fn(args: Sexpr) -> EvalResult;

pub struct Interpreter {
    env: RefCell<Env>,
}

impl From<Atom> for DialValue {
    fn from(atom: Atom) -> Self {
        match atom {
            Atom::Integer(i) => DialValue::Integer(i),
            Atom::Float(f) => DialValue::Float(f),
            Atom::Boolean(b) => DialValue::Boolean(b),
            Atom::String(s) => DialValue::String(s),
            Atom::Symbol(s) => unimplemented!(), // TODO evaluate symbol
            Atom::Identifier(id) => unimplemented!(), // TODO evaluate identifier
            Atom::Nil => DialValue::Nil,
        }
    }
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            env: RefCell::new(Env::new()),
        }
    }

    pub fn eval(&self, expr: Sexpr) -> EvalResult {
        match expr {
            Sexpr::Atom(a) => match a {
                Atom::Symbol(s) => unimplemented!(),      // do symbol lookup
                Atom::Identifier(id) => unimplemented!(), // do id lookup,
                _ => Ok(DialValue::from(a)),
            },
            Sexpr::Cons(car, cdr) => {
                println!("car: {:?}", car);
                println!("cdr: {:?}", cdr);
                match *car {
                    Sexpr::Atom(a) => match a {
                        Atom::Symbol(s) => match s.as_str() {
                            "+" => {
                                let first = self.eval(cdr.car());
                                let rest = self.eval(cdr.cdr());

                                match (first, rest) {
                                    (Ok(first_result), Ok(rest_result)) => {
                                        Ok(first_result + rest_result)
                                    }
                                    (Err(err), _) => Err(err),
                                    (_, Err(err)) => Err(err),
                                }
                            }
                            _ => unimplemented!(),
                        },
                        Atom::Identifier(i) => unimplemented!(), // if id is function, make function call on cdr
                        // note: this isn't working out recursively. should collect cons values
                        _ => Err("malformed expression"),
                    },
                    _ => unimplemented!(),
                }
            }
        }
    }

    fn get_symbol(&self, symbol: String) -> Option<DialValue> {
        self.env.borrow().get(&symbol)
    }

    // fn call_function(&self, func_name: String, args: Sexpr) -> EvalResult {
    //     let builtin = get_builtin(func_name);

    //     match builtin {
    //         Some(func) => (func)(args),
    //         None => Err(format!("function {} not found", func_name).as_str()),
    //     }
    // }
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
