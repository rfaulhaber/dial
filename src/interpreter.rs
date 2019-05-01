use super::env::Env;
use super::parser::{DialParser, Rule, Sexpr};
use super::values::DialValue;
use log::Level;
use std::cell::RefCell;
use std::error;

pub type EvalResult = Result<DialValue, &'static str>;

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
        let mut iter = expr.into_iter();

        let mut stack = Vec::new();

        for sexpr in iter.next() {}
    }

    fn get_symbol(&self, symbol: String) -> Option<DialValue> {
        self.env.borrow().get(&symbol)
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
