use crate::environment::env::Env;
use pest::error::LineColLocation;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use std::cmp;
use std::fmt;
use std::rc::Rc;

// TODO refactor using this: https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#destructuring-nested-structs-and-enums

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct DialParser;

// TODO implement custom parsing error, returning useful values
pub type ParseResult = Result<Expr, String>;

#[derive(Clone)]
pub enum Atom {
	Integer(i64),
	Float(f64),
	Boolean(bool),
	String(String),
	Symbol(String),
	Identifier(String),
	Ratio { num: i64, den: i64 },
	// macros are built-in functions
	Macro(fn(&[Expr]) -> Result<Expr, String>),
	// lambdas are user-defined functions
	Lambda(Lambda),
	Nil,
}

impl cmp::PartialEq for Atom {
	fn eq(&self, other: &Atom) -> bool {
		match (self, other) {
			(Atom::Integer(left), Atom::Integer(right)) => left == right,
			(Atom::Float(left), Atom::Float(right)) => left == right,
			(Atom::Boolean(left), Atom::Boolean(right)) => left == right,
			(Atom::String(left), Atom::String(right)) => left == right,
			(Atom::Symbol(left), Atom::Symbol(right)) => left == right,
			(Atom::Identifier(left), Atom::Identifier(right)) => left == right,
			(
				Atom::Ratio {
					num: l_num,
					den: l_den,
				},
				Atom::Ratio {
					num: r_num,
					den: r_den,
				},
			) => l_num == r_num && l_den == r_den,
			(Atom::Macro(left), Atom::Macro(right)) => false, // TODO temporary, change this
			(Atom::Lambda(left), Atom::Lambda(right)) => false, // TODO temporary, change this
			(Atom::Nil, Atom::Nil) => true,
			_ => false,
		}
	}
}

impl fmt::Debug for Atom {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Atom::Integer(i) => write!(f, "{:?}", i),
			Atom::Float(fl) => write!(f, "{:?}", fl),
			Atom::Boolean(b) => write!(f, "{:?}", b),
			Atom::String(s) => write!(f, "{:?}", s),
			Atom::Symbol(s) => write!(f, "{:?}", s),
			Atom::Identifier(s) => write!(f, "{:?}", s),
			Atom::Ratio { num, den } => write!(f, "{:?}/{:?}", num, den),
			Atom::Macro(func) => write!(f, "#{{core}}"),
			Atom::Lambda(lambda) => write!(f, "#{{lambda}}"),
			Atom::Nil => write!(f, "nil"),
		}
	}
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
			Atom::Ratio { num, den } => write!(f, "{}/{}", num, den),
			Atom::Macro(func) => write!(f, "#{{core}}"),
			Atom::Lambda(lambda) => write!(f, "#{{lambda}}"),
			Atom::Nil => write!(f, "nil"),
		}
	}
}

impl From<i64> for Atom {
	fn from(int: i64) -> Self {
		Atom::Integer(int)
	}
}

impl From<f64> for Atom {
	fn from(float: f64) -> Self {
		Atom::Float(float)
	}
}

impl From<bool> for Atom {
	fn from(b: bool) -> Self {
		Atom::Boolean(b)
	}
}

impl From<String> for Atom {
	fn from(s: String) -> Self {
		Atom::String(s)
	}
}

impl From<&str> for Atom {
	fn from(s: &str) -> Self {
		Atom::String(String::from(s))
	}
}

#[derive(Debug, Clone)]
pub enum Expr {
	Atom(Atom),
	List(Vec<Expr>),
	Vector(Vec<Expr>),
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
			Expr::Vector(v) => {
				let mut out_str = String::new();

				for expr in v {
					out_str.push_str(format!("{} ", expr).as_str());
				}

				out_str.pop();

				write!(f, "[{}]", out_str)
			}
		}
	}
}

