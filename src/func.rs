use super::env::Env;
use super::values::DialValue;
use super::parser::{Sexpr};
use std::fmt;
use std::rc::Rc;

pub enum Func {
	BuiltinFunc(fn(Sexpr))
}

pub struct BuiltinFunc()