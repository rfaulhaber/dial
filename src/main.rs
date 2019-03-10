extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate rustyline;

mod interpreter;
mod parser;
mod repl;

use repl::Repl;

fn main() {
    let mut repl = Repl::new();
    repl.start();
}
