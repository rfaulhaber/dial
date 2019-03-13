use super::interpreter;
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
					interpreter::eval_line(line.as_str());
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
