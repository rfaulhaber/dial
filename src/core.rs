use super::interpreter::EvalResult;
use super::values::DialValue;

pub fn add(args: Vec<DialValue>) -> EvalResult {
	let sum = args.into_iter().sum();

	Ok(sum)
}

pub fn mul(args: Vec<DialValue>) -> EvalResult {
	let sum = args.into_iter().product();

	Ok(sum)
}
