use super::sexpr::*;
use super::{EvalError, EvalResult};
use crate::Env;
use num::rational::{Ratio, Rational64};
use std::ops::{Add, Div, Mul, Sub};

macro_rules! assert_arity {
    ($list:ident, $arity:literal) => {
        if $list.len() > $arity {
            return Err(EvalError::ArityError($arity));
        }
    };
}

macro_rules! define_is_type {
    ($name:ident, $ty:ident) => {
        pub fn $name(vals: &[DialVal], _e: &mut Env) -> EvalResult {
            assert_arity!(vals, 1);

            match vals.get(0).unwrap() {
                DialVal::$ty(_) => Ok(true.into()),
                _ => Ok(false.into()),
            }
        }
    };
}

macro_rules! assert_all_numbers {
    ($list:ident) => {
        if let Some(val) find_non_number($list) {
            return Err(EvalError::TypeError("TODO wrong type".into()));
        }
    }
}

pub type BuiltinFunc = fn(&[DialVal], &mut Env) -> EvalResult;

enum DialNumber {
    Int(i64),
    Float(f64),
    Ratio(Rational64),
}

impl Into<f64> for DialNumber {
    fn into(self) -> f64 {
        match self {
            DialNumber::Int(i) => i as f64,
            DialNumber::Float(f) => f,
            // TODO this may be a bad idea...
            DialNumber::Ratio(r) => (*r.numer() as f64) / (*r.denom() as f64),
        }
    }
}

impl Into<i64> for DialNumber {
    fn into(self) -> i64 {
        match self {
            DialNumber::Int(i) => i,
            DialNumber::Float(f) => f as i64,
            DialNumber::Ratio(r) => r.to_integer(),
        }
    }
}

impl Into<Rational64> for DialNumber {
    fn into(self) -> Rational64 {
        match self {
            DialNumber::Int(i) => Rational64::from(i),
            // TODO fix
            DialNumber::Float(f) => Rational64::from(f as i64),
            DialNumber::Ratio(r) => r,
        }
    }
}

pub fn add(vals: &[DialVal], _: &mut Env) -> EvalResult {
    if has_float(vals) {
        let mut sum = 0.0;

        for val in vals.iter() {
            let num = val.try_as_float()?;
            sum += num;
        }

        Ok(sum.into())
    } else {
        let mut sum = 0;

        for val in vals.iter() {
            let num = val.try_as_int()?;
            sum += num;
        }

        Ok(sum.into())
    }
}

pub fn sub(vals: &[DialVal], _: &mut Env) -> EvalResult {
    if has_float(vals) {
        let mut diff = 0.0;

        for (i, val) in vals.iter().enumerate() {
            let num = val.try_as_float()?;
            if i == 0 {
                diff = num;
            } else {
                diff -= num;
            }
        }

        Ok(diff.into())
    } else {
        let mut diff = 0;

        for (i, val) in vals.iter().enumerate() {
            let num = val.try_as_int()?;
            if i == 0 {
                diff = num;
            } else {
                diff -= num;
            }
        }

        Ok(diff.into())
    }
}

pub fn mul(vals: &[DialVal], _: &mut Env) -> EvalResult {
    if has_float(vals) {
        let mut prod = 1.0;

        for (i, val) in vals.iter().enumerate() {
            let num = val.try_as_float()?;
            if i == 0 {
                prod = num;
            } else {
                prod *= num;
            }
        }

        Ok(prod.into())
    } else {
        let mut prod = 1;

        for (i, val) in vals.iter().enumerate() {
            let num = val.try_as_int()?;
            if i == 0 {
                prod = num;
            } else {
                prod *= num;
            }
        }

        Ok(prod.into())
    }
}

// TODO implement ratio
pub fn div(vals: &[DialVal], e: &mut Env) -> EvalResult {
    match vals.len() {
        0 => Err(EvalError::ArityError(0)),
        1 => Ok(vals.get(0).unwrap().clone()),
        _ => {
            if has_float(vals) {
                let (first, rest) = vals.split_at(1);

                let first_num = first.get(0).unwrap().try_as_float()?;
                let prod = mul(rest, e)?.try_as_float()?;

                if prod == 0.0 {
                    return Err(EvalError::InvalidArgumentError(
                        "cannot divide by zero".into(),
                    ));
                }

                Ok((first_num / prod).into())
            } else {
                let (first, rest) = vals.split_at(1);

                let first_num = first.get(0).unwrap().try_as_int()?;
                let prod = mul(rest, e)?.try_as_int()?;

                if prod == 0 {
                    return Err(EvalError::InvalidArgumentError(
                        "cannot divide by zero".into(),
                    ));
                }

                let ratio = Rational64::new(first_num, prod);

                if ratio.is_integer() {
                    Ok(DialVal::Int(ratio.to_integer()))
                } else {
                    Ok(DialVal::Ratio(ratio))
                }
            }
        }
    }
}

pub fn list(vals: &[DialVal], _e: &mut Env) -> EvalResult {
    Ok(DialVal::List(Vec::from(vals)))
}

pub fn is_empty(vals: &[DialVal], _e: &mut Env) -> EvalResult {
    match vals.first() {
        Some(l) => match l {
            DialVal::List(l) => Ok((l.len() == 0).into()),
            _ => Err(EvalError::InvalidArgumentError(format!(
                "empty? only valid on lists"
            ))),
        },
        None => Err(EvalError::ArityError(1)),
    }
}

pub fn count(vals: &[DialVal], _e: &mut Env) -> EvalResult {
    match vals.first() {
        Some(l) => match l {
            DialVal::List(l) => Ok((l.len() as i64).into()),
            _ => Err(EvalError::InvalidArgumentError(format!(
                "count only valid on lists"
            ))),
        },
        None => Err(EvalError::ArityError(1)),
    }
}

pub fn eq(vals: &[DialVal], _e: &mut Env) -> EvalResult {
    match vals.len() {
        0 => Err(EvalError::ArityError(0)),
        1 => Ok(DialVal::Bool(true)),
        _ => {
            let (first, rest) = vals.split_at(1);
            let first = first.get(0).unwrap();

            for val in rest {
                if first != val {
                    return Ok(DialVal::Bool(false));
                }
            }

            Ok(DialVal::Bool(true))
        }
    }
}

define_is_type!(is_int, Int);
define_is_type!(is_float, Float);
define_is_type!(is_string, Str);
define_is_type!(is_ratio, Ratio);
define_is_type!(is_keyword, Keyword);
define_is_type!(is_symbol, Sym);
define_is_type!(is_list, List);

fn has_float(vals: &[DialVal]) -> bool {
    for val in vals {
        if matches!(val, DialVal::Float(_)) {
            return true;
        }
    }

    false
}

fn has_ratio(vals: &[DialVal]) -> bool {
    for val in vals {
        if matches!(val, DialVal::Ratio(_)) {
            return true;
        }
    }

    false
}

fn get_numbers(vals: &[DialVal]) -> Vec<DialNumber> {
    vals.iter()
        .map(|val| match val {
            DialVal::Int(i) => DialNumber::Int(*i),
            DialVal::Float(f) => DialNumber::Float(*f),
            DialVal::Ratio(r) => DialNumber::Ratio(*r),
            _ => unreachable!(),
        })
        .collect()
}

fn find_non_number(vals: &[DialVal]) -> Option<DialVal> {
    vals.iter()
        .find(|val| !matches!(val, DialVal::Int(_) | DialVal::Ratio(_) | DialVal::Float(_)))
        .map(|val| val.clone())
}
