use std::collections::HashMap;

use num_bigint::{BigInt, Sign};
use num_traits::{Signed, ToPrimitive};

use crate::{
    error::CalcError,
    functions::FUNCTIONS,
    parser::{Assign, Expr, ExprBitwise, Factor, Term, RES_VAR},
};

/// Saves the result to the given hashtable, only returns the variable
/// string of the left hand side. If there was no lhs variable, will still
/// return RES_VAL string
pub fn eval_assignment(
    ass: Assign,
    variables: &mut HashMap<String, BigInt>,
) -> Result<String, CalcError> {
    match ass {
        Assign::Assign(var, expr) => {
            let res = eval_expr_bitwise(expr, variables)?;
            variables.insert(var.clone(), res);
            Ok(var)
        }
        Assign::ExprBitwise(expr) => {
            // Save the result in the special result variable
            let res = eval_expr_bitwise(expr, variables)?;
            variables.insert(RES_VAR.to_string(), res);
            Ok(RES_VAR.to_string())
        }
    }
}

fn eval_expr_bitwise(
    expr_bitwise: ExprBitwise,
    variables: &mut HashMap<String, BigInt>,
) -> Result<BigInt, CalcError> {
    match expr_bitwise {
        ExprBitwise::BitwiseOr(eb, e) => {
            Ok(eval_expr_bitwise(*eb, variables)? | eval_expr(e, variables)?)
        }
        ExprBitwise::BitwiseAnd(eb, e) => {
            Ok(eval_expr_bitwise(*eb, variables)? & eval_expr(e, variables)?)
        }
        ExprBitwise::BitwiseXor(eb, e) => {
            Ok(eval_expr_bitwise(*eb, variables)? ^ eval_expr(e, variables)?)
        }
        ExprBitwise::BitshiftLeft(eb, e) => {
            let rhs = eval_expr(e, variables)?;
            if rhs.is_negative() {
                return Err(CalcError::InvalidBitShiftNegative);
            }
            let Some(rhs) = rhs.to_u16() else {
                return Err(CalcError::InvalidBitShiftTooLarge(rhs));
            };
            Ok(eval_expr_bitwise(*eb, variables)? << rhs)
        }
        ExprBitwise::BitshiftRight(eb, e) => {
            let rhs = eval_expr(e, variables)?;
            if rhs.is_negative() {
                return Err(CalcError::InvalidBitShiftNegative);
            }
            let Some(rhs) = rhs.to_u16() else {
                return Err(CalcError::InvalidBitShiftTooLarge(rhs));
            };
            Ok(eval_expr_bitwise(*eb, variables)? >> rhs)
        }
        ExprBitwise::Expr(e) => eval_expr(e, variables),
    }
}

fn eval_expr(expr: Expr, variables: &mut HashMap<String, BigInt>) -> Result<BigInt, CalcError> {
    match expr {
        Expr::Sum(e, t) => Ok(eval_expr(*e, variables)? + eval_term(t, variables)?),
        Expr::Subtract(e, t) => Ok(eval_expr(*e, variables)? - eval_term(t, variables)?),
        Expr::Term(t) => eval_term(t, variables),
        Expr::Negative(e) => Ok(-(eval_expr(*e, variables)?)),
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
        Term::Modulo(t, f) => Ok(eval_term(*t, variables)? % eval_factor(f, variables)?),
        Term::Factor(f) => eval_factor(f, variables),
    }
}

