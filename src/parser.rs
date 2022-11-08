use crate::tokenizer::Token;
use num_bigint::BigUint;
use std::{error::Error, fmt};

#[derive(Debug, PartialEq)]
pub enum Expr {
    Sum(Box<Expr>, Term),
    Subtract(Box<Expr>, Term),
    Term(Term),
}

#[derive(Debug, PartialEq)]
pub enum Term {
    Mult(Box<Term>, Factor),
    Div(Box<Term>, Factor),
    Factor(Factor),
}

#[derive(Debug, PartialEq)]
pub enum Factor {
    Number(BigUint),
    Parenthesis(Box<Expr>),
    Negative(Box<Factor>),
}

pub fn parse_expr(tokens: &[Token]) -> Result<Expr, ParserError> {
    let mut it = tokens.iter().enumerate();

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
                    return Err(ParserError {
                        t: String::from("Expected `)`"),
                    });
                }
            }
            _ => {
                continue;
            }
        }
    }

    // reached the end of the expression without matching -> must be a term
    Ok(Expr::Term(parse_term(tokens)?))
}

fn parse_term(v: &[Token]) -> Result<Term, ParserError> {
    let mut it = v.iter().enumerate();

    loop {
        let t = &it.next();

        match t {
            None => {
                break;
            }
            Some((i, Token::Mult)) => {
                return Ok(Term::Mult(
                    Box::from(parse_term(&v[0..*i])?),
                    parse_factor(&v[i + 1..])?,
                ));
            }
            Some((i, Token::Div)) => {
                return Ok(Term::Div(
                    Box::from(parse_term(&v[0..*i])?),
                    parse_factor(&v[i + 1..])?,
                ));
            }
            Some((_, Token::LeftPar)) => {
                if !matching_paranthesis(it.by_ref()) {
                    return Err(ParserError {
                        t: String::from("Expected `)`"),
                    });
                }
            }
            Some(_) => {
                continue;
            }
        }
    }

    // reached the end of the expression without matching must be a factor
    Ok(Term::Factor(parse_factor(v)?))
}

fn parse_factor(v: &[Token]) -> Result<Factor, ParserError> {
    let mut it = v.iter();

    match &mut it.next() {
        None => Err(ParserError {
            t: String::from("Expected Number"),
        }),
        Some(Token::Number(n)) if it.next().is_none() => Ok(Factor::Number(n.clone())),
        Some(Token::Minus) => Ok(Factor::Negative(Box::from(parse_factor(&v[1..])?))),
        Some(Token::LeftPar) => {
            if let Some(Token::RightPar) = it.last() {
                Ok(Factor::Parenthesis(Box::from(parse_expr(
                    &v[1..v.len() - 1],
                )?)))
            } else {
                Err(ParserError {
                    t: String::from("Expected `)`"),
                })
            }
        }
        Some(_) => Err(ParserError {
            t: String::from("Expected Expression"),
        }),
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct ParserError {
    t: String,
}

impl Error for ParserError {}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.t)
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
    left_count > 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
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
            ])
            .unwrap(),
            Expr::Sum(
                Box::from(Expr::Term(Term::Factor(Factor::Number(1usize.into())))),
                Term::Mult(
                    Box::from(Term::Factor(Factor::Number(123usize.into()))),
                    Factor::Parenthesis(Box::from(Expr::Term(Term::Div(
                        Box::from(Term::Factor(Factor::Number(12usize.into()))),
                        Factor::Number(234usize.into())
                    )))),
                ),
            )
        );
    }
}
