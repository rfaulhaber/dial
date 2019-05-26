use crate::interpreter::EvalResult;
use crate::parser::Expr;

pub fn list(args: &[Expr]) -> EvalResult {
	Ok(Expr::List(args.to_vec()))
}

pub fn is_list(args: &[Expr]) -> EvalResult {
	match &args[0] {
		Expr::List(_) => Ok(true.into()),
		_ => Ok(false.into()),
	}
}

pub fn is_empty(args: &[Expr]) -> EvalResult {
	match &args[0] {
		Expr::List(list) => Ok(list.is_empty().into()),
		Expr::Vector(vec) => Ok(vec.is_empty().into()),
		_ => Err("not a list or vector".to_string()),
	}
}