use super::env::Env;
use super::values::DialValue;
use std::fmt;
use std::rc::Rc;

pub type Context = fn(Box<Env>) -> DialValue;

pub struct Func {
	name: Option<String>,
	binds: Vec<String>,
	func: Box<FnOnce(Box<Env>) -> DialValue>,
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

	pub fn new(name: Option<String>, binds: Vec<String>, func: Context) -> Func {
		Func { name, binds, func }
	}

	pub fn eval(&self, outer: Box<Env>, args: Vec<DialValue>) -> DialValue {
		let scope = Env::from_outer(outer);

		let zipped_args = self.binds.iter().zip(args.iter());

		for (symbol, value) in zipped_args {
			scope.set(symbol, value.clone());
		}

		(self.func)(Box::new(scope))
	}
}

#[cfg(test)]
mod test {
	use super::*;

	fn test_func_with_env(env: Box<Env>) -> DialValue {
		let value = env.get(&String::from("value")).unwrap();

		value + DialValue::Integer(100)
	}

	#[test]
	fn eval_with_env() {
		let env = Env::new();

		let func = Func::new(
			Some(String::from("test")),
			vec![String::from("value")],
			test_func_with_env,
		);

		let result = func.eval(Box::new(env), vec![DialValue::Integer(6)]);

		assert_eq!(result, DialValue::Integer(106));
	}
}
