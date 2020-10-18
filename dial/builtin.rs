use super::sexpr::*;
use super::{EvalError, EvalResult};
use crate::Env;

pub type BuiltinFunc = fn(&[DialVal], &mut Env) -> EvalResult;

pub fn add(vals: &[DialVal], _: &mut Env) -> EvalResult {
    let mut sum = 0.0;

    for val in vals.iter() {
        let num = match val {
            DialVal::Int(i) => *i as f64,
            DialVal::Float(f) => *f,
            _ => return Err(EvalError::TypeError("non-numeric type specified".into())),
        };

        sum += num;
    }

    Ok(sum.into())
}

pub fn sub(vals: &[DialVal], _: &mut Env) -> EvalResult {
    let mut diff = 0.0;

    for (i, val) in vals.iter().enumerate() {
        let num = val.try_as_number()?;
        if i == 0 {
            diff = num;
        } else {
            diff -= num;
        }
    }

    Ok(diff.into())
}

pub fn mul(vals: &[DialVal], _: &mut Env) -> EvalResult {
    let mut prod = 1.0;

    for (i, val) in vals.iter().enumerate() {
        let num = val.try_as_number()?;
        if i == 0 {
            prod = num;
        } else {
            prod *= num;
        }
    }

    Ok(prod.into())
}

// TODO implement ratio
pub fn div(vals: &[DialVal], e: &mut Env) -> EvalResult {
    match vals.len() {
        0 => Err(EvalError::ArityError(0)),
        1 => Ok(vals.get(0).unwrap().try_as_number()?.into()),
        _ => {
            let (first, rest) = vals.split_at(1);

            let first_num = first.get(0).unwrap().try_as_number()?;
            let prod = mul(rest, e)?.try_as_number()?;

            if prod == 0.0 {
                return Err(EvalError::InvalidArgumentError(
                    "cannot divide by zero".into(),
                ));
            }

            Ok((first_num / prod).into())
        }
    }
}
