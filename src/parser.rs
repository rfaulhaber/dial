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
pub enum Expr {
	Integer(i64),
	Float(f64),
	Boolean(bool),
	String(String),
	Symbol(String),
	Identifier(String),
	Nil,
	List(Vec<Expr>),
}

impl fmt::Display for Expr {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Expr::Integer(i) => write!(f, "{}", i),
			Expr::Float(fl) => write!(f, "{}", fl),
			Expr::Boolean(b) => write!(f, "{}", b),
			Expr::String(s) => write!(f, "{}", s),
			Expr::Symbol(s) => write!(f, "{}", s),
			Expr::Identifier(s) => write!(f, "{}", s),
			Expr::Nil => write!(f, "nil"),
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
		Rule::nil => Expr::Nil,
		Rule::int => Expr::Integer(item.as_str().parse::<i64>().unwrap()),
		Rule::float => Expr::Float(item.as_str().parse::<f64>().unwrap()),
		Rule::boolean => {
			match item.as_str() {
				"true" => Expr::Boolean(true),
				_ => Expr::Boolean(false), // hopefully this is correct!
			}
		}
		Rule::string => Expr::String(String::from(item.as_str())),
		Rule::symbol => Expr::Symbol(String::from(item.as_str())),
		Rule::identifier => Expr::Identifier(String::from(item.as_str())),
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
				assert_eq!(l[0], Expr::Symbol(String::from("*")));
				assert_eq!(l[1], Expr::Integer(2));
				assert_eq!(
					l[2],
					Expr::List(vec![
						Expr::Symbol(String::from("+")),
						Expr::Integer(3),
						Expr::Integer(4),
						Expr::Integer(5),
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

		assert_eq!(parse_result.unwrap(), Expr::Integer(2));
	}
}

#[cfg(test)]
mod expr_iter_test {
	use super::*;

	#[test]
	fn iter_returns_correctly() {
		let parse_result = Expr::from_list("(* 2 (+ 3 4 5))");
		let mut iter = parse_result.unwrap().into_iter();

		assert_eq!(iter.next().unwrap(), Expr::Symbol(String::from("*")));
		assert_eq!(iter.next().unwrap(), Expr::Integer(2));
		assert_eq!(
			iter.next().unwrap(),
			Expr::List(vec![
				Expr::Symbol(String::from("+")),
				Expr::Integer(3),
				Expr::Integer(4),
				Expr::Integer(5),
			])
		);

		assert_eq!(iter.next(), None);
	}
}
