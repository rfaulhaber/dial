#![allow(clippy::suspicious_arithmetic_impl)]

use super::env::Env;
use super::parser::Rule;
use pest::iterators::Pair;
use std::convert::From;
use std::fmt;
use std::iter::{Product, Sum};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, PartialEq, Clone)]
pub enum DialValue {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    // TODO extract ratio math into separate mod or crate
    Ratio { num: i64, den: i64 }, // note: should this just be a tuple?
    // Func(fn(Vec<DialValue>)),                // TODO define FuncRef
    Nil,
}

impl fmt::Display for DialValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DialValue::Integer(int) => write!(f, "{}", int),
            DialValue::Float(float) => write!(f, "{}", float),
            DialValue::String(string) => write!(f, "{}", string),
            DialValue::Boolean(b) => {
                if *b {
                    write!(f, "true")
                } else {
                    write!(f, "false")
                }
            }
            DialValue::Ratio { num, den } => write!(f, "{}/{}", num, den),
            // DialValue::Func(fr) => write!(f, "{}", fr),
            DialValue::Nil => write!(f, "nil"),
        }
    }
}

impl Add for DialValue {
    type Output = DialValue;

    fn add(self, other: DialValue) -> Self::Output {
        match (self, other) {
            (DialValue::Integer(a), DialValue::Integer(b)) => DialValue::Integer(a + b),
            (DialValue::Integer(a), DialValue::Float(b)) => DialValue::Float(a as f64 + b),
            (DialValue::Float(a), DialValue::Integer(b)) => DialValue::Float(a + b as f64),
            (DialValue::Float(a), DialValue::Float(b)) => DialValue::Float(a + b),
            (DialValue::Float(a), DialValue::Ratio { num, den }) => {
                DialValue::Float(a + (num as f64 / den as f64))
            }
            (DialValue::Ratio { num, den }, DialValue::Float(a)) => {
                DialValue::Float(a + (num as f64 / den as f64))
            }
            (DialValue::Integer(a), DialValue::Ratio { num, den }) => new_ratio(a * den + num, den),
            (DialValue::Ratio { num, den }, DialValue::Integer(a)) => new_ratio(a * den + num, den),
            (
                DialValue::Ratio {
                    num: lnum,
                    den: lden,
                },
                DialValue::Ratio { num, den },
            ) => {
                let new_dem = (lden * den) / gcd(lden, den);
                let new_num = (lnum) * (new_dem / lden) + num * (new_dem / den);

                new_ratio(new_num, new_dem)
            }
            _ => panic!("addition not defiend for this type"),
        }
    }
}

impl Sub for DialValue {
    type Output = DialValue;

    fn sub(self, other: DialValue) -> Self::Output {
        match (self, other) {
            (DialValue::Integer(a), DialValue::Integer(b)) => DialValue::Integer(a - b),
            (DialValue::Integer(a), DialValue::Float(b)) => DialValue::Float(a as f64 - b),
            (DialValue::Float(a), DialValue::Integer(b)) => DialValue::Float(a - b as f64),
            (DialValue::Float(a), DialValue::Float(b)) => DialValue::Float(a - b),
            (DialValue::Float(a), DialValue::Ratio { num, den }) => {
                DialValue::Float(a - (num as f64 / den as f64))
            }
            (DialValue::Ratio { num, den }, DialValue::Float(a)) => {
                DialValue::Float((num as f64 / den as f64) - a)
            }
            (DialValue::Integer(a), DialValue::Ratio { num, den }) => new_ratio(a * den - num, den),
            (DialValue::Ratio { num, den }, DialValue::Integer(a)) => {
                new_ratio(num - (a * den), den)
            }
            (
                DialValue::Ratio {
                    num: lnum,
                    den: lden,
                },
                DialValue::Ratio { num, den },
            ) => {
                let new_dem = (lden * den) / gcd(lden, den);
                let new_num = (lnum) * (new_dem / lden) - num * (new_dem / den);

                new_ratio(new_num, new_dem)
            }
            _ => panic!("subtraction not defiend for this type"),
        }
    }
}

impl Mul for DialValue {
    type Output = DialValue;

    fn mul(self, other: DialValue) -> Self::Output {
        match (self, other) {
            (DialValue::Integer(a), DialValue::Integer(b)) => DialValue::Integer(a * b),
            (DialValue::Integer(a), DialValue::Float(b)) => DialValue::Float(a as f64 * b),
            (DialValue::Float(a), DialValue::Integer(b)) => DialValue::Float(a * b as f64),
            (DialValue::Float(a), DialValue::Float(b)) => DialValue::Float(a * b),
            (DialValue::Float(a), DialValue::Ratio { num, den }) => {
                DialValue::Float(a * (num as f64 / den as f64))
            }
            (DialValue::Ratio { num, den }, DialValue::Float(a)) => {
                DialValue::Float((num as f64 / den as f64) * a)
            }
            (DialValue::Integer(a), DialValue::Ratio { num, den }) => new_ratio(a * num, den),
            (DialValue::Ratio { num, den }, DialValue::Integer(a)) => new_ratio(num * a, den),
            (
                DialValue::Ratio {
                    num: lnum,
                    den: lden,
                },
                DialValue::Ratio { num, den },
            ) => new_ratio(lnum * num, lden * den),
            _ => panic!("multiplication not defiend for this type"),
        }
    }
}

