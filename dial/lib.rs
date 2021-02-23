pub mod builtin;
pub mod env;
pub mod parse;
#[macro_use]
pub mod sexpr;
pub mod eval;

use anyhow::Result;
use thiserror::Error;

pub use env::Env;
pub use eval::eval;
use parse::ParseResult;
use sexpr::DialVal;

pub type EvalResult = Result<DialVal, EvalError>;

// TODO "too many arguments" for macro
// TODO "too few arguments" for macro
#[derive(Error, Debug, PartialEq)]
pub enum EvalError {
    #[error("undefined value: {0}")]
    Undefined(String),
    #[error("TypeError: expected {0}")]
    TypeError(String),
    #[error("ArityError: wrong number of args ({0})")]
    ArityError(usize),
    #[error("InvalidArgumentError: {0}")]
    InvalidArgumentError(String),
}

pub fn read(input: String) -> ParseResult<Vec<DialVal>> {
    parse::parse_program(input)
}

pub fn print(val: EvalResult) -> String {
    todo!();
}

#[cfg(test)]
mod mal_tests {
    use super::*;
    use num::rational::Rational64;
    use pretty_assertions::assert_eq;

    macro_rules! assert_expr {
        ($expr:literal, $expected:literal) => {
            let mut env = Env::default();
            let result = eval(parse::parse_sexpr($expr).unwrap(), &mut env);

            assert_eq!($result, $expected, "Expr: {}", $expr);
        };
    }

    #[test]
    fn step_2_eval() {
        let inputs = vec![
            "1",
            "(+ 1 2 3)",
            "(- 5 4 1)",
            "(* 0.5 0.5 0.5)",
            "(/ 1 2 3)",
            "(+ 2 3)",
            "(+ 2 (* 3 4))",
        ];

        let mut env = Env::default();

        let results: Vec<EvalResult> = inputs
            .iter()
            .map(|input| eval(parse::parse_sexpr(input.to_string()).unwrap(), &mut env))
            .collect();

        assert_eq!(
            results,
            vec![
                Ok(DialVal::Int(1)),
                Ok(DialVal::Int(6)),
                Ok(DialVal::Int(0)),
                Ok(DialVal::Float(0.125)),
                Ok(DialVal::Ratio(Rational64::new(1, 6))),
                Ok(DialVal::Int(5)),
                Ok(DialVal::Int(14)),
            ]
        )
    }

    #[test]
    fn step_3_def() {
        let mut env = Env::default();

        let def_input = "(def foo 123)";

        let input_parse = read(def_input.into()).unwrap().pop().unwrap();
        let def_result = eval(input_parse, &mut env);

        assert_eq!(def_result, Ok(DialVal::Int(123)));
    }

    #[test]
    fn step_3_provided_tests() {
        let inputs = vec![
            "(def a 6)",
            "a",
            "(def b (+ a 2))",
            "(+ a b)",
            "(let (c 2) c)",
            "c",
        ];

        let mut env = Env::default();

        let results: Vec<EvalResult> = inputs
            .iter()
            .map(|input| eval(read(input.to_string()).unwrap().pop().unwrap(), &mut env))
            .collect();

        assert_eq!(
            results,
            vec![
                Ok(6.into()),
                Ok(6.into()),
                Ok(8.into()),
                Ok(14.into()),
                Ok(2.into()),
                Err(EvalError::Undefined("no such symbol c".into()))
            ]
        )
    }

    #[test]
    fn step_4_if() {
        let inputs = vec![
            r#"(if true 1 2)"#,
            r#"(if false 1 2)"#,
            "(if)",
            "(if true foo bar baz)",
        ];

        let mut env = Env::default();

        let results: Vec<EvalResult> = inputs
            .iter()
            .map(|input| eval(read(input.to_string()).unwrap().pop().unwrap(), &mut env))
            .collect();

        assert_eq!(
            results,
            vec![
                Ok(1.into()),
                Ok(2.into()),
                Err(EvalError::ArityError(1)),
                Err(EvalError::ArityError(4)),
            ]
        );
    }

    #[test]
    fn step_4_do() {
        let inputs = vec![
            "(do 1 2 3 4)",
            "(do (+ 1 2) (+ 3 4))",
            "(do (def foo 123) (+ foo 123))",
        ];

        let mut env = Env::default();

        let results: Vec<EvalResult> = inputs
            .iter()
            .map(|input| eval(read(input.to_string()).unwrap().pop().unwrap(), &mut env))
            .collect();

        assert_eq!(results, vec![Ok(4.into()), Ok(7.into()), Ok(246.into())]);
    }

