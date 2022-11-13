use num_bigint::BigInt;
use num_traits::Signed;
use phf::phf_map;

use crate::error::CalcError;

pub static FUNCTIONS: phf::Map<&'static str, fn(&[BigInt]) -> Result<BigInt, CalcError>> = phf_map! {
    "sqrt"=> sqrt_func,
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
