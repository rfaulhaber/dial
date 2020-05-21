use std::fmt::Display;

// macro_rules! sexpr {
// 	(nil) => {
// 		S::Atom(Box::new(Atom::Nil))
// 	};
// 	($l:literal) => {
// 		S::Atom(Box::new($l.into()))
// 	}
// }

#[derive(Debug, Clone, PartialEq)]
pub enum S<'s> {
	Atom(Box<Atom<'s>>),
	List(Vec<S<'s>>),
}

impl <'s> Display for S<'s> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			S::Atom(a) => write!(f, "{}", a),
			S::List(l) => {
				write!(f, "(")?;

				for v in l {
					write!(f, "{}", v)?;
				}

				Ok(write!(f, ")")?)
			}
		}
	}
}

impl<'s> From<Atom<'s>> for S<'s> {
	fn from(a: Atom<'s>) -> S<'s> {
		S::Atom(Box::new(a))
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum Atom<'a> {
	Nil,
	Bool(bool),
	Int(i64),
	Float(f64),
	Str(&'a str),
	Sym(&'a str),
	Keyword(&'a str),
	Vec(Vec<Atom<'a>>),
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

impl <'a> From<bool> for Atom<'a> {
	fn from(b: bool) -> Atom<'a> {
		Atom::Bool(b)
	}
}

impl<'a> Display for Atom<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Atom::Nil => write!(f, "nil"),
			Atom::Bool(v)  => write!(f, "{}", v),
			Atom::Int(v) => write!(f, "{}", v),
			Atom::Float(v) => write!(f, "{}", v),
			Atom::Sym(v) => write!(f, "{}", v),
			Atom::Str(v) => write!(f, "\"{}\"", v),
			Atom::Keyword(v) => write!(f, ":{}", v),
			Atom::Vec(v) => {
				let val_strs: Vec<String> = v.iter().map(|v| format!("{}", v)).collect();
				write!(f, "[{}]", val_strs.join(", "))
			}
		}
	}
}