impl Div for DialValue {
    type Output = DialValue;

    fn div(self, other: DialValue) -> Self::Output {
        match (self, other) {
            (DialValue::Integer(a), DialValue::Integer(b)) => {
                let (num, den) = reduce_ratio(a, b);
                DialValue::Ratio { num, den }
            }
            (DialValue::Integer(a), DialValue::Float(b)) => DialValue::Float(a as f64 / b),
            (DialValue::Float(a), DialValue::Integer(b)) => DialValue::Float(a / b as f64),
            (DialValue::Float(a), DialValue::Float(b)) => DialValue::Float(a / b),
            (DialValue::Integer(a), DialValue::Ratio { num, den }) => new_ratio(a * den, num),
            (DialValue::Float(a), DialValue::Ratio { num, den }) => {
                DialValue::Float(a / (num as f64 / den as f64))
            }
            (DialValue::Ratio { num, den }, DialValue::Float(a)) => {
                DialValue::Float((num as f64 / den as f64) / a)
            }
            (DialValue::Ratio { num, den }, DialValue::Integer(a)) => new_ratio(num, den * a),
            (
                DialValue::Ratio {
                    num: lnum,
                    den: lden,
                },
                DialValue::Ratio { num, den },
            ) => new_ratio(lnum * den, lden * num),
            _ => panic!("division not defined for this type"),
        }
    }
}

impl Sum for DialValue {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(DialValue::Integer(0), |sum, val| sum + val)
    }
}

impl Product for DialValue {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(DialValue::Integer(0), |prod, val| prod * val)
    }
}

impl From<i64> for DialValue {
    fn from(item: i64) -> Self {
        DialValue::Integer(item)
    }
}

impl From<f64> for DialValue {
    fn from(item: f64) -> Self {
        DialValue::Float(item)
    }
}

impl From<String> for DialValue {
    fn from(item: String) -> Self {
        DialValue::String(item)
    }
}

impl From<bool> for DialValue {
    fn from(item: bool) -> Self {
        DialValue::Boolean(item)
    }
}

