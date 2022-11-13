use num_bigint::BigInt;
use num_traits::{Signed, ToPrimitive};
use phf::phf_map;

use crate::error::CalcError;

pub static FUNCTIONS: phf::Map<&'static str, fn(&[BigInt]) -> Result<BigInt, CalcError>> = phf_map! {
    "sqrt" => sqrt_func,
    "pow" => pow_func,
};

fn sqrt_func(inputs: &[BigInt]) -> Result<BigInt, CalcError> {
    if inputs.len() != 1 {
        return Err(CalcError::WrongArgumentCount(
            "sqrt".to_string(),
            1,
            inputs.len(),
        ));
    }

    let num = inputs[0].clone();

    if num.is_negative() {
        return Err(CalcError::InvalidFunctionArgument(format!(
            "sqrt function does not take negative argument. Passed {}",
            num
        )));
    }

    Ok(num.sqrt())
}

fn pow_func(inputs: &[BigInt]) -> Result<BigInt, CalcError> {
    if inputs.len() != 2 {
        return Err(CalcError::WrongArgumentCount(
            "pow".to_string(),
            2,
            inputs.len(),
        ));
    }

    let base = inputs[0].clone();
    let exponent = inputs[1].clone();

    Ok(base.pow(exponent.to_u32().ok_or_else(|| {
        CalcError::InvalidFunctionArgument(format!(
            "Exponent of the pow function cannot be negative. Passed {}",
            exponent
        ))
    })?))
}
