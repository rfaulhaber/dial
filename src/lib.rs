mod ast;
mod parse;

use std::{borrow::Cow, cell::RefCell, collections::HashMap, rc::Rc};

use anyhow::Result;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use thiserror::Error;

use ast::{Atom, DialVal};

macro_rules! extract_atom_val {
	($val:ident, $b:block, $($p:path)|+) => {
		match $val {
			DialVal::Atom(a) match a {
				$($p(v))|+ => v,
				_ => $b,
			}
			_ => $b
		}
	}
}

pub type EvalResult<'e> = Result<ast::DialVal<'e>, EvalError>;

#[derive(Error, Debug)]
pub enum EvalError {
	#[error("undefined value: {0}")]
	Undefined(String),
	#[error("TypeError: expected {0}")]
	TypeError(String),
}

#[derive(Clone)]
pub struct Env<'e> {
	symbol_map: HashMap<&'e str, DialVal<'e>>,
	scope: Option<&'e Env<'e>>,
}

impl<'e> Default for Env<'e> {
	fn default() -> Self {
		let mut root = HashMap::new();

		let add = |vals: &[DialVal]| -> EvalResult<'e> {
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

			Ok(Atom::Float(sum).into())
		};

		root.insert(
			"+",
			DialVal::Atom(Atom::Fn {
				name: "+",
				func: add,
			}),
		);

		Env {
			symbol_map: root,
			scope: None,
		}
	}
}

impl<'e> Env<'e> {
	pub fn with_scope(scope: &'e Env<'e>) -> Env<'e> {
		Env {
			symbol_map: HashMap::new(),
			scope: Some(scope),
		}
	}

	pub fn get_value(&self, sym: &str) -> Option<&DialVal<'e>> {
		self.symbol_map.get(sym).or_else(|| {
			if let Some(scope) = self.scope {
				scope.get_value(sym)
			} else {
				None
			}
		})
	}
}

pub fn repl() -> Result<()> {
	let mut rl = Editor::<()>::new();
	let env = Env::default();
	loop {
		let line_res = rl.readline(">> ");
		match line_res {
			Ok(line) => {
				let expr = parse::parse_sexpr(&line);

				match expr {
					Ok(e) => match eval(&e, &Env::default()) {
						Ok(out) => println!("{}", out),
						Err(out) => println!("{:?}", out),
					},
					Err(out) => println!("{:?}", out),
				}
			}
			Err(ReadlineError::Interrupted) => {
				println!("CTRL-C");
				break;
			}
			Err(ReadlineError::Eof) => {
				println!("CTRL-D");
				break;
			}
			Err(err) => {
				println!("Error: {:?}", err);
				break;
			}
		}
	}

	Ok(())
}

pub fn read<'a>(input: String) -> EvalResult<'a> {
	unimplemented!();
}

pub fn eval<'a>(val: &'a DialVal<'a>, env: &'a Env<'a>) -> EvalResult<'a> {
	match val {
		DialVal::Atom(a) => match a {
			Atom::Sym(s) => env
				.get_value(s)
				.map(|o| o.clone())
				.ok_or(EvalError::Undefined("no such symbol".into())),
			_ => Ok(a.clone().into()),
		},
		DialVal::List(l) => {
			if l.is_empty() {
				Ok(DialVal::List(vec![]))
			} else {
				let (first, rest) = l.split_at(1);

				let first = first.get(0).unwrap();

				match eval(first, env) {
					Ok(dv) => match dv {
						DialVal::Atom(a) => match a {
							Atom::Fn { func, .. } => func(rest),
							_ => Err(EvalError::TypeError(format!("{} is not a function", first))),
						},
						_ => Err(EvalError::TypeError(format!("{} is not a function", first))),
					},
					Err(e) => Err(e),
				}
			}
		}
	}
}

pub fn print<'a>(val: EvalResult<'a>) -> String {
	todo!();
}
