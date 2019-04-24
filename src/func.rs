use super::env::Env;
use super::values::DialValue;
use std::fmt;
use std::rc::Rc;

pub type Context = fn(Box<Env>) -> DialValue;

#[derive(Debug, PartialEq, Clone)]
pub enum Arity {
	Count(u8),
	Variable,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Func {
	name: Option<String>,
	arity: Arity,
	// env: Rc<Env>,
	func: Context,
}

impl fmt::Display for Func {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"#<function({})>",
			match self.name.clone() {
				Some(name) => name,
				None => String::from("anonymous"),
			}
		)
	}
}

impl Func {
	// TODO return DialEvalResult

	pub fn new(name: Option<String>, arity: Arity, func: Context) -> Func {
		Func { name, arity, func }
	}
	pub fn eval(&self, outer: Box<Env>) -> DialValue {
		(self.func)(outer)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	// fn test_func(args: Vec<DialValue>, env: Box<Env>) -> DialValue {
	// 	args.first().unwrap().clone()
	// }

	// fn test_func_sum(args: Vec<DialValue>, env: Box<Env>) -> DialValue {
	// 	args.into_iter().sum()
	// }

	fn test_func_with_env(env: Box<Env>) -> DialValue {
		let value = env.get(&String::from("value")).unwrap();

		value + DialValue::Integer(100)
	}

	// #[test]
	// fn eval_basic_test() {
	// 	let func = Func::new(Some(String::from("test")), Arity::Count(1), test_func);

	// 	let string_value = DialValue::from("hello!");

	// 	let result = func.eval(vec![string_value.clone()], Box::new(Env::new()));

	// 	assert_eq!(result, string_value.clone());
	// }

	// #[test]
	// fn eval_multiple_args() {
	// 	let args = vec![DialValue::from(1), DialValue::from(2), DialValue::from(3)];
	// 	let func = Func::new(Some(String::from("test")), Arity::Variable, test_func_sum);

	// 	let result = func.eval(args, Box::new(Env::new()));

	// 	assert_eq!(result, DialValue::Integer(6));
	// }

	#[test]
	fn eval_with_env() {
		let env = Env::new();
		env.set(&String::from("value"), DialValue::Integer(6));

		let func = Func::new(
			Some(String::from("test")),
			Arity::Variable,
			test_func_with_env,
		);

		let result = func.eval(Box::new(env));

		assert_eq!(result, DialValue::Integer(106));
	}
}
