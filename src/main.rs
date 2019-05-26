extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate rustyline;
#[macro_use]
extern crate log;
extern crate env_logger;

mod environment;
mod interpreter;
mod parser;
mod repl;

use repl::Repl;

fn main() {
    env_logger::init();
    let repl = Repl::new();
    repl.start();
}
