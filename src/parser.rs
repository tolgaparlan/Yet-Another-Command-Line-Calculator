use crate::tokenizer::Token;
use std::error::Error;
use std::fmt;

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
    Number(u64),
    Parenthesis(Box<Expr>),
    Negative(Box<Factor>),
}

#[derive(PartialEq, Eq, Debug)]
pub struct ParserError {
    t: String,
}

impl Error for ParserError {}

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
                // iterate until the matching right parenthesis
                let mut left_count = 1;
                for (_, &t) in it.by_ref() {
                    left_count += match t {
                        Token::LeftPar => 1,
                        Token::RightPar => -1,
                        _ => 0,
                    };
                    if left_count == 0 {
                        break;
                    }
                }
                if left_count > 0 {
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
                // iterate until the matching right parenthesis
                let mut left_count = 1;
                for (_, &t) in it.by_ref() {
                    left_count += i32::from(t == Token::LeftPar);
                    left_count -= i32::from(t == Token::RightPar);
                    if left_count == 0 {
                        break;
                    }
                }
                if left_count > 0 {
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

    match it.next() {
        None => Err(ParserError {
            t: String::from("Expected Number"),
        }),
        Some(Token::Number(n)) if it.next().is_none() => Ok(Factor::Number(*n)),
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

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(
            parse_expr(&[
                Token::Number(1),
                Token::Plus,
                Token::Number(123),
                Token::Mult,
                Token::LeftPar,
                Token::Number(12),
                Token::Div,
                Token::Number(234),
                Token::RightPar
            ])
            .unwrap(),
            Expr::Sum(
                Box::from(Expr::Term(Term::Factor(Factor::Number(1)))),
                Term::Mult(
                    Box::from(Term::Factor(Factor::Number(123))),
                    Factor::Parenthesis(Box::from(Expr::Term(Term::Div(
                        Box::from(Term::Factor(Factor::Number(12))),
                        Factor::Number(234)
                    )))),
                ),
            )
        );
    }
}
