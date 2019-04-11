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
            DialValue::Nil => write!(f, "nil"),
        }
    }
}

impl Add for DialValue {
    type Output = DialValue;

    fn add(self, other: DialValue) -> Self::Output {
        match self {
            DialValue::Integer(int) => match other {
                DialValue::Integer(other_int) => DialValue::Integer(int + other_int),
                DialValue::Float(float) => DialValue::Float(int as f64 + float),
                DialValue::String(s) => DialValue::String(format!("{}{}", int, s)),
                _ => panic!("addition not defiend for this type"),
            },
            DialValue::Float(float) => match other {
                DialValue::Integer(int) => DialValue::Float(int as f64 + float),
                DialValue::Float(other_float) => DialValue::Float(float + other_float),
                DialValue::String(s) => DialValue::String(format!("{}{}", float, s)),
                _ => panic!("addition not defiend for this type"),
            },
            DialValue::String(s) => match other {
                DialValue::Integer(int) => DialValue::String(format!("{}{}", s, int)),
                DialValue::Float(float) => DialValue::String(format!("{}{}", s, float)),
                DialValue::String(other_str) => DialValue::String(format!("{}{}", s, other_str)),
                _ => panic!("addition not defiend for this type"),
            },
            _ => {
                info!("self: {}, other: {}", self, other);
                panic!("addition not defiend for this type");
            }
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
                _ => panic!("subtraction not defiend for this type"),
            },
            DialValue::Float(float) => match other {
                DialValue::Integer(int) => DialValue::Float(float - int as f64),
                DialValue::Float(other_float) => DialValue::Float(float - other_float),
                _ => panic!("subtraction not defiend for this type"),
            },
            _ => panic!("subtraction not defiend for this type"),
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
                _ => panic!("multiplication not defiend for this type"),
            },
            DialValue::Float(float) => match other {
                DialValue::Integer(int) => DialValue::Float(int as f64 * float),
                DialValue::Float(other_float) => DialValue::Float(float * other_float),
                _ => panic!("multiplication not defiend for this type"),
            },
            _ => panic!("multiplication not defiend for this type"),
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
                _ => panic!("division not defiend for this type"),
            },
            DialValue::Float(float) => match other {
                DialValue::Integer(int) => DialValue::Float(int as f64 / float),
                DialValue::Float(other_float) => DialValue::Float(float / other_float),
                _ => panic!("division not defiend for this type"),
            },
            _ => panic!("division not defiend for this type"),
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
}
