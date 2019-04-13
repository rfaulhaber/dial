use super::env::Env;
use super::parser::{DialParser, Rule};
use super::values::DialValue;
use log::Level;
use pest::error;
use pest::iterators::Pair;
use pest::Parser;

// TODO implement
// impl<'a> From<Pair<'a, Rule>> for DialType {
// }

pub struct Interpreter {
    env: Env,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter { env: Env::new() }
    }

    pub fn eval_repl(&mut self, input: &str) -> Result<Vec<DialValue>, error::Error<Rule>> {
        self.eval(input, Rule::repl_line)
    }

    pub fn eval(&mut self, input: &str, rule: Rule) -> Result<Vec<DialValue>, error::Error<Rule>> {
        let parsed_input = DialParser::parse(rule, input)?;

        let mut values = Vec::new();

        for pair in parsed_input {
            if log_enabled!(Level::Info) {
                info!("found rule: {:?}", pair.as_rule());
            }

            let val = match pair.as_rule() {
                Rule::int => DialValue::Integer(pair.as_str().parse::<i64>().unwrap()),
                Rule::float => DialValue::Float(pair.as_str().parse::<f64>().unwrap()),
                Rule::string => DialValue::String(String::from(pair.as_str())),
                Rule::boolean => {
                    if pair.as_str() == "true" {
                        DialValue::Boolean(true)
                    } else {
                        DialValue::Boolean(false)
                    }
                }
                Rule::expr => self.eval_expr(pair),
                Rule::def_expr => self.eval_def_expr(pair),
                Rule::let_bind => self.eval_let_bind(pair),
                Rule::symbol => match self.get_symbol(String::from(pair.as_span().as_str())) {
                    Some(val) => val,
                    None => DialValue::Nil, // TODO return error?
                },
                Rule::COMMENT | Rule::nil => DialValue::Nil,
                _ => {
                    info!("rule not implemented yet");
                    DialValue::Nil
                }
            };

            values.push(val);
        }

        Ok(values)
    }

    fn get_symbol(&self, symbol: String) -> Option<DialValue> {
        self.env.get(&symbol)
    }

    fn eval_def_expr(&self, pair: Pair<Rule>) -> DialValue {
        let mut inner = pair.into_inner();
        let symbol = inner.next().unwrap().as_str();

        if log_enabled!(Level::Info) {
            info!("defining symbol: {}", symbol);
        }

        let expr_value = self.eval_expr(inner.next().unwrap()); // this could be better

        self.env.set(&String::from(symbol), expr_value.clone());

        if log_enabled!(Level::Info) {
            info!("symbol defined as: {:?}", expr_value.clone());
        }

        expr_value
    }

    fn eval_let_bind(&mut self, pair: Pair<Rule>) -> DialValue {
        self.push_scope();

        // TODO dedupe code
        let mut inner = pair.into_inner();
        let symbol = inner.next().unwrap().as_str();

        if log_enabled!(Level::Info) {
            info!("defining symbol in let scope: {}", symbol);
        }

        let expr_value = self.eval_expr(inner.next().unwrap()); // this could be better

        self.env.set(&String::from(symbol), expr_value.clone());

        if log_enabled!(Level::Info) {
            info!("symbol defined as: {:?}", expr_value.clone());
        }

        let result = self.eval_expr(inner.next().unwrap()); // TODO handle errors

        self.pop_scope();

        result
    }

    fn eval_expr(&self, pair: Pair<Rule>) -> DialValue {
        let pair_str = pair.as_str();
        let mut inner = pair.into_inner();
        let first = inner.next().unwrap();

        match first.as_rule() {
            Rule::int => DialValue::Integer(pair_str.parse::<i64>().unwrap()),
            Rule::float => DialValue::Float(pair_str.parse::<f64>().unwrap()),
            Rule::add => inner.map(|v| self.eval_expr(v)).sum(),
            Rule::sub => inner
                .map(|v| self.eval_expr(v))
                .fold(DialValue::Nil, |sum, val| sum - val),
            Rule::mul => inner
                .map(|v| self.eval_expr(v))
                .fold(DialValue::Nil, |sum, val| sum * val),
            Rule::div => inner
                .map(|v| self.eval_expr(v))
                .fold(DialValue::Nil, |sum, val| sum / val),
            Rule::nil => DialValue::Nil,

            // Rule::expr => eval_expr(inner),
            Rule::symbol => match self.get_symbol(String::from(pair_str)) {
                Some(val) => val,
                None => DialValue::Nil, // TODO return error?
            },
            _ => unimplemented!(),
        }
    }

    fn push_scope(&mut self) {
        self.env = self.env.push_scope();
    }

    fn pop_scope(&mut self) {
        self.env = match self.env.pop_scope() {
            Some(scope) => scope,
            None => Env::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_eval_add_expr() {
        let mut interp = Interpreter::new();

        let result = interp.eval("(+ 1 2)", Rule::expr);

        assert_eq!(DialValue::Integer(3), *result.unwrap().first().unwrap());
    }

    #[test]
    fn test_eval_add_indef_expr() {
        let mut interp = Interpreter::new();

        let result = interp.eval("(+ 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15)", Rule::expr);

        assert_eq!(DialValue::Integer(120), *result.unwrap().first().unwrap());
    }

    #[test]
    fn test_let_expr_basic() {
        let mut interp = Interpreter::new();

        let result = interp.eval("(let [c 2] c)", Rule::let_bind);

        assert_eq!(DialValue::Integer(2), *result.unwrap().first().unwrap());
    }

    #[test]
    fn test_def_expr_basic() {
        let mut interp = Interpreter::new();

        let assignment = interp.eval("(def a 2)", Rule::def_expr);
        assert_eq!(DialValue::Integer(2), *assignment.unwrap().first().unwrap());

        let result = interp.eval("(def b (+ a 2))", Rule::def_expr);
        assert_eq!(DialValue::Integer(4), *result.unwrap().first().unwrap());
    }
}
