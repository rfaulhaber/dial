use pest::iterators::Pair;
use std::collections::VecDeque;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct DialParser;

// TODO make some macros!

pub struct ParseError;

pub enum Atom {
	Integer(i64),
	Float(f64),
	Boolean(bool),
	String(String),
	Symbol(String),
	Identifier(String),
	Nil,
}

struct ConsCell {
	car: Box<Sexpr>,
	cdr: Box<Sexpr>,
}

// intermediate representation of expressions
#[derive(Debug, PartialEq, Clone)]
pub enum Sexpr {
	Atom(Atom),

	// (+ 1 2) => (+ . (1 . (2 . NIL))) => (cons + (cons 1 (cons 2 nil)))
	// terminal nils are not represented but general shape is as expected
	Cons(Box<Sexpr>, Box<Sexpr>),
	Nil,
}

// TODO implement iterator

impl Sexpr {
	pub fn from_pair(pair: Pair<Rule>) -> Sexpr {
		match pair.as_rule() {
			Rule::nil => Sexpr::Nil,
			Rule::float => parse_float(pair.as_str()),
			Rule::int => parse_int(pair.as_str()),
			Rule::boolean => parse_bool(pair.as_str()),
			Rule::string => parse_string(pair.as_str()),
			Rule::symbol => parse_symbol(pair.as_str()),
			Rule::identifier => Sexpr::Identifier(String::from(pair.as_str())),
			Rule::list => {
				let mut inner = pair.into_inner();
				let left = Sexpr::from_pair(inner.next().unwrap());
				let next = inner.next();

				if next.is_none() {
					Sexpr::cons(left, Sexpr::Nil)
				} else {
					let next_sexpr = Sexpr::from_pair(next.unwrap());

					let mut cdr = inner.map(Sexpr::from_pair).collect::<Vec<Sexpr>>();
					cdr.insert(0, next_sexpr);

					let mut base = Sexpr::Nil;

					for sexpr in cdr.into_iter().rev() {
						base = sexpr.cons_with(base);
					}

					left.cons_with(base)
				}
			}
			_ => unreachable!(),
		}
	}

	pub fn car(&self) -> Sexpr {
		match self {
			Sexpr::Cons(left, _) => *left.clone(),
			_ => self.clone(),
		}
	}

	pub fn cdr(&self) -> Sexpr {
		match self {
			Sexpr::Cons(_, right) => *right.clone(),
			_ => self.clone(),
		}
	}

	pub fn into_iter(&self) -> SexprIter {
		SexprIter::new(self.clone())
	}

	fn cons_with(&self, other: Sexpr) -> Sexpr {
		Sexpr::cons(self.clone(), other)
	}

	fn cons(left: Sexpr, right: Sexpr) -> Sexpr {
		Sexpr::Cons(Box::new(left), Box::new(right))
	}
}

impl From<&str> for Sexpr {
	fn from(item: &str) -> Self {
		Sexpr::String(String::from(item))
	}
}

fn parse_float(s: &str) -> Sexpr {
	let parsed = s.parse::<f64>().unwrap();
	Sexpr::Float(parsed)
}

fn parse_int(s: &str) -> Sexpr {
	let parsed = s.parse::<i64>().unwrap();
	Sexpr::Integer(parsed)
}

fn parse_bool(s: &str) -> Sexpr {
	match s {
		"true" => Sexpr::Boolean(true),
		"false" => Sexpr::Boolean(false),
		_ => unreachable!(), // hopefully!
	}
}

fn parse_string(s: &str) -> Sexpr {
	Sexpr::String(String::from(s))
}

fn parse_symbol(s: &str) -> Sexpr {
	Sexpr::Symbol(String::from(s))
}

pub struct SexprIter {
	queue: VecDeque<Sexpr>,
}

impl SexprIter {
	fn new(expr: Sexpr) -> Self {
		let queue = SexprIter::preorder(expr)
			.into_iter()
			.collect::<VecDeque<Sexpr>>();
		println!("queue: {:?}", queue);

		SexprIter { queue }
	}

