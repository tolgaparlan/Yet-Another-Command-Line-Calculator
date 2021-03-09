use crate::parser::{Expr, Term, Factor};

pub fn eval_expr(e: Expr) -> i64 {
    match e {
        Expr::Sum(e, t) => eval_expr(*e) + eval_term(t),
        Expr::Subtract(e, t) => eval_expr(*e) - eval_term(t),
        Expr::Term(t) => eval_term(t)
    }
}

fn eval_term(t: Term) -> i64 {
    match t {
        Term::Mult(t, f) => eval_term(*t) * eval_factor(f),
        Term::Div(t, f) => eval_term(*t) / eval_factor(f),
        Term::Factor(f) => eval_factor(f)
    }
}

fn eval_factor(f: Factor) -> i64 {
    match f {
        Factor::Number(n) => n as i64,
        Factor::Parenthesis(e) => eval_expr(*e),
        Factor::Negative(n) => -(eval_factor(*n))
    }
}