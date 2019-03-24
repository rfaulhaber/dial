use super::parser::{DialParser, Rule};
use super::values::DialValue;
use log::Level;
use pest::error;
use pest::iterators::Pair;
use pest::Parser;

// TODO implement
// impl<'a> From<Pair<'a, Rule>> for DialType {
// }

pub fn eval_line(input: &str) -> Result<Vec<DialValue>, error::Error<Rule>> {
	eval(input, Rule::repl_line)
}

fn eval(input: &str, rule: Rule) -> Result<Vec<DialValue>, error::Error<Rule>> {
	let parsed_input = DialParser::parse(rule, input)?;

	let mut values = Vec::new();

	for pair in parsed_input {
		if log_enabled!(Level::Info) {
			info!("pair: {:?}", pair);
			info!("as rule: {:?}", pair.as_rule());
		}

		let val = match pair.as_rule() {
			Rule::int => DialValue::Integer(pair.as_str().parse::<i64>().unwrap()),
			Rule::float => DialValue::Float(pair.as_str().parse::<f64>().unwrap()),
			Rule::expr => eval_expr(pair),
			Rule::COMMENT | Rule::nil => DialValue::Nil,
			_ => unimplemented!(),
		};

		values.push(val);
	}

	Ok(values)
}

fn eval_expr(pair: Pair<Rule>) -> DialValue {
	let pair_str = pair.as_str();
	let mut inner = pair.into_inner();
	let first = inner.next().unwrap();

	match first.as_rule() {
		Rule::int => DialValue::Integer(pair_str.parse::<i64>().unwrap()),
		Rule::float => DialValue::Float(pair_str.parse::<f64>().unwrap()),
		Rule::add => inner.map(eval_expr).sum(),
		Rule::sub => inner
			.map(eval_expr)
			.fold(DialValue::Nil, |sum, val| sum - val),
		Rule::mul => inner
			.map(eval_expr)
			.fold(DialValue::Nil, |sum, val| sum * val),
		Rule::div => inner
			.map(eval_expr)
			.fold(DialValue::Nil, |sum, val| sum / val),
		Rule::nil => DialValue::Nil,

		// Rule::expr => eval_expr(inner),
		_ => unimplemented!(),
	}
}
