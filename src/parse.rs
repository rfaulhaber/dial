use super::ast::*;
use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::digit1,
	combinator::{map, map_res},
	sequence::{preceded, tuple},
	IResult,
};

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub struct ParseError(String);

pub fn parse_sexpr<'s>(input: &str) -> ParseResult<Sexpr<'s>> {
	unimplemented!();
}

pub fn parse_atom<'a>(input: &str) -> ParseResult<Atom<'a>> {
	unimplemented!();
}

fn sexpr<'s>(input: &str) -> IResult<&str, Sexpr<'s>> {
	unimplemented!();
}

fn atom<'a>(input: &str) -> IResult<&str, Atom<'a>> {
	alt((
		float_atom,
		int_atom
	))(input)
}

fn int_atom<'a>(input: &str) -> IResult<&str, Atom<'a>> {
	map(int, |i: i64| Atom::Int(i))(input)
}

fn float_atom<'a>(input: &str) -> IResult<&str, Atom<'a>> {
	map(float, |f: f64| Atom::Float(f))(input)
}

fn int<'a>(input: &str) -> IResult<&str, i64> {
	alt((
		map_res(digit1, |digit_str: &str| digit_str.parse::<i64>()),
		map_res(preceded(tag("-"), digit1), |digit_str: &str| {
			digit_str.parse::<i64>().map(|i| -i)
		}),
	))(input)
}

fn float<'a>(input: &str) -> IResult<&str, f64> {
	map_res(
		tuple((int, tag("."), int)),
		|(head, _, tail): (i64, &str, i64)| {
			let s = format!("{}.{}", head, tail);
			s.parse::<f64>()
		},
	)(input)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn atom_test() {
		let inputs = vec!["12", "-34.5"];
		let res: Vec<Atom<'_>> = inputs.iter().map(|s| atom(s).unwrap().1).collect();

		assert_eq!(res, vec![
			Atom::Int(12),
			Atom::Float(-34.5)
		]);
	}

	#[test]
	fn int_test() {
		let inputs = vec!["-123", "4", "0"];
		let res: Vec<Atom<'_>> = inputs.iter().map(|s| int_atom(s).unwrap().1).collect();

		assert_eq!(res, vec![
			Atom::Int(-123),
			Atom::Int(4),
			Atom::Int(0)
		]);
	}

	#[test]
	fn float_test() {
		let inputs = vec!["0.123", "4.56", "-7.089"];
		let res: Vec<Atom<'_>> = inputs.iter().map(|s| float_atom(s).unwrap().1).collect();
		assert_eq!(res, vec![
			Atom::Float(0.123),
			Atom::Float(4.56),
			Atom::Float(-7.89)
		]);
	}
}
