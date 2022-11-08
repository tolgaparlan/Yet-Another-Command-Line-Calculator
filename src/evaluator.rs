use num_bigint::{BigInt, Sign};

use crate::parser::{Expr, Factor, Term};

pub fn eval_expr(e: Expr) -> BigInt {
    match e {
        Expr::Sum(e, t) => eval_expr(*e) + eval_term(t),
        Expr::Subtract(e, t) => eval_expr(*e) - eval_term(t),
        Expr::Term(t) => eval_term(t),
    }
}

fn eval_term(t: Term) -> BigInt {
    match t {
        Term::Mult(t, f) => eval_term(*t) * eval_factor(f),
        Term::Div(t, f) => eval_term(*t) / eval_factor(f),
        Term::Factor(f) => eval_factor(f),
    }
}

fn eval_factor(f: Factor) -> BigInt {
    match f {
        Factor::Number(n) => BigInt::from_biguint(Sign::Plus, n),
        Factor::Parenthesis(e) => eval_expr(*e),
        Factor::Negative(n) => -(eval_factor(*n)),
    }
}
