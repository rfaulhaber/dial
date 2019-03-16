use super::parser::{DialParser, Rule};
use log::Level;
use pest::error;
use pest::iterators::Pair;
use pest::Parser;
use std::iter::Sum;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug)]
pub enum DialValue {
	Integer(i64),
	Float(f64),
	String(String),
	Nil,
}

impl Add for DialValue {
	type Output = DialValue;

	fn add(self, other: DialValue) -> Self::Output {
		match self {
			DialValue::Integer(int) => match other {
				DialValue::Integer(other_int) => DialValue::Integer(int + other_int),
				DialValue::Float(float) => DialValue::Float(int as f64 + float),
				DialValue::String(s) => DialValue::String(format!("{}{}", int, s)),
				DialValue::Nil => self,
			},
			DialValue::Float(float) => match other {
				DialValue::Integer(int) => DialValue::Float(int as f64 + float),
				DialValue::Float(other_float) => DialValue::Float(float + other_float),
				DialValue::String(s) => DialValue::String(format!("{}{}", float, s)),
				DialValue::Nil => self,
			},
			DialValue::String(s) => match other {
				DialValue::Integer(int) => DialValue::String(format!("{}{}", s, int)),
				DialValue::Float(float) => DialValue::String(format!("{}{}", s, float)),
				DialValue::String(other_str) => DialValue::String(format!("{}{}", s, other_str)),
				DialValue::Nil => DialValue::String(s.clone()),
			},
			DialValue::Nil => other,
		}
	}
}

impl Sub for DialValue {
	type Output = DialValue;

	fn sub(self, other: DialValue) -> Self::Output {
		match self {
			DialValue::Integer(int) => match other {
				DialValue::Integer(other_int) => DialValue::Integer(int - other_int),
				DialValue::Float(float) => DialValue::Float(int as f64 - float),
				DialValue::String(_) | DialValue::Nil => DialValue::Nil,
			},
			DialValue::Float(float) => match other {
				DialValue::Integer(int) => DialValue::Float(int as f64 - float),
				DialValue::Float(other_float) => DialValue::Float(float - other_float),
				DialValue::String(_) | DialValue::Nil => DialValue::Nil,
			},
			DialValue::String(_) => DialValue::Nil,
			DialValue::Nil => other,
		}
	}
}

impl Mul for DialValue {
	type Output = DialValue;

	fn mul(self, other: DialValue) -> Self::Output {
		match self {
			DialValue::Integer(int) => match other {
				DialValue::Integer(other_int) => DialValue::Integer(int * other_int),
				DialValue::Float(float) => DialValue::Float(int as f64 * float),
				DialValue::String(_) | DialValue::Nil => DialValue::Nil,
			},
			DialValue::Float(float) => match other {
				DialValue::Integer(int) => DialValue::Float(int as f64 * float),
				DialValue::Float(other_float) => DialValue::Float(float * other_float),
				DialValue::String(_) | DialValue::Nil => DialValue::Nil,
			},
			DialValue::String(_) => DialValue::Nil,
			DialValue::Nil => other,
		}
	}
}

impl Div for DialValue {
	type Output = DialValue;

	fn div(self, other: DialValue) -> Self::Output {
		match self {
			DialValue::Integer(int) => match other {
				DialValue::Integer(other_int) => DialValue::Integer(int / other_int),
				DialValue::Float(float) => DialValue::Float(int as f64 / float),
				DialValue::String(_) | DialValue::Nil => DialValue::Nil,
			},
			DialValue::Float(float) => match other {
				DialValue::Integer(int) => DialValue::Float(int as f64 / float),
				DialValue::Float(other_float) => DialValue::Float(float / other_float),
				DialValue::String(_) | DialValue::Nil => DialValue::Nil,
			},
			DialValue::String(_) => DialValue::Nil,
			DialValue::Nil => other,
		}
	}
}

impl Sum for DialValue {
	fn sum<I>(iter: I) -> Self
	where
		I: Iterator<Item = Self>,
	{
		iter.fold(DialValue::Nil, |sum, val| sum + val)
	}
}

// TODO implement
// impl<'a> From<Pair<'a, Rule>> for DialType {
// }

pub fn eval_line(input: &str) -> Result<DialValue, error::Error<Rule>> {
	eval(input, Rule::repl_line)
}

fn eval(input: &str, rule: Rule) -> Result<DialValue, error::Error<Rule>> {
	let pair = DialParser::parse(rule, input)?.next().unwrap();

	if log_enabled!(Level::Info) {
		info!("pair: {:?}", pair);
		info!("as rule: {:?}", pair.as_rule());
	}

	let val = match pair.as_rule() {
		Rule::int => DialValue::Integer(pair.as_str().parse::<i64>().unwrap()),
		Rule::float => DialValue::Float(pair.as_str().parse::<f64>().unwrap()),
		Rule::expr => eval_expr(pair),
		Rule::COMMENT | Rule::nil => DialValue::Nil,
		_ => unimplemented!(),
	};

	Ok(val)
}

fn eval_expr(pair: Pair<Rule>) -> DialValue {
	let pair_str = pair.as_str();
	let mut inner = pair.into_inner();
	let first = inner.next().unwrap();

	match first.as_rule() {
		Rule::int => DialValue::Integer(pair_str.parse::<i64>().unwrap()),
		Rule::float => DialValue::Float(pair_str.parse::<f64>().unwrap()),
		Rule::add => inner.map(eval_expr).sum(),
		Rule::sub => inner
			.map(eval_expr)
			.fold(DialValue::Nil, |sum, val| sum - val),
		Rule::mul => inner
			.map(eval_expr)
			.fold(DialValue::Nil, |sum, val| sum * val),
		Rule::div => inner
			.map(eval_expr)
			.fold(DialValue::Nil, |sum, val| sum / val),

		// Rule::expr => eval_expr(inner),
		_ => unimplemented!(),
	}
}
