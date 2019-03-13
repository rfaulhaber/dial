use super::parser::{DialParser, Rule};
use pest::iterators::Pair;
use pest::Parser;

struct Ast;

pub enum DialType {
	Integer(u64),
	Float(f64),
	String(String),
	Nil,
	Err(String),
}

// TODO implement
// impl<'a> From<Pair<'a, Rule>> for DialType {
// }

struct Stack;

// TODO return DialType
pub fn eval_line(input: &str) {
	eval(input, Rule::repl_line)
}

// TODO return DialType
fn eval(input: &str, rule: Rule) {
	let parse_result = match DialParser::parse(rule, input) {
		Ok(result) => result,
		Err(err) => {
			// TODO clean up
			println!("unexpected error: {:?}", err);
			return;
		}
	};

	for pair in parse_result {
		for inner_pair in pair.into_inner() {
			let rule = inner_pair.as_rule();
			println!("inner pair: {:?}", inner_pair);
			match rule {
				Rule::add => {
					println!("add: {:?}", inner_pair);
				}
				Rule::subtract => {
					println!("subtract");
				}
				Rule::multiply => {
					println!("multiply");
				}
				Rule::divide => {
					println!("divide");
				}
				Rule::power => {
					println!("power");
				}
				Rule::expr => {
					println!("expr");
				}
				_ => unreachable!(),
			}
		}
	}
}
