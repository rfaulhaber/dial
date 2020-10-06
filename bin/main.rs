use std::cell::RefCell;

use anyhow::Result;
use dial::{eval, read, Env, EvalResult};
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
                rl.add_history_entry(&line);
                let mut env = env.borrow_mut();
                let exprs_res = read(line);

                match exprs_res {
                    Ok(exprs) => {
                        let res: Vec<EvalResult> = exprs
                            .iter()
                            .map(|expr| eval(expr.clone(), &mut env))
                            .collect();

                        for r in res {
                            match r {
                                Ok(out) => println!("{}", out),
                                Err(out) => println!("{:?}", out),
                            }
                        }
                    }
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
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
