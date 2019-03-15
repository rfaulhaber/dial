use super::parser::{DialParser, Rule};
use pest::error;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use std::iter::FromIterator;

#[derive(Debug)]
pub enum DialValue {
	Integer(i64),
	Float(f64),
	String(String),
	Nil,
	Err(String),
}

// TODO implement
// impl<'a> From<Pair<'a, Rule>> for DialType {
// }

// TODO return DialType
pub fn eval_line(input: &str) -> Result<DialValue, error::Error<Rule>> {
	eval(input, Rule::repl_line)
}

// TODO return DialType
fn eval(input: &str, rule: Rule) -> Result<DialValue, error::Error<Rule>> {
	let pair = DialParser::parse(rule, input)?.next().unwrap();

	eprintln!("pair: {:?}", pair);
	eprintln!("as rule: {:?}", pair.as_rule());

	let val = match pair.as_rule() {
		Rule::int => DialValue::Integer(pair.as_str().parse::<i64>().unwrap()),
		Rule::float => DialValue::Float(pair.as_str().parse::<f64>().unwrap()),
		Rule::expr => eval_expr(pair),
		Rule::COMMENT | Rule::nil => DialValue::Nil,
		_ => unimplemented!(),
	};

	Ok(val)
}

fn eval_expr(pair: Pair<Rule>) -> DialValue {
	let pair_str = pair.as_str();
	let mut inner = pair.into_inner();
	let first = inner.next().unwrap();

	match first.as_rule() {
		Rule::int => DialValue::Integer(pair_str.parse::<i64>().unwrap()),
		Rule::float => DialValue::Float(pair_str.parse::<f64>().unwrap()),
		Rule::add => {
			let mut vals = inner.map(eval_expr);
			let all_int = vals.clone().all(|value| match value {
				DialValue::Integer(_) => true,
				_ => false,
			});

			// TODO clean, break into more functions
			if all_int {
				let ret: i64 = vals.fold(0, |sum, val| match val {
					DialValue::Integer(int) => sum + int,
					_ => unreachable!(),
				});

				return DialValue::Integer(ret);
			} else {
				let ret: f64 = vals.fold(0.0, |sum, val| match val {
					DialValue::Integer(int) => sum + int as f64,
					DialValue::Float(float) => sum + float,
					_ => unreachable!(),
				});

				return DialValue::Float(ret);
			}
		}
		// Rule::expr => eval_expr(inner),
		_ => unimplemented!(),
	}
}
