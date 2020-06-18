mod ast;
mod parse;

use anyhow::Result;
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub fn repl() -> Result<()> {
	let mut rl = Editor::<()>::new();
	loop {
		let line = rl.readline(">> ");
		match line {
			Ok(line) => {
				let res = parse::parse_sexpr(line.as_str());

				match res {
					Ok(expr) => println!("{}", expr),
					// TODO improve
					Err(e) => println!("{:?}", e)
				}
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

	Ok(())
}
