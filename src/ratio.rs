use std::fmt;

use std::iter::{Product, Sum};
use std::ops::{Add, Div, Mul, Sub};

pub struct Ratio {
	num: i64,
	den: i64,
}

impl fmt::Display for Ratio {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}/{}", self.num, self.den)
	}
}
