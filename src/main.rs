extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate rustyline;

mod interpreter;
mod parser;
// mod repl;

use parser::{DialParser, Rule};
use pest::Parser;
use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline("user=>");

        match readline {
            Ok(line) => {
                let parse_result = match DialParser::parse(Rule::statement, line.as_str()) {
                    Ok(result) => result,
                    Err(err) => {
                        // TODO clean up
                        println!("unexpected error: {:?}", err);
                        break;
                    }
                };

                for pair in parse_result {
                    for inner_pair in pair.into_inner() {
                        let rule = inner_pair.as_rule();
                        println!("rule: {:?}", rule);
                        match rule {
                            Rule::add => {
                                println!("add");
                            }
                            Rule::subtract => {
                                println!("subtract");
                            }
                            Rule::multiply => {
                                println!("multiply");
                            }
                            Rule::divide => {
                                println!("divide");
                            }
                            Rule::power => {
                                println!("power");
                            }
                            Rule::expr => {
                                println!("expr");
                            }
                            _ => unreachable!(),
                        }
                    }
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
}
