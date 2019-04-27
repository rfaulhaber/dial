// TODO make own type!
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

impl Add for Ratio {

}

impl Sub for Ratio {

}

impl Mul for Ratio {

}

impl Div for Ratio {

}

impl Product for Ratio {

}

impl Sum for Ratio {

}