use super::interpreter::{EvalResult, Interpreter};
use super::parser::{DialParser, Rule, Sexpr};
use super::values::DialValue;
use pest::Parser;
use rustyline::error::ReadlineError;
use rustyline::Editor;

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
                        let parsed = DialParser::parse(Rule::repl_line, line.as_str());

                        match parsed {
                            Ok(result) => {
                                let exprs = result
                                    .map(Sexpr::from_pair)
                                    .map(|expr| interpreter.eval(expr));

                                exprs.for_each(|result| print_eval_result(result));
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
