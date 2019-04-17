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

// TODO implement result, custom error
// type DialEvalResult = Result<DialValue, DialEvalError>;

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

        let values = parsed_input.map(|expr| self.eval_expr(expr)).collect();

        Ok(values)
    }

    fn get_symbol(&self, symbol: String) -> Option<DialValue> {
        self.env.get(&symbol)
    }

    fn eval_def_expr(&mut self, pair: Pair<Rule>) -> DialValue {
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

        expr_value.clone()
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

    fn eval_expr(&mut self, pair: Pair<Rule>) -> DialValue {
        info!("evaluating: {:?}", pair.as_rule());

        match pair.as_rule() {
            Rule::atom => self.eval_atom(pair),
            Rule::op_expr => self.eval_op_expr(pair),
            Rule::list_expr => self.eval_list_expr(pair),
            Rule::do_expr => self.eval_do_expr(pair),
            Rule::def_expr => self.eval_def_expr(pair),
            Rule::let_bind => self.eval_let_bind(pair),
            _ => unreachable!(),
        }
    }

    fn eval_atom(&self, pair: Pair<Rule>) -> DialValue {
        let original = pair.clone();
        let terminal = pair.into_inner().next().unwrap();
        let rule = terminal.as_rule();
        info!("eval_atom: evaluating: {:?}", rule);
        match rule {
            Rule::nil => DialValue::Nil,
            Rule::float => eval_float(terminal),
            Rule::int => eval_int(terminal),
            Rule::boolean => eval_bool(terminal),
            Rule::string => eval_string(terminal),
            Rule::symbol => self.eval_symbol(original),
            _ => unreachable!(),
        }
    }

    fn eval_op_expr(&mut self, pair: Pair<Rule>) -> DialValue {
        let mut inner = pair.into_inner();
        let first = inner.next().unwrap();

        info!("perfomring: {:?}", first.as_rule(),);

        match first.as_rule() {
            Rule::add => inner.map(|v| self.eval_expr(v)).sum(),
            Rule::sub => {
                let mut values = inner.map(|v| self.eval_expr(v));

                let first = values.next().unwrap();

                values.fold(first, |diff, val| diff - val)
            }
            Rule::mul => {
                let mut values = inner.map(|v| self.eval_expr(v));

                let first = values.next().unwrap();

                values.fold(first, |diff, val| diff * val)
            }
            Rule::div => {
                let mut values = inner.map(|v| self.eval_expr(v));

                let first = values.next().unwrap();

                values.fold(first, |quot, val| quot / val)
            }
            _ => unreachable!(),
        }
    }

    fn eval_list_expr(&self, pair: Pair<Rule>) -> DialValue {
        unimplemented!();
    }

    fn eval_do_expr(&mut self, pair: Pair<Rule>) -> DialValue {
        pair.into_inner()
            .map(|inner| self.eval_expr(inner))
            .last()
            .unwrap()
    }

    fn eval_if(&self, pair: Pair<Rule>) -> DialValue {
        unimplemented!();
    }

    fn eval_func(&self, pair: Pair<Rule>) -> DialValue {
        unimplemented!();
    }

    fn eval_symbol(&self, pair: Pair<Rule>) -> DialValue {
        let symbol = pair.as_span().as_str();
        match self.env.get(&String::from(symbol)) {
            Some(val) => val,
            None => DialValue::Nil, // TODO return error?
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

fn eval_int(pair: Pair<Rule>) -> DialValue {
    let parsed = pair.as_str().parse::<i64>().unwrap();
    DialValue::Integer(parsed)
}

fn eval_float(pair: Pair<Rule>) -> DialValue {
    let parsed = pair.as_str().parse::<f64>().unwrap();
    DialValue::Float(parsed)
}

fn eval_bool(pair: Pair<Rule>) -> DialValue {
    unimplemented!();
}

fn eval_string(pair: Pair<Rule>) -> DialValue {
    unimplemented!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn eval_add_expr() {
        let mut interp = Interpreter::new();

        let result = interp.eval("(+ 1 2)", Rule::expr);

        assert_eq!(DialValue::Integer(3), *result.unwrap().first().unwrap());
    }

    #[test]
    fn eval_add_indef_expr() {
        let mut interp = Interpreter::new();

        let result = interp.eval("(+ 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15)", Rule::expr);

        assert_eq!(DialValue::Integer(120), *result.unwrap().first().unwrap());
    }

    #[test]
    fn let_expr_basic() {
        let mut interp = Interpreter::new();

        let result = interp.eval("(let [c 2] c)", Rule::let_bind);

        assert_eq!(DialValue::Integer(2), *result.unwrap().first().unwrap());
    }

    #[test]
    fn def_expr_basic() {
        let mut interp = Interpreter::new();

        let assignment = interp.eval("(def a 2)", Rule::def_expr);
        assert_eq!(DialValue::Integer(2), *assignment.unwrap().first().unwrap());

        let result = interp.eval("(def b (+ a 2))", Rule::def_expr);
        assert_eq!(DialValue::Integer(4), *result.unwrap().first().unwrap());
    }

    #[test]
    fn do_expr_basic() {
        let mut interp = Interpreter::new();

        let assignment = interp.eval("(do (def a 2) (- 2 a) (+ a 6))", Rule::do_expr);
        assert_eq!(DialValue::Integer(8), *assignment.unwrap().first().unwrap());
    }
}
