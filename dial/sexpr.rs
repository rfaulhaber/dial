use super::builtin::BuiltinFunc;
use super::{EvalError, EvalResult};
use std::cmp::PartialEq;
use std::fmt::{self, Debug, Display};

macro_rules! atom {
    ($atom_type:ident, $atom_val:literal) => {
        DialVal::Atom(Atom::$atom_type($atom_val.into()))
    };
}

macro_rules! sym {
    ($val:literal) => {
        DialVal::Atom(Atom::Sym($val.into()))
    };
}

macro_rules! int {
    ($val:literal) => {
        DialVal::Atom(Atom::Int($val))
    };
}

macro_rules! float {
    ($val:literal) => {
        DialVal::Atom(Atom::Float($val))
    };
}

macro_rules! float {
    ($val:literal) => {
        DialVal::Atom(Atom::Float($val))
    };
}

pub type Number = f64;

#[derive(Clone)]
pub enum DialVal {
    Atom(Atom),
    List(Vec<DialVal>),
    Vec(Vec<DialVal>),
}

impl Display for DialVal {
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
            DialVal::Vec(l) => {
                write!(f, "[")?;

                for (i, v) in l.iter().enumerate() {
                    if i == l.len() - 1 {
                        write!(f, "{}", v)?;
                    } else {
                        write!(f, "{} ", v)?;
                    }
                }

                write!(f, "]")
            }
        }
    }
}

impl Debug for DialVal {
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
            DialVal::Vec(l) => {
                write!(f, "[")?;

                for (i, v) in l.iter().enumerate() {
                    if i == l.len() - 1 {
                        write!(f, "{:?}", v)?;
                    } else {
                        write!(f, "{:?} ", v)?;
                    }
                }

                write!(f, "]")
            }
        }
    }
}

impl PartialEq for DialVal {
    fn eq(&self, other: &DialVal) -> bool {
        match (self, other) {
            (DialVal::Atom(left), DialVal::Atom(right)) => left == right,
            (DialVal::List(left), DialVal::List(right))
            | (DialVal::Vec(left), DialVal::Vec(right)) => {
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

impl From<Atom> for DialVal {
    fn from(a: Atom) -> DialVal {
        DialVal::Atom(a)
    }
}

impl From<i64> for DialVal {
    fn from(i: i64) -> DialVal {
        DialVal::Atom(i.into())
    }
}

impl From<f64> for DialVal {
    fn from(f: f64) -> DialVal {
        DialVal::Atom(f.into())
    }
}

impl From<bool> for DialVal {
    fn from(b: bool) -> DialVal {
        DialVal::Atom(b.into())
    }
}

impl From<&str> for DialVal {
    fn from(s: &str) -> DialVal {
        DialVal::Atom(s.into())
    }
}

impl DialVal {
    pub fn is_number(&self) -> bool {
        match self {
            DialVal::Atom(a) => matches!(a, Atom::Int(_) | Atom::Float(_)),
            _ => false,
        }
    }

    pub fn is_list(&self) -> bool {
        matches!(self, DialVal::List(_))
    }

    pub fn try_as_number(&self) -> Result<f64, EvalError> {
        match self {
            DialVal::Atom(a) => match a {
                Atom::Int(i) => Ok(*i as f64),
                Atom::Float(f) => Ok(*f),
                _ => Err(EvalError::TypeError(format!(
                    "non-numeric type specified: {}",
                    self
                ))),
            },
            _ => Err(EvalError::TypeError(format!(
                "non-numeric type specified: {}",
                self
            ))),
        }
    }
}

// TODO convert to &str
#[derive(Clone)]
pub enum Atom {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Sym(String),
    Keyword(String),
    // TODO add both Builtin (Rust code) and Lambda (user defined) variants
    Builtin { name: String, func: BuiltinFunc },
}

impl From<i64> for Atom {
    fn from(i: i64) -> Atom {
        Atom::Int(i)
    }
}

impl From<f64> for Atom {
    fn from(f: f64) -> Atom {
        Atom::Float(f)
    }
}

impl From<bool> for Atom {
    fn from(b: bool) -> Atom {
        Atom::Bool(b)
    }
}

impl From<&str> for Atom {
    fn from(s: &str) -> Atom {
        Atom::Str(s.into())
    }
}

impl Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::Nil => write!(f, "nil"),
            Atom::Bool(v) => write!(f, "{}", v),
            Atom::Int(v) => write!(f, "{}", v),
            Atom::Float(v) => write!(f, "{}", v),
            Atom::Sym(v) => write!(f, "{}", v),
            Atom::Str(v) => write!(f, "\"{}\"", v),
            Atom::Keyword(v) => write!(f, ":{}", v),
            Atom::Builtin { name, .. } => write!(f, "#builtin: {}", name),
        }
    }
}

impl Debug for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::Nil => write!(f, "nil"),
            Atom::Bool(v) => write!(f, "bool({:?})", v),
            Atom::Int(v) => write!(f, "int({:?})", v),
            Atom::Float(v) => write!(f, "float({:?})", v),
            Atom::Sym(v) => write!(f, "sym({:?})", v),
            Atom::Str(v) => write!(f, "str(\"{:?}\")", v),
            Atom::Keyword(v) => write!(f, "kw(:{:?})", v),
            Atom::Builtin { name, .. } => write!(f, "#builtin: {:?}", name),
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

impl PartialEq for Atom {
    fn eq(&self, other: &Atom) -> bool {
        match (self, other) {
            (
                Atom::Builtin {
                    name: left_name, ..
                },
                Atom::Builtin {
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
                Atom::Keyword
            ),
        }
    }
}

impl Atom {
    pub fn sym(s: &str) -> Atom {
        Atom::Sym(s.into())
    }

    pub fn keyword(s: &str) -> Atom {
        Atom::Keyword(s.into())
    }
}
