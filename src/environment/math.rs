#![allow(clippy::suspicious_arithmetic_impl)]
use crate::interpreter::EvalResult;
use crate::parser::{Atom, Expr};

use std::cmp::{Ordering, PartialOrd};
use std::ops::{Add, Div, Mul, Sub};

impl Expr {
    fn as_inner_atom(&self) -> Atom {
        match self {
            Expr::Atom(a) => a.clone(),
            _ => panic!("not an atom"),
        }
    }
}

// TODO specify arity?
pub fn add(args: &[Expr]) -> EvalResult {
    if args.is_empty() {
        Ok(0.into())
    } else {
        let sum = args
            .iter()
            .fold(Atom::Integer(0), |sum, val| sum + val.as_inner_atom());

        Ok(sum.into())
    }
}

pub fn sub(args: &[Expr]) -> EvalResult {
    match args.len() {
        0 => Err("not enough arguments".to_string()),
        1 => {
            let result = Atom::from(0) - args[0].as_inner_atom();
            Ok(result.into())
        }
        _ => {
            let first = &args[0];

            let diff = args[1..].iter().fold(first.as_inner_atom(), |diff, val| {
                diff - val.as_inner_atom()
            });

            Ok(diff.into())
        }
    }
}

pub fn mul(args: &[Expr]) -> EvalResult {
    match args.len() {
        0 => Ok(1.into()),
        1 => Ok(args[0].clone()),
        _ => {
            let first = args[0].as_inner_atom().clone();
            let product = args[1..]
                .iter()
                .fold(first, |sum, val| sum * val.as_inner_atom());

            Ok(product.into())
        }
    }
}

pub fn div(args: &[Expr]) -> EvalResult {
    match args.len() {
        0 => Err("not enough arguments".to_string()),
        1 => {
            let result = Atom::from(1) / args[0].as_inner_atom();
            Ok(result.into())
        }
        _ => {
            let first = &args[0];

            let diff = args[1..].iter().fold(first.as_inner_atom(), |diff, val| {
                diff / val.as_inner_atom()
            });

            Ok(diff.into())
        }
    }
}

pub fn gt(args: &[Expr]) -> EvalResult {
    // match &args[0].as_inner_atom().partial_cmp(&args[1].as_inner_atom()) {
    //     Some(Ordering::Equal) | Some(Ordering::Less) => Ok(false.into()),
    //     Some(Ordering::Greater) => Ok(true.into()),
    //     None => Err("cannot compare these two types".to_string()),
    // }

    match args.len() {
        0 => Err("wrong number of args".to_string()),
        1 => Ok(true.into()),
        _ => {
            let atom_vec = args.iter().map(|expr| expr.as_inner_atom()).collect();
            let iter = PairIter::new(atom_vec);

            let mut ret = true;
            // TODO rewrite with closures
            for (left, right) in iter {
                match right {
                    Some(val) => ret = ret && (left.partial_cmp(&val) == Some(Ordering::Greater)),
                    None => ret = ret && true,
                }
            }

            Ok(ret.into())
        }
    }

}

pub fn ge(args: &[Expr]) -> EvalResult {
    match &args[0]
        .as_inner_atom()
        .partial_cmp(&args[1].as_inner_atom())
    {
        Some(Ordering::Equal) | Some(Ordering::Greater) => Ok(true.into()),
        Some(Ordering::Less) => Ok(false.into()),
        None => Err("cannot compare these two types".to_string()),
    }
}

pub fn lt(args: &[Expr]) -> EvalResult {
    match &args[0]
        .as_inner_atom()
        .partial_cmp(&args[1].as_inner_atom())
    {
        Some(Ordering::Equal) | Some(Ordering::Greater) => Ok(true.into()),
        Some(Ordering::Less) => Ok(false.into()),
        None => Err("cannot compare these two types".to_string()),
    }
}

pub fn le(args: &[Expr]) -> EvalResult {
    unimplemented!();
}

pub fn eq(args: &[Expr]) -> EvalResult {
    unimplemented!();
}

impl Add for Atom {
    type Output = Atom;

