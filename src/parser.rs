use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct DialParser;

pub struct ParseError;

// intermediate representation of expressions
#[derive(Debug, PartialEq, Clone)]
pub enum Sexpr {
	Integer(i64),
	Float(f64),
	Boolean(bool),
	String(String),
	Symbol(String),

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

#[cfg(test)]
mod tests {
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
		let parsed = DialParser::parse(Rule::list_expr, "(* 2 (+ 3 4 5))").unwrap();

		let three_rest = Sexpr::cons(
			Sexpr::Integer(3),
			Sexpr::cons(
				Sexpr::Integer(4),
				Sexpr::cons(Sexpr::Integer(5), Sexpr::Nil),
			),
		);
		let plus_rest = Sexpr::cons(Sexpr::Symbol(String::from("+")), three_rest);

		let mut sexprs = parsed.map(Sexpr::from_pair);

		assert_eq!(sexprs.next().unwrap(), Sexpr::Symbol(String::from("*")));
		assert_eq!(sexprs.next().unwrap(), Sexpr::Integer(2));
		assert_eq!(sexprs.next().unwrap(), plus_rest);
	}

}

// pub trait FromPair {
// 	fn from_pair(pair: Pair<Rule>) -> Self;
// }

// impl FromPair for Sexpr {
// 	fn from_pair(pair: Pair<Rule>) -> Self {}
// }
