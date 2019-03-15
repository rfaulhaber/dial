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

	let mut stack: Vec<Rule> = Vec::new();

	eprintln!("pair: {:?}", pair);
	eprintln!("as rule: {:?}", pair.as_rule());
	let val = match pair.as_rule() {
		Rule::add => eval_add(pair.into_inner()),
		Rule::subtract => eval_sub(pair.into_inner()),
		Rule::float => DialValue::Float(pair.as_str().parse().unwrap()),
		Rule::int => DialValue::Integer(pair.as_str().parse().unwrap()),
		Rule::COMMENT | Rule::nil => DialValue::Nil,
		_ => unimplemented!(),
	};

	Ok(val)
}

fn eval_expr(pair: Pair<Rule>) -> DialValue {
	match pair.as_rule() {
		Rule::add => eval_add(pair.into_inner()),
		_ => unimplemented!(),
	}
}

fn eval_add(pairs: Pairs<Rule>) -> DialValue {
	unimplemented!();
}

fn eval_sub(pairs: Pairs<Rule>) -> DialValue {
	unimplemented!();
}

fn eval_mult(pairs: Pairs<Rule>) -> DialValue {
	unimplemented!();
}

fn eval_div(pairs: Pairs<Rule>) -> DialValue {
	unimplemented!();
}
