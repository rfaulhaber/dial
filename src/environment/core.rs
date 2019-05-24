use crate::interpreter::EvalResult;
use crate::parser::{Atom, Expr};

pub fn list(args: &[Expr]) -> EvalResult {
	Ok(Expr::List(args.to_vec()))
}