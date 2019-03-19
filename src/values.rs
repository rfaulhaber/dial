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