	fn preorder(expr: Sexpr) -> Vec<Sexpr> {
		match expr {
			Sexpr::Cons(left, right) => {
				let mut left_tree = SexprIter::preorder(*left.clone());
				let mut right_tree = SexprIter::preorder(*right.clone());

				left_tree.append(&mut right_tree);

				left_tree
			}
			_ => vec![expr],
		}
	}
}

impl Iterator for SexprIter {
	type Item = Sexpr;

	fn next(&mut self) -> Option<Self::Item> {
		self.queue.pop_front()
	}
}

#[cfg(test)]
mod sexpr_tests {
	use super::*;
	use pest::Parser;

	#[test]
	fn from_atom() {
		let parsed = DialParser::parse(Rule::atom, "2").unwrap().next().unwrap();
		let result = Sexpr::from_pair(parsed);

		assert_eq!(result, Sexpr::Integer(2));
	}

	#[test]
	fn from_list() {
		let parsed = DialParser::parse(Rule::list, "(* 2 (+ 3 4 5))").unwrap();

		let root = Sexpr::cons(
			Sexpr::Symbol(String::from("*")),
			Sexpr::cons(
				Sexpr::Integer(2),
				Sexpr::cons(
					Sexpr::cons(
						Sexpr::Symbol(String::from("+")),
						Sexpr::cons(
							Sexpr::Integer(3),
							Sexpr::cons(Sexpr::Integer(4), Sexpr::Integer(5)),
						),
					),
					Sexpr::Nil,
				),
			),
		);

		let mut sexprs = parsed.map(Sexpr::from_pair);

		assert_eq!(sexprs.next().unwrap(), root);
	}

	#[test]
	fn two_lists() {
		let parsed = DialParser::parse(Rule::list, "((1 2) (3 4))").unwrap();

		let root = Sexpr::cons(
			Sexpr::cons(
				Sexpr::Integer(1),
				Sexpr::cons(Sexpr::Integer(2), Sexpr::Nil),
			),
			Sexpr::cons(
				Sexpr::Integer(3),
				Sexpr::cons(Sexpr::Integer(4), Sexpr::Nil),
			),
		);

		let mut sexprs = parsed.map(Sexpr::from_pair);

		assert_eq!(sexprs.next().unwrap(), root);
	}

	#[test]
	fn flat_list() {
		let parsed = DialParser::parse(Rule::list, "(1 2 3 4 5)").unwrap();

		let root = Sexpr::cons(
			Sexpr::Integer(1),
			Sexpr::cons(
				Sexpr::Integer(2),
				Sexpr::cons(
					Sexpr::Integer(3),
					Sexpr::cons(
						Sexpr::Integer(4),
						Sexpr::cons(Sexpr::Integer(5), Sexpr::Nil),
					),
				),
			),
		);

		let mut sexprs = parsed.map(Sexpr::from_pair);

		assert_eq!(sexprs.next().unwrap(), root);
	}
}

#[cfg(test)]
mod sexpriter_tests {
	use super::*;
	use pest::Parser;

	#[test]
	fn iter_through_sexpr() {
		let mut parsed = DialParser::parse(Rule::list, "(* 2 (+ 3 4 5))").unwrap();
		let mut sexprs = Sexpr::from_pair(parsed.next().unwrap()).into_iter();

		assert_eq!(sexprs.next().unwrap(), Sexpr::Symbol(String::from("*")));
		assert_eq!(sexprs.next().unwrap(), Sexpr::Integer(2));
		assert_eq!(sexprs.next().unwrap(), Sexpr::Symbol(String::from("+")));
		assert_eq!(sexprs.next().unwrap(), Sexpr::Integer(3));
		assert_eq!(sexprs.next().unwrap(), Sexpr::Integer(4));
		assert_eq!(sexprs.next().unwrap(), Sexpr::Integer(5));
		assert_eq!(sexprs.next().unwrap(), Sexpr::Nil);
		assert_eq!(sexprs.next().unwrap(), Sexpr::Nil);
		assert_eq!(sexprs.next(), None);
	}
}
