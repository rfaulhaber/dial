use super::interpreter::{EvalResult, Interpreter};
use super::parser::{DialParser, Expr, Rule};
use log::Level;
use pest::Parser;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::time::Instant;

pub struct Repl;

impl Repl {
    pub fn new() -> Repl {
        Repl {}
    }

    pub fn start(&self) {
        let mut rl = Editor::<()>::new();
        let mut interpreter = Interpreter::new();
        loop {
            let readline = rl.readline("user=> ");

            match readline {
                Ok(line) => {
                    if !line.is_empty() {
                        rl.add_history_entry(line.as_ref());
                        let parsed = DialParser::parse(Rule::repl_line, line.as_str());

                        match parsed {
                            Ok(result) => {
                                let exprs = result.map(Expr::from).map(|expr| {
                                    // TODO make more elegant, this is kind of weird
                                    if log_enabled!(Level::Info) {
                                        let start = Instant::now();
                                        let result = interpreter.eval(expr);
                                        let end = Instant::now();

                                        info!("duration: {:?}", end.duration_since(start));

                                        result
                                    } else {
                                        interpreter.eval(expr)
                                    }
                                });

                                exprs.for_each(print_eval_result);
                            }
                            // TODO make smarter
                            Err(err) => println!("error encountered in parsing: {:?}", err),
                        }
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

pub fn print_eval_result(er: EvalResult) {
    match er {
        Ok(result) => println!("{}", result),
        Err(err) => eprintln!("error: {}", err),
    }
}