impl cmp::PartialEq for Expr {
	fn eq(&self, other: &Expr) -> bool {
		match (self, other) {
			(Expr::Atom(left), Expr::Atom(right)) => left == right,
			(Expr::List(left), Expr::List(right)) => {
				if left.len() == right.len() {
					for (left_item, right_item) in left.iter().zip(right.iter()) {
						if left_item != right_item {
							return false;
						}
					}

					true
				} else {
					false
				}
			}
			(Expr::Vector(left), Expr::Vector(right)) => {
				if left.len() == right.len() {
					for (left_item, right_item) in left.iter().zip(right.iter()) {
						if left_item != right_item {
							return false;
						}
					}

					true
				} else {
					false
				}
			}
			_ => false,
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

impl From<i64> for Expr {
	fn from(int: i64) -> Self {
		Expr::Atom(int.into())
	}
}

impl From<f64> for Expr {
	fn from(float: f64) -> Self {
		Expr::Atom(float.into())
	}
}

impl From<bool> for Expr {
	fn from(b: bool) -> Self {
		Expr::Atom(b.into())
	}
}

impl From<String> for Expr {
	fn from(s: String) -> Self {
		Expr::Atom(s.into())
	}
}

impl From<&str> for Expr {
	fn from(s: &str) -> Self {
		Expr::Atom(s.into())
	}
}

impl From<Lambda> for Expr {
	fn from(l: Lambda) -> Self {
		Expr::Atom(Atom::Lambda(l))
	}
}

impl<'a> From<Pair<'a, Rule>> for Expr {
	fn from(pair: Pair<'a, Rule>) -> Self {
		parse_pair(pair)
	}
}

impl<'a> From<Pairs<'a, Rule>> for Expr {
	fn from(pairs: Pairs<'a, Rule>) -> Self {
		let list: Vec<Expr> = pairs.map(parse_pair).collect();

		println!("list len: {}", list.len());

		list[0].clone()
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

	pub fn is_vector(&self) -> bool {
		match self {
			Expr::Vector(_) => true,
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

	pub fn as_atom(&self) -> Option<Atom> {
		match self {
			Expr::Atom(a) => Some(a.clone()),
			_ => None,
		}
	}
}

fn parse_pair(pair: Pair<Rule>) -> Expr {
	match pair.as_rule() {
		Rule::atom => parse_atom(pair),
		Rule::list => {
			let inner = pair.into_inner();
			Expr::List(inner.map(parse_pair).collect())
		}
		Rule::vector => {
			let inner = pair.into_inner();
			Expr::Vector(inner.map(parse_pair).collect())
		}
		_ => unreachable!(),
	}
}

fn parse_atom(pair: Pair<Rule>) -> Expr {
	// atoms should only be one thing, so this is okay
	let item = pair.into_inner().next().unwrap();

	info!("item as rule: {:?}", item.as_rule());
	info!("item as span: {:?}", item.as_span());

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
		Rule::token => Atom::Symbol(String::from(item.as_str())).into(),
		Rule::identifier => Atom::Identifier(String::from(item.as_str())).into(),
		_ => unreachable!(),
	}
}

#[derive(Clone)]
pub struct Lambda {
	pub params: Vec<String>,
	pub body: Box<Expr>,
}

impl Lambda {
	pub fn new(params: Vec<String>, body: Box<Expr>) -> Lambda {
		Lambda { params, body }
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
		let pairs = DialParser::parse(Rule::list, "(* 2 (+ 3 4 5))").unwrap();
		let result = Expr::from(pairs);

		assert!(result.is_list());

		let expected = Expr::List(vec![
			Atom::Symbol(String::from("*")).into(),
			Atom::Integer(2).into(),
			Expr::List(vec![
				Atom::Symbol(String::from("+")).into(),
				Atom::Integer(3).into(),
				Atom::Integer(4).into(),
				Atom::Integer(5).into(),
			]),
		]);

		assert_eq!(result, expected);
	}

	#[test]
	fn from_atom_returns_expr() {
		let parse_result = Expr::from_atom("2");
		assert!(parse_result.is_ok());

		assert_eq!(parse_result.unwrap(), Atom::Integer(2).into());
	}

	#[test]
	fn from_paris_returns_expr() {
		let parse_result = DialParser::parse(Rule::list, "(* 2 (+ 3 4 5))").unwrap();
		let expr = Expr::from(parse_result);

		match expr {
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
