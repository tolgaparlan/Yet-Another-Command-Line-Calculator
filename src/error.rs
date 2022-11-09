use num_bigint::BigInt;

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum CalcError {
    #[error("Invalid token at index {0}")]
    InvalidToken(usize),
    #[error("Attempted dividing {0} by zero")]
    DivisionByZero(BigInt),
    #[error("Expected `)`")]
    UnclosedParanthesis,
    #[error("Invalid Expresssion")]
    InvalidExpression,
    #[error("Unknown Variable {0}")]
    UnknownVariable(String),
}
