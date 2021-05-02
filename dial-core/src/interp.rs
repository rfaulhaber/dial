use std::error::Error;

use crate::{env::Env, val::Val};

pub struct Interpreter {
    env: Env,
}

impl Interpreter {
    // TODO make result
    pub fn eval(&mut self, val: &[Val]) -> Option<Val> {
        todo!();
    }
}
