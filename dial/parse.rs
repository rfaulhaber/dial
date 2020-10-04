use super::ast::*;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while},
    character::complete::{anychar, char, digit1, multispace0, multispace1},
    combinator::{cut, map, map_res, recognize, verify},
    multi::separated_list,
    sequence::pair,
    sequence::{delimited, preceded, tuple},
    IResult,
};

use anyhow::Result;
use thiserror::Error;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Error, Debug)]
#[error("could not parse input {msg}")]
pub struct ParseError {
    msg: String,
}

pub fn parse_sexpr(input: String) -> ParseResult<DialVal> {
    match sexpr(&input) {
        Ok((_, expr)) => Ok(expr),
        Err(src) => Err(ParseError {
            msg: format!("{}", src),
        }),
    }
}

fn sexpr(input: &str) -> IResult<&str, DialVal> {
    preceded(multispace0, alt((atom_sexpr, sexpr_inner)))(input)
}

fn sexpr_inner(input: &str) -> IResult<&str, DialVal> {
    delimited(
        char('('),
        list_content,
        preceded(multispace0, cut(char(')'))),
    )(input)
}

fn list_content(input: &str) -> IResult<&str, DialVal> {
    map(
        preceded(
            multispace0,
            separated_list(multispace1, alt((atom_sexpr, sexpr_inner))),
        ),
        |v| DialVal::List(v),
    )(input)
}

fn atom_sexpr(input: &str) -> IResult<&str, DialVal> {
    map(atom, |a| DialVal::Atom(a))(input)
}

fn atom(input: &str) -> IResult<&str, Atom> {
    alt((float_atom, int_atom, str_atom, keyword_atom, sym_atom))(input)
}

fn int_atom(input: &str) -> IResult<&str, Atom> {
    map(int, |i: i64| Atom::Int(i))(input)
}

fn float_atom(input: &str) -> IResult<&str, Atom> {
    map(float, |f: f64| Atom::Float(f))(input)
}

fn str_atom(input: &str) -> IResult<&str, Atom> {
    map(str, |s| Atom::Str(s.into()))(input)
}

fn keyword_atom(input: &str) -> IResult<&str, Atom> {
    map(keyword, |s| Atom::Keyword(s.into()))(input)
}

fn sym_atom(input: &str) -> IResult<&str, Atom> {
    map(sym, |s| Atom::Sym(s.into()))(input)
}

fn int(input: &str) -> IResult<&str, i64> {
    alt((
        map_res(digit1, |digit_str: &str| digit_str.parse::<i64>()),
        map_res(preceded(tag("-"), digit1), |digit_str: &str| {
            digit_str.parse::<i64>().map(|i| -i)
        }),
    ))(input)
}

fn float(input: &str) -> IResult<&str, f64> {
    map_res(
        tuple((int, tag("."), int)),
        |(head, _, tail): (i64, &str, i64)| {
            let s = format!("{}.{}", head, tail);
            s.parse::<f64>()
        },
    )(input)
}

fn str(input: &str) -> IResult<&str, &str> {
    preceded(tag("\""), take_until("\""))(input)
}

fn keyword(input: &str) -> IResult<&str, &str> {
    preceded(tag(":"), sym)(input)
}

fn sym(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        verify(anychar, valid_first_sym_char),
        take_while(valid_sym_char),
    ))(input)
}

fn valid_first_sym_char(c: &char) -> bool {
    !c.is_whitespace() && !c.is_numeric() && !is_never_symbol(c)
}

fn valid_sym_char(c: char) -> bool {
    !c.is_whitespace() && (c.is_alphanumeric() || !is_never_symbol(&c))
}

fn is_never_symbol(c: &char) -> bool {
    matches!(c, '(' | ')')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_sym_works() {
        let input = "test)";
        let res = sym(input);

        assert_eq!(sym("test"), Ok(("", "test")));
    }

    #[test]
    fn sexpr_with_whitespace_parses() {
        let input = "   ( 123   456 )";
        let result = sexpr(input);

        let expected = Ok((
            "",
            DialVal::List(vec![
                DialVal::Atom(Atom::Int(123)),
                DialVal::Atom(Atom::Int(456)),
            ]),
        ));

        assert_eq!(result, expected, "could not parse {}", input);
    }

    #[test]
    fn sexpr_atom_parses() {
        let input = "123";
        let result = sexpr(input).unwrap();

        let expected = DialVal::Atom(Atom::Int(123));

        assert_eq!(result.1, expected);
    }

    #[test]
    fn sexpr_list_parses() {
        let input = "(123 456)";
        let result = sexpr(input);

        let expected = Ok((
            "",
            DialVal::List(vec![
                DialVal::Atom(Atom::Int(123)),
                DialVal::Atom(Atom::Int(456)),
            ]),
        ));

        assert_eq!(result, expected, "could not parse {}", input);
    }

    #[test]
    fn sexpr_single_list_parses() {
        let input = "( 123 )";
        let result = sexpr(input);

        let expected = Ok(("", DialVal::List(vec![DialVal::Atom(Atom::Int(123))])));

        assert_eq!(result, expected, "could not parse {}", input);
    }

    #[test]
    fn atom_test() {
        let inputs = vec!["12", "-34.5", r#""foo bar""#, ":foo", "foo"];
        let res: Vec<Atom> = inputs.iter().map(|s| atom(s).unwrap().1).collect();

        assert_eq!(
            res,
            vec![
                Atom::Int(12),
                Atom::Float(-34.5),
                Atom::Str("foo bar".into()),
                Atom::Keyword("foo".into()),
                Atom::Sym("foo".into())
            ]
        );

        let new_res = atom("(");
        assert!(new_res.is_err(), format!("res: {:?}", new_res));
    }

    #[test]
    fn odd_symbols_parse() {
        let inputs = vec!["+", "foo/bar", "baz-quux"];
        let res: Vec<Atom> = inputs.iter().map(|s| atom(s).unwrap().1).collect();

        assert_eq!(
            res,
            vec![
                Atom::Sym("+".into()),
                Atom::Sym("foo/bar".into()),
                Atom::Sym("baz-quux".into())
            ]
        );
    }

    #[test]
    fn int_test() {
        let inputs = vec!["-123", "4", "0"];
        let res: Vec<Atom> = inputs.iter().map(|s| int_atom(s).unwrap().1).collect();

        assert_eq!(res, vec![Atom::Int(-123), Atom::Int(4), Atom::Int(0)]);
    }

    #[test]
    fn float_test() {
        let inputs = vec!["0.123", "4.56", "-7.089"];
        let res: Vec<Atom> = inputs.iter().map(|s| float_atom(s).unwrap().1).collect();
        assert_eq!(
            res,
            vec![Atom::Float(0.123), Atom::Float(4.56), Atom::Float(-7.89)]
        );
    }
}
