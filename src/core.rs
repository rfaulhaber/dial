use super::interpreter::EvalResult;
use super::parser::Sexpr;
use super::values::DialValue;

pub type BuiltinFunc = fn(args: Sexpr) -> EvalResult;

pub fn get_builtin(name: String) -> Option<BuiltinFunc> {
	match name.as_str() {
		"+" => Some(add),
		"*" => Some(mul),
		_ => None,
	}
}

pub fn add(args: Sexpr) -> EvalResult {
	match args {
		Sexpr::Atom(a) => match a {
			Atom::Integer(i) => 
		}
	}
	let sum = args.into_iter().sum();

	Ok(sum)
}

pub fn mul(args: Sexpr) -> EvalResult {
	let sum = args.into_iter().product();

	Ok(sum)
}
