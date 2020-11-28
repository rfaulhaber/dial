use super::sexpr::*;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while},
    character::complete::{alpha1, anychar, char, digit1, multispace0, multispace1},
    combinator::{all_consuming, cut, map, map_res, recognize, verify},
    multi::{many1, separated_list0},
    sequence::{delimited, pair, preceded, tuple},
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

pub fn parse_program(input: String) -> ParseResult<Vec<DialVal>> {
    match program(&input) {
        Ok((_, expr)) => Ok(expr),
        Err(src) => Err(ParseError {
            msg: format!("{}", src),
        }),
    }
}

pub fn parse_sexpr(input: String) -> ParseResult<DialVal> {
    match sexpr(&input) {
        Ok((_, expr)) => Ok(expr),
        Err(src) => Err(ParseError {
            msg: format!("{}", src),
        }),
    }
}

fn program(input: &str) -> IResult<&str, Vec<DialVal>> {
    all_consuming(many1(sexpr))(input)
}

fn sexpr(input: &str) -> IResult<&str, DialVal> {
    preceded(multispace0, alt((atom, sexpr_inner, vector)))(input)
}

fn sexpr_inner(input: &str) -> IResult<&str, DialVal> {
    inner_list('(', ')', |v| DialVal::List(v))(input)
}

fn atom(input: &str) -> IResult<&str, DialVal> {
    alt((
        float_atom,
        int_atom,
        str_atom,
        bool_atom,
        nil_atom,
        keyword_atom,
        sym_atom,
    ))(input)
}

fn vector(input: &str) -> IResult<&str, DialVal> {
    inner_list('[', ']', |v| DialVal::Vec(v))(input)
}

fn sexpr_contents(input: &str) -> IResult<&str, DialVal> {
    alt((atom, sexpr_inner, vector))(input)
}

fn int_atom(input: &str) -> IResult<&str, DialVal> {
    map(int, |i: i64| DialVal::Int(i))(input)
}

fn float_atom(input: &str) -> IResult<&str, DialVal> {
    map(float, |f: f64| DialVal::Float(f))(input)
}

fn str_atom(input: &str) -> IResult<&str, DialVal> {
    map(str, |s| DialVal::Str(s.into()))(input)
}

fn keyword_atom(input: &str) -> IResult<&str, DialVal> {
    map(keyword, |s| DialVal::Keyword(s.into()))(input)
}

fn sym_atom(input: &str) -> IResult<&str, DialVal> {
    map(sym, |s| DialVal::Sym(s.into()))(input)
}

fn bool_atom(input: &str) -> IResult<&str, DialVal> {
    map(bool, |s| {
        DialVal::Bool(match s {
            "true" => true,
            "false" => false,
            _ => unreachable!(),
        })
    })(input)
}