    fn add(self, other: Atom) -> Self::Output {
        match (self, other) {
            (Atom::Integer(a), Atom::Integer(b)) => Atom::Integer(a + b),
            (Atom::Integer(a), Atom::Float(b)) => Atom::Float(a as f64 + b),
            (Atom::Float(a), Atom::Integer(b)) => Atom::Float(a + b as f64),
            (Atom::Float(a), Atom::Float(b)) => Atom::Float(a + b),
            (Atom::Float(a), Atom::Ratio { num, den }) => {
                Atom::Float(a + (num as f64 / den as f64))
            }
            (Atom::Ratio { num, den }, Atom::Float(a)) => {
                Atom::Float(a + (num as f64 / den as f64))
            }
            (Atom::Integer(a), Atom::Ratio { num, den }) => new_ratio(a * den + num, den),
            (Atom::Ratio { num, den }, Atom::Integer(a)) => new_ratio(a * den + num, den),
            (
                Atom::Ratio {
                    num: lnum,
                    den: lden,
                },
                Atom::Ratio { num, den },
            ) => {
                let new_dem = (lden * den) / gcd(lden, den);
                let new_num = (lnum) * (new_dem / lden) + num * (new_dem / den);

                new_ratio(new_num, new_dem)
            }
            _ => panic!("addition not defiend for this type"),
        }
    }
}

impl Sub for Atom {
    type Output = Atom;

    fn sub(self, other: Atom) -> Self::Output {
        match (self, other) {
            (Atom::Integer(a), Atom::Integer(b)) => Atom::Integer(a - b),
            (Atom::Integer(a), Atom::Float(b)) => Atom::Float(a as f64 - b),
            (Atom::Float(a), Atom::Integer(b)) => Atom::Float(a - b as f64),
            (Atom::Float(a), Atom::Float(b)) => Atom::Float(a - b),
            (Atom::Float(a), Atom::Ratio { num, den }) => {
                Atom::Float(a - (num as f64 / den as f64))
            }
            (Atom::Ratio { num, den }, Atom::Float(a)) => {
                Atom::Float((num as f64 / den as f64) - a)
            }
            (Atom::Integer(a), Atom::Ratio { num, den }) => new_ratio(a * den - num, den),
            (Atom::Ratio { num, den }, Atom::Integer(a)) => new_ratio(num - (a * den), den),
            (
                Atom::Ratio {
                    num: lnum,
                    den: lden,
                },
                Atom::Ratio { num, den },
            ) => {
                let new_dem = (lden * den) / gcd(lden, den);
                let new_num = (lnum) * (new_dem / lden) - num * (new_dem / den);

                new_ratio(new_num, new_dem)
            }
            _ => panic!("subtraction not defiend for this type"),
        }
    }
}

impl Mul for Atom {
    type Output = Atom;

    fn mul(self, other: Atom) -> Self::Output {
        match (self, other) {
            (Atom::Integer(a), Atom::Integer(b)) => Atom::Integer(a * b),
            (Atom::Integer(a), Atom::Float(b)) => Atom::Float(a as f64 * b),
            (Atom::Float(a), Atom::Integer(b)) => Atom::Float(a * b as f64),
            (Atom::Float(a), Atom::Float(b)) => Atom::Float(a * b),
            (Atom::Float(a), Atom::Ratio { num, den }) => {
                Atom::Float(a * (num as f64 / den as f64))
            }
            (Atom::Ratio { num, den }, Atom::Float(a)) => {
                Atom::Float((num as f64 / den as f64) * a)
            }
            (Atom::Integer(a), Atom::Ratio { num, den }) => new_ratio(a * num, den),
            (Atom::Ratio { num, den }, Atom::Integer(a)) => new_ratio(num * a, den),
            (
                Atom::Ratio {
                    num: lnum,
                    den: lden,
                },
                Atom::Ratio { num, den },
            ) => new_ratio(lnum * num, lden * den),
            _ => panic!("multiplication not defiend for this type"),
        }
    }
}

impl Div for Atom {
    type Output = Atom;

