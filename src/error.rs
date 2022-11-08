use num_bigint::BigInt;

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum ArithmeticError {
    #[error("Invalid token at index {0}")]
    InvalidToken(usize),
    #[error("Attempted dividing {0} by zero")]
    DivisionByZero(BigInt),
    #[error("Expected `)`")]
    UnclosedParanthesis,
    #[error("Expected Number")]
    ExpectedNumber,
}
