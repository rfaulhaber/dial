use pest::iterators::{Pair, Pairs};
use std::collections::VecDeque;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct DialParser;

// TODO make some macros!

pub struct ParseError;

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

// intermediate representation of expressions
#[derive(Debug, PartialEq, Clone)]
pub enum Sexpr {
	Atom(Atom),

	// (+ 1 2) => (+ . (1 . (2 . NIL))) => (cons + (cons 1 (cons 2 nil)))
	// terminal nils are not represented but general shape is as expected
	Cons(Box<Sexpr>, Box<Sexpr>),
}

impl Sexpr {
	pub fn from_pair(pair: Pair<Rule>) -> Sexpr {
		match pair.as_rule() {
			Rule::atom => parse_atom(pair),
			Rule::list => {
				let mut inner = pair.into_inner();

				let car = Sexpr::from_pair(inner.next().unwrap());
				let cdr = Sexpr::from_pairs(&mut inner);

				Sexpr::cons(car, cdr)
			}
			_ => unreachable!(),
		}
	}

	fn from_pairs(pairs: &mut Pairs<Rule>) -> Sexpr {
		let next = pairs.next();

		if next.is_none() {
			Sexpr::Atom(Atom::Nil)
		} else {
			Sexpr::cons(Sexpr::from_pair(next.unwrap()), Sexpr::from_pairs(pairs))
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

	// pub fn into_iter(&self) -> SexprIter {
	// 	SexprIter::new(self.clone())
	// }

	fn cons_with(&self, other: Sexpr) -> Sexpr {
		Sexpr::cons(self.clone(), other)
	}

	fn cons(left: Sexpr, right: Sexpr) -> Sexpr {
		Sexpr::Cons(Box::new(left), Box::new(right))
	}
}

fn parse_atom(pair: Pair<Rule>) -> Sexpr {
	let pair_str = pair.as_str();

	let atom = match pair.into_inner().next().unwrap().as_rule() {
		Rule::nil => Atom::Nil,
		Rule::float => parse_float(pair_str),
		Rule::int => parse_int(pair_str),
		Rule::boolean => parse_bool(pair_str),
		Rule::string => parse_string(pair_str),
		Rule::symbol => parse_symbol(pair_str),
		Rule::identifier => parse_identifier(pair_str),
		_ => unreachable!(),
	};

	Sexpr::Atom(atom)
}

fn parse_float(s: &str) -> Atom {
	let parsed = s.parse::<f64>().unwrap();
	Atom::Float(parsed)
}

fn parse_int(s: &str) -> Atom {
	let parsed = s.parse::<i64>().unwrap();
	Atom::Integer(parsed)
}

fn parse_bool(s: &str) -> Atom {
	match s {
		"true" => Atom::Boolean(true),
		"false" => Atom::Boolean(false),
		_ => unreachable!(), // hopefully!
	}
}

fn parse_string(s: &str) -> Atom {
	Atom::String(String::from(s))
}

fn parse_symbol(s: &str) -> Atom {
	Atom::Symbol(String::from(s))
}

fn parse_identifier(s: &str) -> Atom {
	Atom::Identifier(String::from(s))
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

		assert_eq!(result, Sexpr::Atom(Atom::Integer(2)));
	}

	#[test]
	fn from_list() {
		let parsed = DialParser::parse(Rule::list, "(* 2 (+ 3 4 5))").unwrap();

		let mul = Atom::Symbol(String::from("*"));
		let two = Atom::Integer(2);
		let plus = Atom::Symbol(String::from("+"));
		let three = Atom::Integer(3);
		let four = Atom::Integer(4);
		let five = Atom::Integer(5);

		let root = Sexpr::cons(
			Sexpr::Atom(mul),
			Sexpr::cons(
				Sexpr::Atom(two),
				Sexpr::cons(
					Sexpr::cons(
						Sexpr::Atom(plus),
						Sexpr::cons(
							Sexpr::Atom(three),
							Sexpr::cons(
								Sexpr::Atom(four),
								Sexpr::cons(Sexpr::Atom(five), Sexpr::Atom(Atom::Nil)),
							),
						),
					),
					Sexpr::Atom(Atom::Nil),
				),
			),
		);

		let mut sexprs = parsed.map(Sexpr::from_pair);

		assert_eq!(sexprs.next().unwrap(), root);
	}

	#[test]
	fn two_lists() {
		let parsed = DialParser::parse(Rule::list, "((1 2) (3 4))").unwrap();

		let left = Sexpr::cons(
			Sexpr::Atom(Atom::Integer(1)),
			Sexpr::cons(Sexpr::Atom(Atom::Integer(2)), Sexpr::Atom(Atom::Nil)),
		);

		let right = Sexpr::cons(
			Sexpr::Atom(Atom::Integer(3)),
			Sexpr::cons(Sexpr::Atom(Atom::Integer(4)), Sexpr::Atom(Atom::Nil)),
		);

		let root = Sexpr::cons(left, Sexpr::cons(right, Sexpr::Atom(Atom::Nil)));

		let mut sexprs = parsed.map(Sexpr::from_pair);

		assert_eq!(sexprs.next().unwrap(), root);
	}

	#[test]
	fn flat_list() {
		let parsed = DialParser::parse(Rule::list, "(1 2 3 4 5)").unwrap();

		let root = Sexpr::cons(
			Sexpr::Atom(Atom::Integer(1)),
			Sexpr::cons(
				Sexpr::Atom(Atom::Integer(2)),
				Sexpr::cons(
					Sexpr::Atom(Atom::Integer(3)),
					Sexpr::cons(
						Sexpr::Atom(Atom::Integer(4)),
						Sexpr::cons(Sexpr::Atom(Atom::Integer(5)), Sexpr::Atom(Atom::Nil)),
					),
				),
			),
		);

		let mut sexprs = parsed.map(Sexpr::from_pair);

		assert_eq!(sexprs.next().unwrap(), root);
	}
}

// #[cfg(test)]
// mod sexpriter_tests {
// 	use super::*;
// 	use pest::Parser;

// 	#[test]
// 	fn iter_through_sexpr() {
// 		let mut parsed = DialParser::parse(Rule::list, "(* 2 (+ 3 4 5))").unwrap();
// 		let mut sexprs = Sexpr::from_pair(parsed.next().unwrap()).into_iter();

// 		assert_eq!(sexprs.next().unwrap(), Sexpr::Symbol(String::from("*")));
// 		assert_eq!(sexprs.next().unwrap(), Sexpr::Integer(2));
// 		assert_eq!(sexprs.next().unwrap(), Sexpr::Symbol(String::from("+")));
// 		assert_eq!(sexprs.next().unwrap(), Sexpr::Integer(3));
// 		assert_eq!(sexprs.next().unwrap(), Sexpr::Integer(4));
// 		assert_eq!(sexprs.next().unwrap(), Sexpr::Integer(5));
// 		assert_eq!(sexprs.next().unwrap(), Sexpr::Nil);
// 		assert_eq!(sexprs.next().unwrap(), Sexpr::Nil);
// 		assert_eq!(sexprs.next(), None);
// 	}
// }
