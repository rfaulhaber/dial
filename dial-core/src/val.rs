use super::env::Env;
use num_rational::Rational64;
use std::{cell::RefCell, rc::Rc};

pub type BuiltinFunc = fn(&[Val], &mut Env) -> Val;

#[derive(Clone)]
pub enum Func {
    Lambda {
        name: Option<String>,
        params: Vec<String>,
        body: Box<Val>,
        env: Rc<RefCell<Env>>,
    },
    Builtin {
        name: String,
        params: Vec<String>,
        func: BuiltinFunc,
    },
}

#[derive(Clone)]
pub enum Val {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Sym(String),
    Keyword(String),
    Ratio(Rational64),
    Func(Func),
}
