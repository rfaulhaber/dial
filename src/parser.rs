use pest::error::LineColLocation;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use std::fmt;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct DialParser;

// TODO make some macros!
// TODO implement custom parsing error, returning useful values
pub type ParseResult = Result<Expr, String>;

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
	Integer(i64),
	Float(f64),
	Boolean(bool),
	String(String),
	Symbol(String),
	Identifier(String),
	Nil,
}

impl fmt::Display for Atom {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Atom::Integer(i) => write!(f, "{}", i),
			Atom::Float(fl) => write!(f, "{}", fl),
			Atom::Boolean(b) => write!(f, "{}", b),
			Atom::String(s) => write!(f, "{}", s),
			Atom::Symbol(s) => write!(f, "{}", s),
			Atom::Identifier(s) => write!(f, "{}", s),
			Atom::Nil => write!(f, "nil"),
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
	Atom(Atom),
	List(Vec<Expr>),
}

impl fmt::Display for Expr {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Expr::Atom(a) => write!(f, "{}", a),
			Expr::List(l) => {
				let mut out_str = String::new();

				for expr in l {
					out_str.push_str(format!("{} ", expr).as_str());
				}

				out_str.pop();

				write!(f, "({})", out_str)
			}
		}
	}
}

impl Into<Atom> for Expr {
	fn into(self) -> Atom {
		match self {
			Expr::Atom(a) => a,
			_ => panic!("cannot convert List into Atom"),
		}
	}
}

impl From<Atom> for Expr {
	fn from(atom: Atom) -> Self {
		Expr::Atom(atom)
	}
}

impl Expr {
	pub fn from_list(s: &str) -> ParseResult {
		let ast = DialParser::parse(Rule::list, s);

		match ast {
			Ok(mut result) => Ok(parse_pair(result.next().unwrap())),
			Err(err) => match err.line_col {
				LineColLocation::Pos((line, col)) => {
					let err_str = format!("error ({}, {}): invalid symbol", line, col);

					Err(err_str)
				}
				LineColLocation::Span((line, col), (end_line, end_col)) => Err(format!(
					"error ({}, {}), ({}, {}): invalid form",
					line, col, end_line, end_col
				)),
			},
		}
	}

	pub fn from_atom(s: &str) -> ParseResult {
		let ast = DialParser::parse(Rule::atom, s);

		match ast {
			Ok(mut result) => Ok(parse_atom(result.next().unwrap())),
			Err(err) => Err(format!("could not parse atom: {}", err)),
		}
	}

	pub fn is_list(&self) -> bool {
		match self {
			Expr::List(_) => true,
			_ => false,
		}
	}

	pub fn is_atom(&self) -> bool {
		match self {
			Expr::Atom(_) => true,
			_ => false,
		}
	}

	pub fn into_iter(&self) -> ExprIter {
		match self {
			Expr::List(l) => ExprIter {
				items: l.clone(),
				current: 0,
			},
			_ => ExprIter {
				items: vec![self.clone()],
				current: 0,
			},
		}
	}
}

fn parse_pair(pair: Pair<Rule>) -> Expr {
	match pair.as_rule() {
		Rule::atom => parse_atom(pair),
		Rule::list => {
			let mut inner = pair.into_inner();

			Expr::List(inner.map(parse_pair).collect())
		}
		_ => unreachable!(),
	}
}

fn parse_atom(pair: Pair<Rule>) -> Expr {
	// atoms should only be one thing, so this is okay
	let item = pair.into_inner().next().unwrap();

	match item.as_rule() {
		Rule::nil => Atom::Nil.into(),
		Rule::int => Atom::Integer(item.as_str().parse::<i64>().unwrap()).into(),
		Rule::float => Atom::Float(item.as_str().parse::<f64>().unwrap()).into(),
		Rule::boolean => {
			match item.as_str() {
				"true" => Atom::Boolean(true).into(),
				_ => Atom::Boolean(false).into(), // hopefully this is correct!
			}
		}
		Rule::string => Atom::String(String::from(item.as_str())).into(),
		Rule::symbol => Atom::Symbol(String::from(item.as_str())).into(),
		Rule::identifier => Atom::Identifier(String::from(item.as_str())).into(),
		_ => unreachable!(),
	}
}

// TODO implement custom iterator where, if atom, returns self
// otherwise, returns every expr in list
pub struct ExprIter {
	items: Vec<Expr>,
	current: usize,
}

impl Iterator for ExprIter {
	type Item = Expr;

	fn next(&mut self) -> Option<Self::Item> {
		let current = self.items.get(self.current);

		match current {
			Some(item) => {
				self.current += 1;
				Some(item.clone())
			}
			None => None,
		}
	}
}

#[cfg(test)]
mod parser_test {
	use super::*;

	#[test]
	fn display_expr() {
		let expected = "(* 2 (+ 3 4 5))";
		let expr = Expr::from_list(expected).unwrap();
		let result = format!("{}", expr);

		assert_eq!(expected, result);
	}

	#[test]
	fn from_list_returns_expr() {
		let parse_result = Expr::from_list("(* 2 (+ 3 4 5))");

		assert!(parse_result.is_ok());

		let result = parse_result.unwrap();

		assert!(result.is_list());

		match result {
			Expr::List(l) => {
				assert_eq!(l[0], Atom::Symbol(String::from("*")).into());
				assert_eq!(l[1], Atom::Integer(2).into());
				assert_eq!(
					l[2],
					Expr::List(vec![
						Atom::Symbol(String::from("+")).into(),
						Atom::Integer(3).into(),
						Atom::Integer(4).into(),
						Atom::Integer(5).into(),
					])
				);
			}
			_ => unreachable!(),
		}
	}

	#[test]
	fn from_atom_returns_expr() {
		let parse_result = Expr::from_atom("2");
		assert!(parse_result.is_ok());

		assert_eq!(parse_result.unwrap(), Atom::Integer(2).into());
	}
}

#[cfg(test)]
mod expr_iter_test {
	use super::*;

	#[test]
	fn iter_returns_correctly() {
		let parse_result = Expr::from_list("(* 2 (+ 3 4 5))");
		let mut iter = parse_result.unwrap().into_iter();

		assert_eq!(iter.next().unwrap(), Atom::Symbol(String::from("*")).into());
		assert_eq!(iter.next().unwrap(), Atom::Integer(2).into());
		assert_eq!(
			iter.next().unwrap(),
			Expr::List(vec![
				Atom::Symbol(String::from("+")).into(),
				Atom::Integer(3).into(),
				Atom::Integer(4).into(),
				Atom::Integer(5).into(),
			])
		);

		assert_eq!(iter.next(), None);
	}
}
