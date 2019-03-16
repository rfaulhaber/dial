use super::interpreter;
use super::interpreter::DialValue;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::io::Write;

pub struct Repl {
	running: bool,
}

impl Repl {
	pub fn new() -> Repl {
		Repl { running: false }
	}

	pub fn start(&self) {
		let mut rl = Editor::<()>::new();
		loop {
			let readline = rl.readline("user=> ");

			match readline {
				Ok(line) => {
					if !line.is_empty() {
						match interpreter::eval_line(line.as_str()) {
							Ok(value) => print_val(value),
							Err(err) => println!("error encountered: {:?}", err),
						};
					}
				}
				Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
					break;
				}
				Err(err) => {
					println!("Error: {:?}", err);
					break;
				}
			}
		}
	}

	pub fn close(&self) {
		unimplemented!();
	}
}

fn print_val(val: DialValue) {
	match val {
		DialValue::Integer(int) => println!("{}", int),
		DialValue::Float(float) => println!("{}", float),
		DialValue::String(s) => println!("{}", s),
		DialValue::Nil => println!("nil"),
	}
}