fn eval_factor(f: Factor, variables: &mut HashMap<String, BigInt>) -> Result<BigInt, CalcError> {
    match f {
        Factor::Number(n) => Ok(BigInt::from_biguint(Sign::Plus, n)),
        Factor::Parenthesis(e) => eval_expr(*e, variables),
        Factor::Variable(var) => variables
            .get(var.as_str())
            .ok_or(CalcError::UnknownVariable(var))
            .cloned(),
        Factor::Function(f_name, es) => {
            // Parse the function name
            let f = FUNCTIONS
                .get(f_name.as_str())
                .ok_or_else(|| CalcError::UnknownFunction(f_name.to_string()))?;

            // Evaluate all the expressions
            let evaluated_es = es
                .into_iter()
                .map(|e| eval_expr(e, variables))
                .collect::<Result<Vec<BigInt>, CalcError>>()?;

            // Call the function with all the arguments
            Ok((f)(&evaluated_es)?)
        }
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
            eval_expr(
                Expr::Negative(Box::new(Expr::Term(Term::Factor(Factor::Number(
                    BigUint::from(120usize)
                ))))),
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
                Assign::Assign(
                    String::from("asd"),
                    ExprBitwise::Expr(Expr::Term(Term::Factor(Factor::Number(BigUint::from(
                        123usize
                    ))))),
                ),
                &mut vars,
            ),
            Ok(String::from("asd"))
        );
        assert_eq!(vars[&String::from("asd")], BigInt::from(123));
    }

    #[test]
    fn test_evaluation_variable() {
        let mut vars = HashMap::from([(String::from("asd"), BigInt::from(123))]);

        assert_eq!(
            eval_factor(Factor::Variable(String::from("asd")), &mut vars),
            Ok(BigInt::from(123))
        )
    }

    #[test]
    fn test_evaluation_assign_twice() {
        let mut vars = HashMap::from([(String::from("asd"), BigInt::from(123))]);

        assert_eq!(
            eval_assignment(
                Assign::Assign(
                    String::from("asd"),
                    ExprBitwise::Expr(Expr::Term(Term::Factor(Factor::Number(BigUint::from(
                        10usize
                    ))))),
                ),
                &mut vars,
            ),
            Ok(String::from("asd"))
        );
        assert_eq!(vars[&String::from("asd")], BigInt::from(10));
    }

    #[test]
    fn test_evaluation_res_variable_assign() {
        // Must assign an expression result in the special variable
        let mut vars = HashMap::new();

        eval_assignment(
            Assign::ExprBitwise(ExprBitwise::Expr(Expr::Term(Term::Factor(Factor::Number(
                BigUint::from(120usize),
            ))))),
            &mut vars,
        )
        .unwrap();

        assert_eq!(vars[&RES_VAR.to_string()], BigInt::from(120));
    }

    #[test]
    fn test_evaluation_modulo() {
        assert_eq!(
            eval_term(
                Term::Modulo(
                    Box::from(Term::Factor(Factor::Number(BigUint::from(120usize)))),
                    Factor::Number(BigUint::from(17usize)),
                ),
                &mut HashMap::new()
            ),
            Ok(BigInt::from(1))
        )
    }

    #[test]
    fn test_evaluation_shift_large() {
        assert_eq!(
            eval_expr_bitwise(
                ExprBitwise::BitshiftRight(
                    Box::new(ExprBitwise::Expr(Expr::Term(Term::Factor(Factor::Number(
                        BigUint::from(1usize)
                    ))))),
                    Expr::Term(Term::Factor(Factor::Number(BigUint::from(
                        u16::MAX as usize + 1
                    ))))
                ),
                &mut HashMap::new()
            ),
            Err(CalcError::InvalidBitShiftTooLarge(BigInt::from(
                u16::MAX as usize + 1
            )))
        );
    }

    #[test]
    fn test_evaluation_shift_negative() {
        assert_eq!(
            eval_expr_bitwise(
                ExprBitwise::BitshiftRight(
                    Box::new(ExprBitwise::Expr(Expr::Term(Term::Factor(Factor::Number(
                        BigUint::from(1usize),
                    ))))),
                    Expr::Negative(Box::new(Expr::Term(Term::Factor(Factor::Number(
                        BigUint::from(1usize),
                    ))))),
                ),
                &mut HashMap::new(),
            ),
            Err(CalcError::InvalidBitShiftNegative)
        );
    }

    #[test]
    fn test_evaluation_function() {
        assert_eq!(
            eval_factor(
                Factor::Function(
                    "sqrt".to_string(),
                    vec![Expr::Term(Term::Factor(Factor::Number(BigUint::from(
                        16usize
                    ))))]
                ),
                &mut HashMap::new()
            ),
            Ok(BigInt::from(4))
        )
    }

    #[test]
    fn test_evaluation_function_wrong_arguments() {
        assert!(eval_factor(
            Factor::Function(
                "sqrt".to_string(),
                vec![Expr::Negative(Box::new(Expr::Term(Term::Factor(
                    Factor::Number(BigUint::from(16usize))
                ))))]
            ),
            &mut HashMap::new()
        )
        .is_err());
    }

    #[test]
    fn test_evaluation_function_multiple_argument() {
        assert_eq!(
            eval_factor(
                Factor::Function(
                    "pow".to_string(),
                    vec![
                        Expr::Negative(Box::new(Expr::Term(Term::Factor(Factor::Number(
                            BigUint::from(16usize)
                        ))))),
                        Expr::Term(Term::Factor(Factor::Number(BigUint::from(2usize))))
                    ]
                ),
                &mut HashMap::new()
            ),
            Ok(BigInt::from(16 * 16))
        );
    }
}
