use super::env::Env;
use super::parser::Rule;
use pest::iterators::Pair;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub struct FuncRef {
	env: Rc<Env>,
	pair: Pair<'static, Rule>,
}

pub fn new(outer: Rc<Env>, pair: Pair<Rule>) -> FuncRef {
	FuncRef { env: outer, pair }
}

impl<'a> FuncRef<'a> {
	pub fn get_context(&self) -> (&Env, Pair<'a, Rule>) {
		(self.env.as_ref(), self.pair.clone())
	}
}

impl<'a> fmt::Display for FuncRef<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "#<function>")
	}
}
