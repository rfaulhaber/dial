use super::parser::{DialParser, Rule};
use pest::Parser;

pub enum EvalResult {
	Integer(u64),
	Float(f64),
	Str(String),
	Nil,
	Err(String),
}

pub fn eval_statement(input: &str) {
	eval(input, Rule::statement)
}

fn eval(input: &str, rule: Rule) {
	let parse_result = match DialParser::parse(Rule::repl_line, input) {
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
			println!("rule: {:?}", rule);
			match rule {
				Rule::add => {
					println!("add");
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