    fn div(self, other: Atom) -> Self::Output {
        match (self, other) {
            (Atom::Integer(a), Atom::Integer(b)) => {
                let (num, den) = reduce_ratio(a, b);
                Atom::Ratio { num, den }
            }
            (Atom::Integer(a), Atom::Float(b)) => Atom::Float(a as f64 / b),
            (Atom::Float(a), Atom::Integer(b)) => Atom::Float(a / b as f64),
            (Atom::Float(a), Atom::Float(b)) => Atom::Float(a / b),
            (Atom::Integer(a), Atom::Ratio { num, den }) => new_ratio(a * den, num),
            (Atom::Float(a), Atom::Ratio { num, den }) => {
                Atom::Float(a / (num as f64 / den as f64))
            }
            (Atom::Ratio { num, den }, Atom::Float(a)) => {
                Atom::Float((num as f64 / den as f64) / a)
            }
            (Atom::Ratio { num, den }, Atom::Integer(a)) => new_ratio(num, den * a),
            (
                Atom::Ratio {
                    num: lnum,
                    den: lden,
                },
                Atom::Ratio { num, den },
            ) => new_ratio(lnum * den, lden * num),
            _ => panic!("division not defined for this type"),
        }
    }
}

impl PartialOrd for Atom {
    fn partial_cmp(&self, other: &Atom) -> Option<Ordering> {
        // TODO fill out types
        match (self, other) {
            (Atom::Integer(a), Atom::Integer(b)) => Some(a.cmp(b)),
            (Atom::Float(a), Atom::Float(b)) => a.partial_cmp(b),
            (
                Atom::Ratio {
                    num: lnum,
                    den: lden,
                },
                Atom::Ratio { num, den },
            ) => {
                let left = (*lnum as f64) / (*lden as f64);
                let right = (*num as f64) / (*den as f64);

                left.partial_cmp(&right)
            }
            _ => None,
        }
    }
}

fn new_ratio(num: i64, den: i64) -> Atom {
    let (top, bottom) = reduce_ratio(num, den);

    if bottom == 1 {
        return Atom::Integer(top);
    }

    if bottom < 0 || top < 0 {
        return Atom::Ratio {
            num: -top,
            den: -bottom,
        };
    }

    Atom::Ratio {
        num: top,
        den: bottom,
    }
}

