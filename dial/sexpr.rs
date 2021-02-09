use crate::Env;

use super::{builtin::BuiltinFunc, EvalError};
use num::rational::Rational64;
use std::cmp::PartialEq;
use std::fmt::{self, Debug, Display};

#[derive(Clone)]
pub enum DialVal {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Sym(String),
    Keyword(String),
    Ratio(Rational64),
    Builtin {
        name: String,
        func: BuiltinFunc,
    },
    Lambda {
        params: Vec<String>,
        body: Box<DialVal>,
        env: Env,
    },

    // collections
    List(Vec<DialVal>),
    Vec(Vec<DialVal>),
}

impl Display for DialVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DialVal::Nil => write!(f, "nil"),
            DialVal::Bool(v) => write!(f, "{}", v),
            DialVal::Int(v) => write!(f, "{}", v),
            DialVal::Float(v) => write!(f, "{}", v),
            DialVal::Sym(v) => write!(f, "{}", v),
            DialVal::Str(v) => write!(f, "\"{}\"", v),
            DialVal::Keyword(v) => write!(f, ":{}", v),
            DialVal::Ratio(v) => write!(f, "{}/{}", v.numer(), v.denom()),
            DialVal::Builtin { name, .. } => write!(f, "#builtin: {}", name),
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
            DialVal::Lambda { .. } => write!(f, "#<function>"),
        }
    }
}

impl Debug for DialVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DialVal::Nil => write!(f, "nil"),
            DialVal::Bool(v) => write!(f, "bool({:?})", v),
            DialVal::Int(v) => write!(f, "int({:?})", v),
            DialVal::Float(v) => write!(f, "float({:?})", v),
            DialVal::Sym(v) => write!(f, "sym({:?})", v),
            DialVal::Str(v) => write!(f, "str(\"{:?}\")", v),
            DialVal::Keyword(v) => write!(f, "kw(:{:?})", v),
            DialVal::Ratio(v) => write!(f, "ratio({}/{})", v.numer(), v.denom()),
            DialVal::Builtin { name, .. } => write!(f, "#builtin: {:?}", name),
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
            DialVal::Lambda { .. } => write!(f, "#<function>"),
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

impl PartialEq for DialVal {
    fn eq(&self, other: &DialVal) -> bool {
        match (self, other) {
            (DialVal::List(left), DialVal::List(right))
            | (DialVal::Vec(left), DialVal::Vec(right)) => {
                for (l_val, r_val) in left.iter().zip(right.iter()) {
                    if l_val != r_val {
                        return false;
                    }
                }

                true
            }
            (
                DialVal::Builtin {
                    name: left_name, ..
                },
                DialVal::Builtin {
                    name: right_name, ..
                },
            ) => left_name == right_name,
            (DialVal::Nil, DialVal::Nil) => true,
            _ => auto_eq_list!(
                self,
                other,
                DialVal::Bool,
                DialVal::Int,
                DialVal::Float,
                DialVal::Sym,
                DialVal::Str,
                DialVal::Keyword,
                DialVal::Ratio
            ),
            _ => false,
        }
    }
}

impl From<f64> for DialVal {
    fn from(f: f64) -> DialVal {
        DialVal::Float(f)
    }
}

impl From<i64> for DialVal {
    fn from(i: i64) -> DialVal {
        DialVal::Int(i)
    }
}

impl From<bool> for DialVal {
    fn from(b: bool) -> DialVal {
        DialVal::Bool(b)
    }
}

pub struct DialValIter {
    items: Vec<DialVal>,
}

impl Iterator for DialValIter {
    type Item = DialVal;

    fn next(&mut self) -> Option<Self::Item> {
        self.items.pop()
    }
}

impl DialVal {
    pub fn is_number(&self) -> bool {
        matches!(self, DialVal::Int(_) | DialVal::Float(_))
    }

    pub fn is_list(&self) -> bool {
        matches!(self, DialVal::List(_))
    }

    pub fn try_as_float(&self) -> Result<f64, EvalError> {
        match self {
            DialVal::Float(f) => Ok(*f),
            DialVal::Int(i) => Ok(*i as f64),
            _ => Err(EvalError::TypeError(format!(
                "non-float type specified: {}",
                self
            ))),
        }
    }

    pub fn try_as_int(&self) -> Result<i64, EvalError> {
        match self {
            DialVal::Int(i) => Ok(*i),
            _ => Err(EvalError::TypeError(format!(
                "non-int type specified: {}",
                self
            ))),
        }
    }

    pub fn into_iter(self) -> DialValIter {
        match self {
            DialVal::List(l) => {
                let mut items = l.clone();
                items.reverse();
                DialValIter { items }
            }
            i => DialValIter { items: vec![i] },
        }
    }

    pub fn display_type(&self) -> String {
        match self {
            DialVal::Nil => "nil",
            DialVal::Bool(_) => "bool",
            DialVal::Int(_) => "int",
            DialVal::Float(_) => "float",
            DialVal::Str(_) => "string",
            DialVal::Sym(_) => "symbol",
            DialVal::Keyword(_) => "keyword",
            DialVal::Ratio(_) => "ratio",
            DialVal::Builtin { .. } => "builtin",
            DialVal::Lambda { .. } => "fn",
            DialVal::List(_) => "list",
            DialVal::Vec(_) => "vector",
        }
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_list_works() {
        let val = DialVal::List(vec![
            DialVal::Sym("+".into()),
            DialVal::Int(1),
            DialVal::Int(2),
        ]);

        let mut iter = val.into_iter();

        assert_eq!(Some(DialVal::Sym("+".into())), iter.next());
        assert_eq!(Some(DialVal::Int(1)), iter.next());
        assert_eq!(Some(DialVal::Int(2)), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn iter_single_item_works() {
        let val = DialVal::Sym("+".into());

        let mut iter = val.into_iter();

        assert_eq!(Some(DialVal::Sym("+".into())), iter.next());
        assert_eq!(None, iter.next());
    }
}
