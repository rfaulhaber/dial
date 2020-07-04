mod ast;
mod env;
mod parse;

use std::{cell::RefCell, collections::HashMap};

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

pub type EvalResult = Result<ast::DialVal, EvalError>;

#[derive(Error, Debug, PartialEq)]
pub enum EvalError {
	#[error("undefined value: {0}")]
	Undefined(String),
	#[error("TypeError: expected {0}")]
	TypeError(String),
}

#[derive(Clone)]
pub struct Env {
	symbol_map: HashMap<String, DialVal>,
	scope: Option<Box<Env>>,
}

impl Default for Env {
	fn default() -> Self {
		let mut root = HashMap::new();

		root.insert(
			"+".into(),
			DialVal::Atom(Atom::Fn {
				name: "+".into(),
				func: env::add,
			}),
		);

		Env {
			symbol_map: root,
			scope: None,
		}
	}
}

impl Env {
	pub fn with_scope(scope: Env) -> Env {
		Env {
			symbol_map: HashMap::new(),
			scope: Some(Box::new(scope)),
		}
	}

	pub fn get_value(&self, sym: String) -> Option<&DialVal> {
		self.symbol_map.get(&sym).or_else(|| {
			if let Some(scope) = &self.scope {
				scope.get_value(sym)
			} else {
				None
			}
		})
	}
}

pub fn repl() -> Result<()> {
	let mut rl = Editor::<()>::new();
	let env = RefCell::new(Env::default());
	loop {
		let line_res = rl.readline(">> ");
		match line_res {
			Ok(line) => {
				let mut env = env.borrow_mut();
				let expr = parse::parse_sexpr(line);

				match expr {
					Ok(e) => match eval(e, &mut env) {
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

pub fn read(input: String) -> EvalResult {
	unimplemented!();
}

pub fn eval(val: DialVal, env: &mut Env) -> EvalResult {
	match val {
		DialVal::Atom(a) => match a {
			Atom::Sym(s) => env
				.get_value(s)
				.cloned()
				.ok_or_else(|| EvalError::Undefined("no such symbol".into())),
			_ => Ok(a.clone().into()),
		},
		DialVal::List(l) => {
			if l.is_empty() {
				Ok(DialVal::List(vec![]))
			} else {
				let (first, rest) = l.split_at(1);

				let first = first.get(0).unwrap();

				match eval(first.clone(), env) {
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

pub fn print(val: EvalResult) -> String {
	todo!();
}

#[cfg(test)]
mod mal_tests {
	use super::*;

	#[test]
	fn step_2_eval() {
		let inputs = vec![
			"1",
			"(+ 1 2 3)",
			"(- 5 4 1)",
			"(* 0.5 0.5 0.5)",
			"(/ 1 2 3)",
		];

		let mut env = Env::default();

		let results: Vec<EvalResult> = inputs
			.iter()
			.map(|input| eval(parse::parse_sexpr(input.to_string()).unwrap(), &mut env))
			.collect();

		assert_eq!(
			results,
			vec![
				Ok(DialVal::Atom(Atom::Int(1))),
				Ok(DialVal::Atom(Atom::Int(6))),
				Ok(DialVal::Atom(Atom::Int(0))),
				Ok(DialVal::Atom(Atom::Float(0.125))),
				Ok(DialVal::Atom(Atom::Float(1.0 / 6.0))),
			]
		)
	}
}