fn new_ratio(num: i64, den: i64) -> DialValue {
    let (top, bottom) = reduce_ratio(num, den);

    if bottom == 1 {
        return DialValue::Integer(top);
    }

    if bottom < 0 || top < 0 {
        return DialValue::Ratio {
            num: -top,
            den: -bottom,
        };
    }

    DialValue::Ratio {
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
        let left = DialValue::Integer(1);
        let right = DialValue::Integer(2);

        assert_eq!(DialValue::Integer(3), left + right);
    }

    #[test]
    fn addition_mixed_type_defined() {
        assert_eq!(
            DialValue::Float(6.5),
            DialValue::Integer(1) + DialValue::Float(5.5)
        );
    }

    #[test]
    fn subtraction_int_defined() {
        let left = DialValue::Integer(1);
        let right = DialValue::Integer(2);

        assert_eq!(DialValue::Integer(1), right - left);
    }

    #[test]
    fn subtraction_mixed_type_defined() {
        assert_eq!(
            DialValue::Float(4.5),
            DialValue::Float(5.5) - DialValue::Integer(1)
        );
    }

    #[test]
    fn sum_should_sum_all_values() {
        let vals = vec![DialValue::Integer(1), DialValue::Integer(2)];

        let result: DialValue = vals.into_iter().sum();

        assert_eq!(result, DialValue::Integer(3));
    }

    #[test]
    fn int_div_by_ratio() {
        let int = DialValue::Integer(2);
        let ratio = DialValue::Ratio { num: 1, den: 3 }; // 1/3

        let result = int / ratio;

        assert_eq!(result, DialValue::Integer(6));
    }

    #[test]
    fn ratio_div_by_int() {
        let int = DialValue::Integer(2);
        let ratio = DialValue::Ratio { num: 1, den: 3 }; // 1/3

        let result = ratio / int;

        assert_eq!(result, DialValue::Ratio { num: 1, den: 6 });
    }

    #[test]
    fn float_div_by_ratio() {
        let float = DialValue::Float(2.0);
        let ratio = DialValue::Ratio { num: 1, den: 2 };

        let result = float / ratio;

        assert_eq!(result, DialValue::Float(4.0));
    }

    #[test]
    fn ratio_div_by_float() {
        let float = DialValue::Float(2.0);
        let ratio = DialValue::Ratio { num: 1, den: 2 };

        let result = ratio / float;

        assert_eq!(result, DialValue::Float(0.25));
    }

    #[test]
    fn ratio_div_by_ratio() {
        let left = DialValue::Ratio { num: 1, den: 4 };
        let right = DialValue::Ratio { num: 1, den: 2 };

        let result = left / right;

        assert_eq!(result, DialValue::Ratio { num: 1, den: 2 });
    }

    #[test]
    fn int_mul_by_ratio() {
        let int = DialValue::Integer(2);
        let ratio = DialValue::Ratio { num: 1, den: 3 }; // 1/3

        let result = int * ratio;

        assert_eq!(result, DialValue::Ratio { num: 2, den: 3 });
    }

    #[test]
    fn ratio_mul_by_int() {
        let int = DialValue::Integer(2);
        let ratio = DialValue::Ratio { num: 1, den: 3 }; // 1/3

        let result = ratio * int;

        assert_eq!(result, DialValue::Ratio { num: 2, den: 3 });
    }

    #[test]
    fn float_mul_by_ratio() {
        let float = DialValue::Float(2.0);
        let ratio = DialValue::Ratio { num: 1, den: 2 };

        let result = float * ratio;

        assert_eq!(result, DialValue::Float(1.0));
    }

    #[test]
    fn ratio_mul_by_float() {
        let float = DialValue::Float(2.0);
        let ratio = DialValue::Ratio { num: 1, den: 2 };

        let result = ratio * float;

        assert_eq!(result, DialValue::Float(1.0));
    }

    #[test]
    fn ratio_mul_by_ratio() {
        let left = DialValue::Ratio { num: 1, den: 4 };
        let right = DialValue::Ratio { num: 1, den: 2 };

        let result = left * right;

        assert_eq!(result, DialValue::Ratio { num: 1, den: 8 });
    }

    #[test]
    fn int_add_by_ratio() {
        let int = DialValue::Integer(2);
        let ratio = DialValue::Ratio { num: 1, den: 3 }; // 1/3

        let result = int * ratio;

        assert_eq!(result, DialValue::Ratio { num: 2, den: 3 });
    }

    #[test]
    fn ratio_add_by_int() {
        let int = DialValue::Integer(2);
        let ratio = DialValue::Ratio { num: 1, den: 3 }; // 1/3

        let result = ratio + int;

        assert_eq!(result, DialValue::Ratio { num: 7, den: 3 });
    }

    #[test]
    fn float_add_by_ratio() {
        let float = DialValue::Float(2.0);
        let ratio = DialValue::Ratio { num: 1, den: 2 };

        let result = float + ratio;

        assert_eq!(result, DialValue::Float(2.5));
    }

    #[test]
    fn ratio_add_by_float() {
        let float = DialValue::Float(2.0);
        let ratio = DialValue::Ratio { num: 1, den: 2 };

        let result = ratio + float;

        assert_eq!(result, DialValue::Float(2.5));
    }

    #[test]
    fn ratio_add_by_ratio() {
        let left = DialValue::Ratio { num: 1, den: 4 };
        let right = DialValue::Ratio { num: 1, den: 2 };

        let result = left + right;

        assert_eq!(result, DialValue::Ratio { num: 3, den: 4 });
    }

    #[test]
    fn int_sub_by_ratio() {
        let int = DialValue::Integer(2);
        let ratio = DialValue::Ratio { num: 1, den: 3 }; // 1/3

        let result = int * ratio;

        assert_eq!(result, DialValue::Ratio { num: 2, den: 3 });
    }

    #[test]
    fn ratio_sub_by_int() {
        let int = DialValue::Integer(2);
        let ratio = DialValue::Ratio { num: 1, den: 2 }; // 1/3

        let result = ratio - int;

        assert_eq!(result, DialValue::Ratio { num: -3, den: 2 });
    }

    #[test]
    fn float_sub_by_ratio() {
        let float = DialValue::Float(2.0);
        let ratio = DialValue::Ratio { num: 1, den: 2 };

        let result = float - ratio;

        assert_eq!(result, DialValue::Float(1.5));
    }

    #[test]
    fn ratio_sub_by_float() {
        let float = DialValue::Float(2.0);
        let ratio = DialValue::Ratio { num: 1, den: 2 };

        let result = ratio - float;

        assert_eq!(result, DialValue::Float(-1.5));
    }

    #[test]
    fn ratio_sub_by_ratio() {
        let left = DialValue::Ratio { num: 1, den: 4 };
        let right = DialValue::Ratio { num: 1, den: 2 };

        let result = left - right;

        assert_eq!(result, DialValue::Ratio { num: -1, den: 4 });
    }

}
