use super::EvalResult;
use std::cmp::PartialEq;
use std::fmt::{self, Debug, Display};

#[derive(Clone)]
pub enum DialVal<'s> {
	// TODO flatten?
	Atom(Atom<'s>),
	List(Vec<DialVal<'s>>),
}

impl<'s> Display for DialVal<'s> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			DialVal::Atom(a) => write!(f, "{}", a),
			DialVal::List(l) => {
				write!(f, "(")?;

				for (i, v) in l.iter().enumerate() {
					if i == l.len() - 1 {
						write!(f, "{}", v)?;
					} else {
						write!(f, "{} ", v)?;
					}
				}

				write!(f, ")")
			}
		}
	}
}

impl<'d> Debug for DialVal<'d> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			DialVal::Atom(a) => write!(f, "{:?}", a),
			DialVal::List(l) => {
				write!(f, "(")?;

				for (i, v) in l.iter().enumerate() {
					if i == l.len() - 1 {
						write!(f, "{:?}", v)?;
					} else {
						write!(f, "{:?} ", v)?;
					}
				}

				write!(f, ")")
			}
		}
	}
}

impl<'d> PartialEq for DialVal<'d> {
	fn eq(&self, other: &DialVal<'d>) -> bool {
		match (self, other) {
			(DialVal::Atom(left), DialVal::Atom(right)) => left == right,
			(DialVal::List(left), DialVal::List(right)) => {
				for (l_val, r_val) in left.iter().zip(right.iter()) {
					if l_val != r_val {
						return false;
					}
				}

				true
			}
			_ => false,
		}
	}
}

impl<'s> From<Atom<'s>> for DialVal<'s> {
	fn from(a: Atom<'s>) -> DialVal<'s> {
		DialVal::Atom(a)
	}
}

impl<'s> DialVal<'s> {
	pub fn is_number(&self) -> bool {
		match self {
			DialVal::Atom(a) => matches!(a, Atom::Int(_) | Atom::Float(_)),
			_ => false,
		}
	}

	pub fn is_list(&self) -> bool {
		matches!(self, DialVal::List(_))
	}
}

pub type BuiltinFunc<'f> = fn(&[DialVal<'f>]) -> EvalResult<'f>;

#[derive(Clone)]
pub enum Atom<'a> {
	Nil,
	Bool(bool),
	Int(i64),
	Float(f64),
	Str(&'a str),
	Sym(&'a str),
	Keyword(&'a str),
	Vec(Vec<Atom<'a>>),
	Fn {
		name: &'a str,
		func: BuiltinFunc<'a>,
	},
}

impl<'a> From<i64> for Atom<'a> {
	fn from(i: i64) -> Atom<'a> {
		Atom::Int(i)
	}
}

impl<'a> From<f64> for Atom<'a> {
	fn from(f: f64) -> Atom<'a> {
		Atom::Float(f)
	}
}

impl<'a> From<bool> for Atom<'a> {
	fn from(b: bool) -> Atom<'a> {
		Atom::Bool(b)
	}
}

impl<'a> Display for Atom<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Atom::Nil => write!(f, "nil"),
			Atom::Bool(v) => write!(f, "{}", v),
			Atom::Int(v) => write!(f, "{}", v),
			Atom::Float(v) => write!(f, "{}", v),
			Atom::Sym(v) => write!(f, "{}", v),
			Atom::Str(v) => write!(f, "\"{}\"", v),
			Atom::Keyword(v) => write!(f, ":{}", v),
			Atom::Vec(v) => {
				let val_strs: Vec<String> = v.iter().map(|v| format!("{}", v)).collect();
				write!(f, "[{}]", val_strs.join(", "))
			}
			Atom::Fn { name, .. } => write!(f, "#builtin: {}", name),
		}
	}
}

impl<'a> Debug for Atom<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Atom::Nil => write!(f, "nil"),
			Atom::Bool(v) => write!(f, "bool({:?})", v),
			Atom::Int(v) => write!(f, "int({:?})", v),
			Atom::Float(v) => write!(f, "float({:?})", v),
			Atom::Sym(v) => write!(f, "sym({:?})", v),
			Atom::Str(v) => write!(f, "str(\"{:?}\")", v),
			Atom::Keyword(v) => write!(f, "kw(:{:?})", v),
			Atom::Vec(v) => {
				let val_strs: Vec<String> = v.iter().map(|v| format!("{:?}", v)).collect();
				write!(f, "vec([{}])", val_strs.join(", "))
			}
			Atom::Fn { name, .. } => write!(f, "#builtin: {:?}", name),
		}
	}
}

macro_rules! auto_eq_list {
	($self:ident, $other:ident, $($p:path),+) => {
		match ($self, $other) {
			$(
				($p(left), $p(right)) => left == right,
			)+
			_ => false,
		}
	}
}

impl<'a> PartialEq for Atom<'a> {
	fn eq(&self, other: &Atom<'a>) -> bool {
		match (self, other) {
			(
				Atom::Fn {
					name: left_name, ..
				},
				Atom::Fn {
					name: right_name, ..
				},
			) => left_name == right_name,
			(Atom::Nil, Atom::Nil) => true,
			_ => auto_eq_list!(
				self,
				other,
				Atom::Bool,
				Atom::Int,
				Atom::Float,
				Atom::Sym,
				Atom::Str,
				Atom::Keyword,
				Atom::Vec
			),
		}
	}
}