fn reduce_ratio(num: i64, den: i64) -> (i64, i64) {
    let common = gcd(num, den);

    (num / common, den / common)
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

struct PairIter<T> {
    elements: Vec<T>,
}

impl<T> PairIter<T> {
    fn new(elements: Vec<T>) -> PairIter<T> {
        PairIter { elements }
    }
}

impl<T> Iterator for PairIter<T>
where
    T: std::clone::Clone,
{
    type Item = (T, Option<T>);

    fn next(&mut self) -> Option<Self::Item> {
        let mut first_two = self.elements.iter().take(2);

        let result = match (first_two.next(), first_two.next()) {
            (Some(first), Some(second)) => Some((first.clone(), Some(second.clone()))),
            (Some(first), None) => Some((first.clone(), None)),
            _ => None,
        };

        // TODO fix index error
        self.elements = self.elements[2..].to_vec();

        result
    }
}

#[cfg(test)]
mod math_fn_tests {
    use super::*;

    #[test]
    fn add_two_numbers() {
        let left = Expr::Atom(Atom::Integer(2));
        let right = Expr::Atom(Atom::Integer(3));

        let result = add(&[left, right]);

        assert_eq!(result, Ok(Expr::Atom(Atom::Integer(5))));
    }

    #[test]
    fn add_different_types() {
        let vals: &[Expr] = &[2.into(), 3.4.into(), 4.into(), 5.5.into()];
        let result = add(vals);

        assert_eq!(result, Ok(Expr::Atom(Atom::Float(14.9))));
    }

    #[test]
    fn sub_two_numbers() {
        let left = Expr::Atom(Atom::Integer(3));
        let right = Expr::Atom(Atom::Integer(2));

        let result = sub(&[left, right]);

        assert_eq!(result, Ok(Expr::Atom(Atom::Integer(1))));
    }

    #[test]
    fn sub_one_number() {
        let number = Expr::Atom(Atom::Integer(2));
        let result = sub(&[number]);

        assert_eq!(result, Ok(Expr::Atom(Atom::Integer(-2))));
    }

    #[test]
    fn sub_different_types() {
        let vals: &[Expr] = &[2.into(), 3.5.into()];
        let result = sub(vals);

        assert_eq!(result, Ok(Expr::Atom(Atom::Float(-1.5))));
    }

    #[test]
    fn mul_one_number() {
        let vals: &[Expr] = &[5.into()];
        let result = mul(vals);

        assert_eq!(result, Ok(5.into()));
    }

    #[test]
    fn mul_two_numbers() {
        let vals: &[Expr] = &[5.into(), 10.into()];
        let result = mul(vals);

        assert_eq!(result, Ok(50.into()));
    }

    #[test]
    fn mul_many_numbers() {
        let vals: &[Expr] = &[5.into(), 10.into(), 2.5.into()];
        let result = mul(vals);

        assert_eq!(result, Ok(125.0.into()));
    }

    #[test]
    fn div_one_number() {
        let vals: &[Expr] = &[3.into()];
        let result = div(vals);

        assert_eq!(result, Ok(Expr::Atom(Atom::Ratio { num: 1, den: 3 })));
    }

    #[test]
    fn div_two_numbers() {
        let vals: &[Expr] = &[3.into(), 6.into()];
        let result = div(vals);

        assert_eq!(result, Ok(Expr::Atom(Atom::Ratio { num: 1, den: 2 })));
    }

    #[test]
    fn div_many_numbers() {
        let vals: &[Expr] = &[1.into(), 2.into(), 3.into(), 4.into()];
        let result = div(vals);

        assert_eq!(result, Ok(Expr::Atom(Atom::Ratio { num: 1, den: 24 })));
    }

    #[test]
    fn div_floats() {
        let vals: &[Expr] = &[1.0.into(), 2.0.into()];
        let result = div(vals);

        assert_eq!(result, Ok(Expr::Atom(Atom::Float(0.5))));
    }
}

#[cfg(test)]
mod ops_tests {
    use super::*;

    #[test]
    fn addition_int_defined() {
        let left = Atom::Integer(1);
        let right = Atom::Integer(2);

        assert_eq!(Atom::Integer(3), left + right);
    }

    #[test]
    fn addition_mixed_type_defined() {
        assert_eq!(Atom::Float(6.5), Atom::Integer(1) + Atom::Float(5.5));
    }

    #[test]
    fn subtraction_int_defined() {
        let left = Atom::Integer(1);
        let right = Atom::Integer(2);

        assert_eq!(Atom::Integer(1), right - left);
    }

    #[test]
    fn subtraction_mixed_type_defined() {
        assert_eq!(Atom::Float(4.5), Atom::Float(5.5) - Atom::Integer(1));
    }

    #[test]
    fn int_div_by_ratio() {
        let int = Atom::Integer(2);
        let ratio = Atom::Ratio { num: 1, den: 3 }; // 1/3

        let result = int / ratio;

        assert_eq!(result, Atom::Integer(6));
    }

    #[test]
    fn ratio_div_by_int() {
        let int = Atom::Integer(2);
        let ratio = Atom::Ratio { num: 1, den: 3 }; // 1/3

        let result = ratio / int;

        assert_eq!(result, Atom::Ratio { num: 1, den: 6 });
    }

    #[test]
    fn float_div_by_ratio() {
        let float = Atom::Float(2.0);
        let ratio = Atom::Ratio { num: 1, den: 2 };

        let result = float / ratio;

        assert_eq!(result, Atom::Float(4.0));
    }

    #[test]
    fn ratio_div_by_float() {
        let float = Atom::Float(2.0);
        let ratio = Atom::Ratio { num: 1, den: 2 };

        let result = ratio / float;

        assert_eq!(result, Atom::Float(0.25));
    }

    #[test]
    fn ratio_div_by_ratio() {
        let left = Atom::Ratio { num: 1, den: 4 };
        let right = Atom::Ratio { num: 1, den: 2 };

        let result = left / right;

        assert_eq!(result, Atom::Ratio { num: 1, den: 2 });
    }

    #[test]
    fn int_mul_by_ratio() {
        let int = Atom::Integer(2);
        let ratio = Atom::Ratio { num: 1, den: 3 }; // 1/3

        let result = int * ratio;

        assert_eq!(result, Atom::Ratio { num: 2, den: 3 });
    }

    #[test]
    fn ratio_mul_by_int() {
        let int = Atom::Integer(2);
        let ratio = Atom::Ratio { num: 1, den: 3 }; // 1/3

        let result = ratio * int;

        assert_eq!(result, Atom::Ratio { num: 2, den: 3 });
    }

    #[test]
    fn float_mul_by_ratio() {
        let float = Atom::Float(2.0);
        let ratio = Atom::Ratio { num: 1, den: 2 };

        let result = float * ratio;

        assert_eq!(result, Atom::Float(1.0));
    }

    #[test]
    fn ratio_mul_by_float() {
        let float = Atom::Float(2.0);
        let ratio = Atom::Ratio { num: 1, den: 2 };

        let result = ratio * float;

        assert_eq!(result, Atom::Float(1.0));
    }

    #[test]
    fn ratio_mul_by_ratio() {
        let left = Atom::Ratio { num: 1, den: 4 };
        let right = Atom::Ratio { num: 1, den: 2 };

        let result = left * right;

        assert_eq!(result, Atom::Ratio { num: 1, den: 8 });
    }

    #[test]
    fn int_add_by_ratio() {
        let int = Atom::Integer(2);
        let ratio = Atom::Ratio { num: 1, den: 3 }; // 1/3

        let result = int * ratio;

        assert_eq!(result, Atom::Ratio { num: 2, den: 3 });
    }

    #[test]
    fn ratio_add_by_int() {
        let int = Atom::Integer(2);
        let ratio = Atom::Ratio { num: 1, den: 3 }; // 1/3

        let result = ratio + int;

        assert_eq!(result, Atom::Ratio { num: 7, den: 3 });
    }

    #[test]
    fn float_add_by_ratio() {
        let float = Atom::Float(2.0);
        let ratio = Atom::Ratio { num: 1, den: 2 };

        let result = float + ratio;

        assert_eq!(result, Atom::Float(2.5));
    }

    #[test]
    fn ratio_add_by_float() {
        let float = Atom::Float(2.0);
        let ratio = Atom::Ratio { num: 1, den: 2 };

        let result = ratio + float;

        assert_eq!(result, Atom::Float(2.5));
    }

    #[test]
    fn ratio_add_by_ratio() {
        let left = Atom::Ratio { num: 1, den: 4 };
        let right = Atom::Ratio { num: 1, den: 2 };

        let result = left + right;

        assert_eq!(result, Atom::Ratio { num: 3, den: 4 });
    }

    #[test]
    fn int_sub_by_ratio() {
        let int = Atom::Integer(2);
        let ratio = Atom::Ratio { num: 1, den: 3 }; // 1/3

        let result = int * ratio;

        assert_eq!(result, Atom::Ratio { num: 2, den: 3 });
    }

    #[test]
    fn ratio_sub_by_int() {
        let int = Atom::Integer(2);
        let ratio = Atom::Ratio { num: 1, den: 2 }; // 1/3

        let result = ratio - int;

        assert_eq!(result, Atom::Ratio { num: -3, den: 2 });
    }

    #[test]
    fn float_sub_by_ratio() {
        let float = Atom::Float(2.0);
        let ratio = Atom::Ratio { num: 1, den: 2 };

        let result = float - ratio;

        assert_eq!(result, Atom::Float(1.5));
    }

    #[test]
    fn ratio_sub_by_float() {
        let float = Atom::Float(2.0);
        let ratio = Atom::Ratio { num: 1, den: 2 };

        let result = ratio - float;

        assert_eq!(result, Atom::Float(-1.5));
    }

    #[test]
    fn ratio_sub_by_ratio() {
        let left = Atom::Ratio { num: 1, den: 4 };
        let right = Atom::Ratio { num: 1, den: 2 };

        let result = left - right;

        assert_eq!(result, Atom::Ratio { num: -1, den: 4 });
    }

}
