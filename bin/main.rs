use std::cell::RefCell;

use anyhow::Result;
use dial::{eval, parse, Env};
use rustyline::{error::ReadlineError, Editor};

fn main() {
    repl();
}

pub fn repl() -> Result<()> {
    let mut rl = Editor::<()>::new();
    let env = RefCell::new(Env::default());
    loop {
        let line_res = rl.readline(">> ");
        match line_res {
            Ok(line) => {
                let mut env = env.borrow_mut();
                let expr = parse::parse_sexpr(line);

                match expr {
                    Ok(e) => match eval(e, &mut env) {
                        Ok(out) => println!("{}", out),
                        Err(out) => println!("{:?}", out),
                    },
                    Err(out) => println!("{:?}", out),
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
