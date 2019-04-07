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

	pub fn eval_repl(&self, input: &str) -> Result<Vec<DialValue>, error::Error<Rule>> {
		self.eval(input, Rule::repl_line)
	}

	pub fn eval(&self, input: &str, rule: Rule) -> Result<Vec<DialValue>, error::Error<Rule>> {
		let parsed_input = DialParser::parse(rule, input)?;

		let mut values = Vec::new();

		for pair in parsed_input {
			if log_enabled!(Level::Info) {
				info!("found rule: {:?}", pair.as_rule());
			}

			let val = match pair.as_rule() {
				Rule::int => DialValue::Integer(pair.as_str().parse::<i64>().unwrap()),
				Rule::float => DialValue::Float(pair.as_str().parse::<f64>().unwrap()),
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

	fn eval_let_bind(&self, pair: Pair<Rule>) -> DialValue {
		unimplemented!();
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
}