    #[test]
    fn step_4_fn() {
        let inputs = vec![
            "((fn (a) a) 7)",
            "((fn (a) (+ a 1)) 10)",
            "((fn (a b) (+ a b)) 2 3)",
        ];

        let mut env = Env::default();

        let results: Vec<EvalResult> = inputs
            .iter()
            .map(|input| eval(read(input.to_string()).unwrap().pop().unwrap(), &mut env))
            .collect();

        assert_eq!(results, vec![Ok(7.into()), Ok(11.into()), Ok(5.into())]);
    }

    #[test]
    fn step_4_list_fn() {
        let inputs = vec!["(list 1 2 3)"];

        let mut env = Env::default();

        let results: Vec<EvalResult> = inputs
            .iter()
            .map(|input| eval(read(input.to_string()).unwrap().pop().unwrap(), &mut env))
            .collect();

        assert_eq!(
            results,
            vec![Ok(DialVal::List(vec![1.into(), 2.into(), 3.into()]))]
        );
    }

    #[test]
    fn step_4_is_list_fn() {
        let inputs = vec!["(list? (list 1 2 3))", "(list? 1)"];

        let mut env = Env::default();

        let results: Vec<EvalResult> = inputs
            .iter()
            .map(|input| eval(read(input.to_string()).unwrap().pop().unwrap(), &mut env))
            .collect();

        assert_eq!(results, vec![Ok(true.into()), Ok(false.into())]);
    }

    #[test]
    fn step_4_is_empty_fn() {
        let inputs = vec!["(empty? (list))", "(empty? (list 1 2 3))"];

        let mut env = Env::default();

        let results: Vec<EvalResult> = inputs
            .iter()
            .map(|input| eval(read(input.to_string()).unwrap().pop().unwrap(), &mut env))
            .collect();

        assert_eq!(results, vec![Ok(true.into()), Ok(false.into())]);
    }

    #[test]
    fn step_4_count_fn() {
        let inputs = vec!["(count (list 1 2 3))", "(count (list))"];

        let mut env = Env::default();

        let results: Vec<EvalResult> = inputs
            .iter()
            .map(|input| eval(read(input.to_string()).unwrap().pop().unwrap(), &mut env))
            .collect();

        assert_eq!(results, vec![Ok(3.into()), Ok(0.into())]);
    }

    #[test]
    fn step_4_closure() {
        let inputs = vec![
            "( (fn (a b) (+ b a)) 3 4)",
            "( (fn () 4) )",
            "( (fn (f x) (f x)) (fn (a) (+ 1 a)) 7)",
            "( ( (fn (a) (fn (b) (+ a b))) 5) 7)",
            r#"(do
                (def gen-plus5 (fn () (fn (b) (+ 5 b))))
                (def plus5 (gen-plus5))
                (plus5 7))"#,
            r#"(do
                (def gen-plusX (fn (x) (fn (b) (+ x b))))
                (def plus7 (gen-plusX 7))
                (plus7 8))"#,
            r#"(do
                (def gen-plusX (fn (x) (fn (b) (+ x b))))
                (def plus7 (gen-plusX 7))
                (plus7 8)
                (plus7 8))"#,
        ];

        let mut env = Env::default();

        let expected = vec![
            Ok(DialVal::Int(7)),
            Ok(DialVal::Int(4)),
            Ok(DialVal::Int(8)),
            Ok(DialVal::Int(12)),
            Ok(DialVal::Int(12)),
            Ok(DialVal::Int(15)),
            Ok(DialVal::Int(15)),
        ];

        let results = map_results(inputs, &mut env);

        assert_eq!(results, expected);
    }

    #[test]
    #[ignore]
    // this test takes a long time to run!
    fn step_5_tco() {
        let inputs = vec![
            "(do (def sum2 (fn (n acc) (if (= n 0) acc (sum2 (- n 1) (+ n acc))))) (sum2 10 0))",
            "(do (def sum2 (fn (n acc) (if (= n 0) acc (sum2 (- n 1) (+ n acc))))) (sum2 10000 0))",
            "(do (def foo (fn (n) (if (= n 0) 0 (bar (- n 1))))) (def bar (fn (n) (if (= n 0) 0 (foo (- n 1))))) (foo 10000))"
        ];

        let mut env = Env::default();

        let expected = vec![Ok(55.into()), Ok(50005000.into()), Ok(0.into())];

        let results: Vec<EvalResult> = inputs
            .iter()
            .map(|input| eval(read(input.to_string()).unwrap().pop().unwrap(), &mut env))
            .collect();

        assert_eq!(results, expected);
    }
}

fn map_results(inputs: Vec<&str>, env: &mut Env) -> Vec<EvalResult> {
    inputs
        .iter()
        .map(|input| eval(read(input.to_string()).unwrap().pop().unwrap(), env))
        .collect()
}
