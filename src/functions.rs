use num_bigint::BigInt;
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
    Ok(num.sqrt())
}
