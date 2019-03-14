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
					if line.is_empty() {
						continue;
					}

					match interpreter::eval_line(line.as_str()) {
						Ok(value) => match value {
							DialValue::Nil => println!("nil"),
							_ => println!("unimplemented! value: {:?}", value),
						},
						Err(err) => println!("error encountered: {:?}", err),
					};
				}
				Err(ReadlineError::Interrupted) => {
					println!("CTRL-C");
					break;
				}
				Err(ReadlineError::Eof) => {
					println!("CTRL-D");
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
