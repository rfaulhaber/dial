extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate rustyline;
#[macro_use]
extern crate log;
extern crate env_logger;

mod env;
// mod func;
mod interpreter;
mod parser;
mod repl;
mod values;

use repl::Repl;

fn main() {
    env_logger::init();
    let mut repl = Repl::new();
    repl.start();
}
