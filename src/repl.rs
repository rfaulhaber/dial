use super::parser::{DialParser, Rule};
use pest::ParseResult;
use std::io::Read;

pub struct Repl {
	parser: DialParser,
}

// impl Repl {
// 	pub fn read(&self, reader: &mut std::io::Read) -> Option<String> {
// 		unimplemented!();
// 	}

// 	pub fn eval(&self, )
// }
