use num_bigint::{BigInt, Sign};

use crate::{
    error::ArithmeticError,
    parser::{Expr, Factor, Term},
};

pub fn eval_expr(e: Expr) -> Result<BigInt, ArithmeticError> {
    match e {
        Expr::Sum(e, t) => Ok(eval_expr(*e)? + eval_term(t)?),
        Expr::Subtract(e, t) => Ok(eval_expr(*e)? - eval_term(t)?),
        Expr::Term(t) => eval_term(t),
    }
}

fn eval_term(t: Term) -> Result<BigInt, ArithmeticError> {
    match t {
        Term::Mult(t, f) => Ok(eval_term(*t)? * eval_factor(f)?),
        Term::Div(t, f) => {
            let lhs = eval_term(*t)?;
            lhs.checked_div(&eval_factor(f)?)
                .ok_or(ArithmeticError::DivisionByZero(lhs))
        }
        Term::Factor(f) => eval_factor(f),
    }
}

fn eval_factor(f: Factor) -> Result<BigInt, ArithmeticError> {
    match f {
        Factor::Number(n) => Ok(BigInt::from_biguint(Sign::Plus, n)),
        Factor::Parenthesis(e) => eval_expr(*e),
        Factor::Negative(n) => Ok(-(eval_factor(*n)?)),
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::BigUint;

    use super::*;

    #[test]
    fn test_evaluation_paranthesis() {
        assert_eq!(
            eval_expr(Expr::Term(Term::Factor(Factor::Parenthesis(Box::from(
                Expr::Term(Term::Div(
                    Box::from(Term::Factor(Factor::Number(BigUint::from(120usize)))),
                    Factor::Number(BigUint::from(24usize)),
                ),)
            ))))),
            Ok(BigInt::from(5))
        )
    }

    #[test]
    fn test_evaluation_div_by_zero() {
        assert_eq!(
            eval_expr(Expr::Term(Term::Div(
                Box::from(Term::Factor(Factor::Number(BigUint::from(120usize)))),
                Factor::Number(BigUint::from(0usize)),
            ))),
            Err(ArithmeticError::DivisionByZero(BigInt::from(120)))
        )
    }

    #[test]
    fn test_evaluation_negative() {
        assert_eq!(
            eval_factor(Factor::Negative(Box::new(Factor::Number(BigUint::from(
                120usize
            ))))),
            Ok(BigInt::from(-120))
        )
    }
}
