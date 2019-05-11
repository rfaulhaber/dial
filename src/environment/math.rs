#![allow(clippy::suspicious_arithmetic_impl)]
use crate::interpreter::EvalResult;
use crate::parser::{Atom, Expr};
use std::iter::{Product, Sum};
use std::ops::{Add, Div, Mul, Sub};

pub fn add(args: &[Expr]) -> EvalResult {
    if args.is_empty() {
        Ok(0.into())
    } else {
        unimplemented!();
    }
}

pub fn sub(args: &[Expr]) -> EvalResult {
    unimplemented!();
}

pub fn mul(args: &[Expr]) -> EvalResult {
    unimplemented!();
}

pub fn div(args: &[Expr]) -> EvalResult {
    unimplemented!();
}

fn int_from_expr(expr: Expr) -> Result<i64, &'static str> {
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

impl Sum for Atom {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Atom::Integer(0), |sum, val| sum + val)
    }
}

impl Product for Atom {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Atom::Integer(0), |prod, val| prod * val)
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

#[cfg(test)]
mod test {
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
    fn sum_should_sum_all_values() {
        let vals = vec![Atom::Integer(1), Atom::Integer(2)];

        let result: Atom = vals.into_iter().sum();

        assert_eq!(result, Atom::Integer(3));
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
