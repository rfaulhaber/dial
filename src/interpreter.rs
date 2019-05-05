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
                Atom::Symbol(s) => unimplemented!(), // TODO do symbol lookup
                Atom::Identifier(id) => unimplemented!(), // TODO do id lookup,
                _ => Ok(DialValue::from(a)),
            },
            Sexpr::Cons(car, cdr) => {
                println!("car: {:?}", car);
                println!("cdr: {:?}", cdr);

                // TODO do symbol lookup on car, if function is valid, pass cdr to it

                let func = match *car {
                    Sexpr::Atom(a) => match a {
                        Atom::Symbol(s) => unimplemented!(), // TODO look for builtin function
                        Atom::Identifier(i) => unimplemented!(), // TODO look for user-defined function
                        _ => return Err("unknown symbol found"),
                    },
                    _ => return Err("invalid form found"),
                };

                // (func)(cdr)
            }
        }
    }

    fn get_symbol(&self, symbol: String) -> Option<DialValue> {
        self.env.borrow().get(&symbol)
    }

    // fn collect_expr(&self, sexpr: Sexpr) -> Vec<DialValue> {
    //     match sexpr {
    //         Sexpr::Cons(left, right) => {
    //             match *left {
    //                 Sexpr::Atom(a) => match (a) {
    //                     Atom::Symbol(_) | Atom::Identifier(_) => {
    //                         let left_eval = self.eval(*left);
    //                     }
    //                 },
    //             }
    //             let left_eval = self.collect_expr(*left);
    //             let right_eval = self.collect_expr(*right);

    //             left_eval.append(&mut right_eval);

    //             left_eval
    //         }
    //         Sexpr::Atom(a) => vec![DialValue::from(a)],
    //     }
    // }

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