fn nil_atom(input: &str) -> IResult<&str, DialVal> {
    map(nil, |_| DialVal::Nil)(input)
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

fn bool(input: &str) -> IResult<&str, &str> {
    recognize(verify(alpha1, |s: &str| s == "true" || s == "false"))(input)
}

fn nil(input: &str) -> IResult<&str, &str> {
    recognize(verify(alpha1, |s: &str| s == "nil"))(input)
}

// TODO make return function
fn inner_list<'a, F>(
    open_delim: char,
    close_delim: char,
    func: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, DialVal>
where
    F: FnMut(Vec<DialVal>) -> DialVal,
{
    delimited(
        char(open_delim),
        map(
            preceded(
                multispace0,
                separated_list0(multispace1, alt((atom, sexpr_inner, vector))),
            ),
            func,
        ),
        preceded(multispace0, cut(char(close_delim))),
    )
}

fn valid_first_sym_char(c: &char) -> bool {
    !c.is_whitespace() && !c.is_numeric() && !is_never_symbol(c)
}

fn valid_sym_char(c: char) -> bool {
    !c.is_whitespace() && (c.is_alphanumeric() || !is_never_symbol(&c))
}

fn is_never_symbol(c: &char) -> bool {
    matches!(c, '(' | ')' | '[' | ']')
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
            DialVal::List(vec![DialVal::Int(123), DialVal::Int(456)]),
        ));

        assert_eq!(result, expected, "could not parse {}", input);
    }

    #[test]
    fn sexpr_atom_parses() {
        let input = "123";
        let result = sexpr(input).unwrap();

        let expected = DialVal::Int(123);

        assert_eq!(result.1, expected);
    }

    #[test]
    fn sexpr_list_parses() {
        let input = "(123 456)";
        let result = sexpr(input);

        let expected = Ok((
            "",
            DialVal::List(vec![DialVal::Int(123), DialVal::Int(456)]),
        ));

        assert_eq!(result, expected, "could not parse {}", input);
    }

    #[test]
    fn sexpr_single_list_parses() {
        let input = "( 123 )";
        let result = sexpr(input);

        let expected = Ok(("", DialVal::List(vec![DialVal::Int(123)])));

        assert_eq!(result, expected, "could not parse {}", input);
    }

    #[test]
    fn atom_test() {
        let inputs = vec!["12", "-34.5", r#""foo bar""#, ":foo", "foo"];
        let res: Vec<DialVal> = inputs.iter().map(|s| atom(s).unwrap().1).collect();

        assert_eq!(
            res,
            vec![
                DialVal::Int(12),
                DialVal::Float(-34.5),
                DialVal::Str("foo bar".into()),
                DialVal::Keyword("foo".into()),
                DialVal::Sym("foo".into())
            ]
        );

        let new_res = atom("(");
        assert!(new_res.is_err(), format!("res: {:?}", new_res));
    }

    #[test]
    fn odd_symbols_parse() {
        let inputs = vec!["+", "foo/bar", "baz-quux"];
        let res: Vec<DialVal> = inputs.iter().map(|s| atom(s).unwrap().1).collect();

        assert_eq!(
            res,
            vec![
                DialVal::Sym("+".into()),
                DialVal::Sym("foo/bar".into()),
                DialVal::Sym("baz-quux".into())
            ]
        );
    }

    #[test]
    fn int_test() {
        let inputs = vec!["-123", "4", "0"];
        let res: Vec<DialVal> = inputs.iter().map(|s| int_atom(s).unwrap().1).collect();

        assert_eq!(
            res,
            vec![DialVal::Int(-123), DialVal::Int(4), DialVal::Int(0)]
        );
    }

    #[test]
    fn float_test() {
        let inputs = vec!["0.123", "4.56", "-7.089"];
        let res: Vec<DialVal> = inputs.iter().map(|s| float_atom(s).unwrap().1).collect();
        assert_eq!(
            res,
            vec![
                DialVal::Float(0.123),
                DialVal::Float(4.56),
                DialVal::Float(-7.89)
            ]
        );
    }

    #[test]
    fn vector_test() {
        let input = "[1 2 [3 4 5.5] (+ 6 7 [8 9 10])]";
        let result = vector(input);

        let expected = DialVal::Vec(vec![
            DialVal::Int(1).into(),
            DialVal::Int(2).into(),
            DialVal::Vec(vec![
                DialVal::Int(3).into(),
                DialVal::Int(4).into(),
                DialVal::Float(5.5).into(),
            ]),
            DialVal::List(vec![
                DialVal::Sym("+".into()).into(),
                DialVal::Int(6).into(),
                DialVal::Int(7).into(),
                DialVal::Vec(vec![
                    DialVal::Int(8).into(),
                    DialVal::Int(9).into(),
                    DialVal::Int(10).into(),
                ]),
            ]),
        ]);

        assert_eq!(result, Ok(("", expected)));
    }

    #[test]
    fn bool_test() {
        let input = "true false truth falsy trueVal falseVal";
        let result: Vec<DialVal> = input
            .split(" ")
            .map(|w| parse_sexpr(w.into()).unwrap())
            .collect();

        let expected = vec![
            DialVal::Bool(true),
            DialVal::Bool(false),
            DialVal::Sym("truth".into()),
            DialVal::Sym("falsy".into()),
            DialVal::Sym("trueVal".into()),
            DialVal::Sym("falseVal".into()),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn nil_test() {
        let input = "nil nill";

        let result: Vec<DialVal> = input
            .split(" ")
            .map(|w| parse_sexpr(w.into()).unwrap())
            .collect();

        let expected = vec![DialVal::Nil, DialVal::Sym("nill".into())];

        assert_eq!(result, expected);
    }
}
