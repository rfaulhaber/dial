use super::interpreter::Interpreter;
use super::values::DialValue;
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub struct Repl;

impl Repl {
	pub fn new() -> Repl {
		Repl {}
	}

	pub fn start(&self) {
		let mut rl = Editor::<()>::new();
		let interpreter = Interpreter::new();
		loop {
			let readline = rl.readline("user=> ");

			match readline {
				Ok(line) => {
					if !line.is_empty() {
						match interpreter.eval_repl(line.as_str()) {
							Ok(values) => {
								for value in values {
									print_val(value);
								}
							}
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
