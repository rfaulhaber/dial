use super::parser::{DialParser, Rule};
use pest::error;
use pest::iterators::Pair;
use pest::Parser;

pub enum DialValue {
	Integer(u64),
	Float(f64),
	String(String),
	Nil,
	Err(String),
}

// TODO implement
// impl<'a> From<Pair<'a, Rule>> for DialType {
// }

// TODO return DialType
pub fn eval_line(input: &str) {
	eval(input, Rule::repl_line).unwrap();
}

// TODO return DialType
fn eval(input: &str, rule: Rule) -> Result<DialValue, error::Error<Rule>> {
	let parse_result = DialParser::parse(rule, input)?;

	let stack: Vec<DialValue> = Vec::new();

	for pair in parse_result {
		println!("pair: {:?}", pair);
		println!("as rule: {:?}", pair.as_rule());
	}

	unimplemented!();
}

// fn parse_value(pair: Pair<Rule>) -> DialValue {

// 	// match pair.as_rule() {
// 	// 	Rule::expr => {}
// 	// 	Rule::number => {}
// 	// 	_ => unreachable!(),
// 	// }
// }
