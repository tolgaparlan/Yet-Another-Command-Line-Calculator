use std::collections::HashMap;

use num_bigint::{BigInt, Sign};

use crate::{
    error::ArithmeticError,
    parser::{Expr, Factor, Term},
};

pub fn eval_expr(
    expr: Expr,
    variables: &mut HashMap<&str, BigInt>,
) -> Result<BigInt, ArithmeticError> {
    match expr {
        Expr::Sum(e, t) => Ok(eval_expr(*e, variables)? + eval_term(t, variables)?),
        Expr::Subtract(e, t) => Ok(eval_expr(*e, variables)? - eval_term(t, variables)?),
        Expr::Term(t) => eval_term(t, variables),
    }
}

fn eval_term(t: Term, variables: &mut HashMap<&str, BigInt>) -> Result<BigInt, ArithmeticError> {
    match t {
        Term::Mult(t, f) => Ok(eval_term(*t, variables)? * eval_factor(f, variables)?),
        Term::Div(t, f) => {
            let lhs = eval_term(*t, variables)?;
            lhs.checked_div(&eval_factor(f, variables)?)
                .ok_or(ArithmeticError::DivisionByZero(lhs))
        }
        Term::Factor(f) => eval_factor(f, variables),
    }
}

fn eval_factor(
    f: Factor,
    variables: &mut HashMap<&str, BigInt>,
) -> Result<BigInt, ArithmeticError> {
    match f {
        Factor::Number(n) => Ok(BigInt::from_biguint(Sign::Plus, n)),
        Factor::Parenthesis(e) => eval_expr(*e, variables),
        Factor::Negative(n) => Ok(-(eval_factor(*n, variables)?)),
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::BigUint;

    use super::*;

    #[test]
    fn test_evaluation_paranthesis() {
        assert_eq!(
            eval_expr(
                Expr::Term(Term::Factor(Factor::Parenthesis(Box::from(Expr::Term(
                    Term::Div(
                        Box::from(Term::Factor(Factor::Number(BigUint::from(120usize)))),
                        Factor::Number(BigUint::from(24usize)),
                    ),
                ))))),
                &mut HashMap::new()
            ),
            Ok(BigInt::from(5))
        )
    }

    #[test]
    fn test_evaluation_div_by_zero() {
        assert_eq!(
            eval_expr(
                Expr::Term(Term::Div(
                    Box::from(Term::Factor(Factor::Number(BigUint::from(120usize)))),
                    Factor::Number(BigUint::from(0usize)),
                )),
                &mut HashMap::new()
            ),
            Err(ArithmeticError::DivisionByZero(BigInt::from(120)))
        )
    }

    #[test]
    fn test_evaluation_negative() {
        assert_eq!(
            eval_factor(
                Factor::Negative(Box::new(Factor::Number(BigUint::from(120usize)))),
                &mut HashMap::new()
            ),
            Ok(BigInt::from(-120))
        )
    }
}
