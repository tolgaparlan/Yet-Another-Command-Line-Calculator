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
    #[error("Cannot use special function name as variable: {0}")]
    SpecialVariableInvalidUse(String),
    #[error("Right hand side of bit shift operation too large: {0}")]
    InvalidBitShiftTooLarge(BigInt),
    #[error("Attempted to bit shift by negative value")]
    InvalidBitShiftNegative,
    #[error("Function {0} expects {1} arguments. Passed {2}")]
    WrongArgumentCount(String, usize, usize),
    #[error("Unknown Function {0}")]
    UnknownFunction(String),
    #[error("{0}")]
    InvalidFunctionArgument(String),
}
