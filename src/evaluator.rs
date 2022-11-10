use std::collections::HashMap;

use num_bigint::{BigInt, Sign};

use crate::{
    error::CalcError,
    parser::{Assignment, Expr, Factor, Term, RES_VAR},
};

pub fn eval_assignment(
    ass: Assignment,
    variables: &mut HashMap<String, BigInt>,
) -> Result<BigInt, CalcError> {
    match ass {
        Assignment::Assign(var, expr) => {
            let res = eval_expr(expr, variables)?;
            variables.insert(var, res.clone());
            Ok(res)
        }
        Assignment::Expr(expr) => {
            // Save the result in the special result variable
            let res = eval_expr(expr, variables)?;
            variables.insert(RES_VAR.to_string(), res.clone());
            Ok(res)
        }
    }
}

pub fn eval_expr(expr: Expr, variables: &mut HashMap<String, BigInt>) -> Result<BigInt, CalcError> {
    match expr {
        Expr::Sum(e, t) => Ok(eval_expr(*e, variables)? + eval_term(t, variables)?),
        Expr::Subtract(e, t) => Ok(eval_expr(*e, variables)? - eval_term(t, variables)?),
        Expr::Term(t) => eval_term(t, variables),
    }
}

fn eval_term(t: Term, variables: &mut HashMap<String, BigInt>) -> Result<BigInt, CalcError> {
    match t {
        Term::Mult(t, f) => Ok(eval_term(*t, variables)? * eval_factor(f, variables)?),
        Term::Div(t, f) => {
            let lhs = eval_term(*t, variables)?;
            lhs.checked_div(&eval_factor(f, variables)?)
                .ok_or(CalcError::DivisionByZero(lhs))
        }
        Term::Factor(f) => eval_factor(f, variables),
    }
}

fn eval_factor(f: Factor, variables: &mut HashMap<String, BigInt>) -> Result<BigInt, CalcError> {
    match f {
        Factor::Number(n) => Ok(BigInt::from_biguint(Sign::Plus, n)),
        Factor::Parenthesis(e) => eval_expr(*e, variables),
        Factor::Negative(n) => Ok(-(eval_factor(*n, variables)?)),
        Factor::Variable(var) => variables
            .get(var.as_str())
            .ok_or(CalcError::UnknownVariable(var))
            .cloned(),
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
            Err(CalcError::DivisionByZero(BigInt::from(120)))
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

    #[test]
    fn test_evaluation_assignment() {
        let mut vars = HashMap::new();
        assert_eq!(
            eval_assignment(
                Assignment::Assign(
                    String::from("asd"),
                    Expr::Term(Term::Factor(Factor::Number(BigUint::from(123usize)))),
                ),
                &mut vars,
            ),
            Ok(BigInt::from(123))
        );
        assert_eq!(vars[&String::from("asd")], BigInt::from(123));
    }

    #[test]
    fn test_evaluation_variable() {
        let mut vars = HashMap::from([(String::from("asd"), BigInt::from(123))]);

        assert_eq!(
            eval_factor(
                Factor::Negative(Box::new(Factor::Variable(String::from("asd")))),
                &mut vars
            ),
            Ok(BigInt::from(-123))
        )
    }

    #[test]
    fn test_evaluation_assign_twice() {
        let mut vars = HashMap::from([(String::from("asd"), BigInt::from(123))]);

        assert_eq!(
            eval_assignment(
                Assignment::Assign(
                    String::from("asd"),
                    Expr::Term(Term::Factor(Factor::Negative(Box::new(Factor::Variable(
                        String::from("asd")
                    ))))),
                ),
                &mut vars,
            ),
            Ok(BigInt::from(-123))
        );
        assert_eq!(vars[&String::from("asd")], BigInt::from(-123));
    }

    #[test]
    fn test_evaluation_res_variable_assign() {
        // Must assign an expression result in the special variable
        let mut vars = HashMap::new();

        eval_assignment(
            Assignment::Expr(Expr::Term(Term::Factor(Factor::Negative(Box::new(
                Factor::Number(BigUint::from(120usize)),
            ))))),
            &mut vars,
        )
        .unwrap();

        assert_eq!(vars[&RES_VAR.to_string()], BigInt::from(-120));
    }
}
