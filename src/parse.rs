use super::ast::*;
use nom::{
	branch::alt,
	character::complete::digit1,
	combinator::{map, map_res},
	sequence::preceded,
	bytes::complete::tag,
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
	unimplemented!();
}

fn int<'a>(input: &str) -> IResult<&str, Atom<'a>> {
	alt((
		map_res(digit1, |digit_str: &str| {
			digit_str.parse::<i64>().map(Atom::Int)
		}),
		map(preceded(tag("-"), digit1), |digit_str: &str| {
			Atom::Int(-1 * digit_str.parse::<i64>().unwrap())
		}),
	))(input)
}

fn float<'a>(input: &str) -> IResult<&str, Atom<'a>> {
	unimplemented!();
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn int_test() {
		let valid = "-123";
		let res = int(valid);
		assert_eq!(res.unwrap().1, Atom::Int(-123));

		let invalid = "foo";
		let res = int(invalid);
		assert!(res.is_err());
	}
}
