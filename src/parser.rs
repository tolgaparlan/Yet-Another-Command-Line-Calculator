use crate::{error::CalcError, tokenizer::Token};
use num_bigint::BigUint;

// This must not be an alphanumeric value in order to keep the parsing logic simple
pub const RES_VAR: char = '$';

#[derive(Debug, PartialEq)]
pub enum Assign {
    Assign(String, ExprBitwise),
    ExprBitwise(ExprBitwise),
}

#[derive(Debug, PartialEq)]
pub enum ExprBitwise {
    BitwiseOr(Box<ExprBitwise>, Expr),
    BitwiseAnd(Box<ExprBitwise>, Expr),
    BitwiseXor(Box<ExprBitwise>, Expr),
    BitshiftLeft(Box<ExprBitwise>, Expr),
    BitshiftRight(Box<ExprBitwise>, Expr),
    Expr(Expr),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Sum(Box<Expr>, Term),
    Subtract(Box<Expr>, Term),
    Term(Term),
    Negative(Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum Term {
    Mult(Box<Term>, Factor),
    Div(Box<Term>, Factor),
    Modulo(Box<Term>, Factor),
    Factor(Factor),
}

#[derive(Debug, PartialEq)]
pub enum Factor {
    Number(BigUint),
    Variable(String),
    Parenthesis(Box<Expr>),
}

pub fn parse_assignment(tokens: &[Token]) -> Result<Assign, CalcError> {
    let mut it = tokens.iter().enumerate();

    // If the first token is a variable, followed by an equals sign, this is an assignment.
    // Otherwise just an expression

    if let Some((_, Token::Variable(var))) = it.next() {
        if let Some((i, Token::Equals)) = it.next() {
            return Ok(Assign::Assign(
                var.to_string(),
                parse_bitwise_expr(&tokens[i + 1..])?,
            ));
        }
    }

    Ok(Assign::ExprBitwise(parse_bitwise_expr(tokens)?))
}

fn parse_bitwise_expr(tokens: &[Token]) -> Result<ExprBitwise, CalcError> {
    let it = tokens.iter().enumerate().peekable();

    for (index, token) in it {
        match token {
            Token::BitwiseAnd => {
                return Ok(ExprBitwise::BitwiseAnd(
                    Box::new(parse_bitwise_expr(&tokens[0..index])?),
                    parse_expr(&tokens[index + 1..])?,
                ))
            }
            Token::BitwiseOr => {
                return Ok(ExprBitwise::BitwiseOr(
                    Box::new(parse_bitwise_expr(&tokens[0..index])?),
                    parse_expr(&tokens[index + 1..])?,
                ))
            }
            Token::BitwiseXor => {
                return Ok(ExprBitwise::BitwiseXor(
                    Box::new(parse_bitwise_expr(&tokens[0..index])?),
                    parse_expr(&tokens[index + 1..])?,
                ))
            }
            Token::BitshiftRight => {
                return Ok(ExprBitwise::BitshiftRight(
                    Box::new(parse_bitwise_expr(&tokens[0..index])?),
                    parse_expr(&tokens[index + 1..])?,
                ))
            }
            Token::BitshiftLeft => {
                return Ok(ExprBitwise::BitshiftLeft(
                    Box::new(parse_bitwise_expr(&tokens[0..index])?),
                    parse_expr(&tokens[index + 1..])?,
                ))
            }
            _ => continue,
        }
    }

    // matched nothing so must be a normal expression
    Ok(ExprBitwise::Expr(parse_expr(tokens)?))
}

fn parse_expr(tokens: &[Token]) -> Result<Expr, CalcError> {
    let mut it = tokens.iter().enumerate().peekable();

    // If the first token is a minus, this is a negative
    if let Some((_, &Token::Minus)) = it.peek() {
        // avoid double negative expressions
        it.next();
        if let Some((_, &Token::Minus)) = it.peek() {
            return Err(CalcError::InvalidExpression);
        }

        return Ok(Expr::Negative(Box::new(parse_expr(&tokens[1..])?)));
    }

    while let Some((index, token)) = it.next() {
        match token {
            Token::Plus => {
                return Ok(Expr::Sum(
                    Box::from(parse_expr(&tokens[0..index])?),
                    parse_term(&tokens[index + 1..])?,
                ));
            }
            Token::Minus if index > 0 => {
                return Ok(Expr::Subtract(
                    Box::from(parse_expr(&tokens[0..index])?),
                    parse_term(&tokens[index + 1..])?,
                ));
            }
            Token::LeftPar => {
                if !matching_paranthesis(it.by_ref()) {
                    return Err(CalcError::UnclosedParanthesis);
                }
            }
            _ => continue,
        }
    }

    // reached the end of the expression without matching -> must be a term
    Ok(Expr::Term(parse_term(tokens)?))
}

fn parse_term(v: &[Token]) -> Result<Term, CalcError> {
    let mut it = v.iter().enumerate();

    while let Some((index, token)) = it.next() {
        match token {
            Token::Mult | Token::Div | Token::Modulo => {
                let lhs = Box::from(parse_term(&v[0..index])?);
                let rhs = parse_factor(&v[index + 1..])?;
                return Ok(match token {
                    Token::Mult => Term::Mult(lhs, rhs),
                    Token::Div => Term::Div(lhs, rhs),
                    Token::Modulo => Term::Modulo(lhs, rhs),
                    _ => panic!(), // Cannot happen as checked above
                });
            }
            Token::LeftPar => {
                if !matching_paranthesis(it.by_ref()) {
                    return Err(CalcError::UnclosedParanthesis);
                }
            }
            _ => {
                continue;
            }
        }
    }

    // reached the end of the expression without matching must be a factor
    Ok(Term::Factor(parse_factor(v)?))
}

fn parse_factor(tokens: &[Token]) -> Result<Factor, CalcError> {
    let mut it = tokens.iter();

    match &mut it.next() {
        Some(Token::Number(n)) if it.next().is_none() => Ok(Factor::Number(n.clone())),
        Some(Token::Variable(var)) if it.next().is_none() => Ok(Factor::Variable(var.to_string())),
        Some(Token::ResultVariable) if it.next().is_none() => {
            Ok(Factor::Variable(RES_VAR.to_string()))
        }
        Some(Token::LeftPar) => {
            if let Some(Token::RightPar) = it.last() {
                Ok(Factor::Parenthesis(Box::from(parse_expr(
                    &tokens[1..tokens.len() - 1],
                )?)))
            } else {
                Err(CalcError::UnclosedParanthesis)
            }
        }
        _ => Err(CalcError::InvalidExpression),
    }
}

/// iterate until the matching right parenthesis
fn matching_paranthesis(it: &mut dyn Iterator<Item = (usize, &Token)>) -> bool {
    let mut left_count = 1;
    for (_, t) in it {
        left_count += match t {
            Token::LeftPar => 1,
            Token::RightPar => -1,
            _ => 0,
        };
        if left_count == 0 {
            break;
        }
    }
    left_count == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_expr() {
        assert_eq!(
            parse_expr(&[
                Token::Number(1usize.into()),
                Token::Plus,
                Token::Number(123usize.into()),
                Token::Mult,
                Token::LeftPar,
                Token::Number(12usize.into()),
                Token::Div,
                Token::Number(234usize.into()),
                Token::RightPar
            ]),
            Ok(Expr::Sum(
                Box::from(Expr::Term(Term::Factor(Factor::Number(1usize.into())))),
                Term::Mult(
                    Box::from(Term::Factor(Factor::Number(123usize.into()))),
                    Factor::Parenthesis(Box::from(Expr::Term(Term::Div(
                        Box::from(Term::Factor(Factor::Number(12usize.into()))),
                        Factor::Number(234usize.into())
                    )))),
                ),
            ))
        );
    }

    #[test]
    fn test_parser_assignment() {
        assert_eq!(
            parse_assignment(&[
                Token::Variable("a".to_string()),
                Token::Equals,
                Token::Number(12usize.into()),
            ]),
            Ok(Assign::Assign(
                "a".to_string(),
                ExprBitwise::Expr(Expr::Term(Term::Factor(Factor::Number(12usize.into()))))
            ))
        )
    }

    #[test]
    fn test_parser_factor_paran() {
        assert_eq!(
            parse_factor(&[
                Token::LeftPar,
                Token::Number(12usize.into()),
                Token::Div,
                Token::Number(234usize.into()),
                Token::RightPar
            ]),
            Ok(Factor::Parenthesis(Box::new(Expr::Term(Term::Div(
                Box::new(Term::Factor(Factor::Number(12usize.into()))),
                Factor::Number(234usize.into())
            )))))
        )
    }

    #[test]
    fn test_parser_res_variable() {
        assert_eq!(
            parse_assignment(&[
                Token::Variable("a".to_string()),
                Token::Equals,
                Token::ResultVariable,
            ]),
            Ok(Assign::Assign(
                "a".to_string(),
                ExprBitwise::Expr(Expr::Term(Term::Factor(Factor::Variable(
                    RES_VAR.to_string()
                ))))
            ))
        )
    }

    #[test]
    fn test_parser_res_variable_lhs() {
        // Result variable cannot be at the left hand side of an assignment
        assert_eq!(
            parse_assignment(&[
                Token::ResultVariable,
                Token::Equals,
                Token::Variable("a".to_string()),
            ]),
            Err(CalcError::InvalidExpression)
        )
    }

    #[test]
    fn test_parser_negative_expr() {
        assert_eq!(
            parse_assignment(&[Token::Minus, Token::Variable("a".to_string()),]),
            Ok(Assign::ExprBitwise(ExprBitwise::Expr(Expr::Negative(
                Box::new(Expr::Term(Term::Factor(Factor::Variable("a".to_string()))))
            ))))
        )
    }

    #[test]
    fn test_parser_double_negative_expr() {
        assert_eq!(
            parse_assignment(&[Token::Minus, Token::Minus, Token::Variable("a".to_string()),]),
            Err(CalcError::InvalidExpression)
        )
    }
}
