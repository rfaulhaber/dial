use super::ast::*;
use super::{EvalError, EvalResult};

pub fn add(vals: &[DialVal]) -> EvalResult {
	let mut sum = 0.0;

	for val in vals.iter() {
		let num = match val {
			DialVal::Atom(a) => match a {
				Atom::Int(i) => *i as f64,
				Atom::Float(f) => *f,
				_ => return Err(EvalError::TypeError("non-numeric type specified".into())),
			},
			_ => return Err(EvalError::TypeError("non-numeric type specified".into())),
		};

		sum += num;
	}

	Ok(DialVal::Atom(sum.into()))
}
