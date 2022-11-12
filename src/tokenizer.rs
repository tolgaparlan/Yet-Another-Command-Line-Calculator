use std::{iter::Peekable, str::FromStr};

use num_bigint::BigUint;
use num_traits::Num;

use crate::{error::CalcError, parser::RES_VAR, special_function::SPECIAL_FUNCTIONS};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Token {
    Number(BigUint),
    Plus,
    Minus,
    Mult,
    Div,
    LeftPar,
    RightPar,
    Equals,
    Variable(String),
    ResultVariable, // Special variable `$` to store the result of the last operation
}

/// Makes a list of tokens from given string. Can fail given unrecognised characters
pub fn tokenize(line: &str) -> Result<Vec<Token>, crate::error::CalcError> {
    let mut it = line.chars().enumerate().peekable();
    let mut tokens = vec![];

    while let Some((index, c)) = it.next() {
        let token = match c {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Mult,
            '/' => Token::Div,
            '(' => Token::LeftPar,
            ')' => Token::RightPar,
            '=' => Token::Equals,
            '0' => {
                // Consume a hex or binary number
                match it.peek() {
                    // don't forget the case the user just entered number 0
                    None => Token::Number(BigUint::from(0usize)),
                    Some((_, x)) if !x.is_ascii_alphanumeric() => {
                        Token::Number(BigUint::from(0usize))
                    }
                    Some((_, 'x')) | Some((_, 'b')) => {
                        let radix = match it.next() {
                            Some((_, 'x')) => 16,
                            Some((_, 'b')) => 2,
                            _ => panic!(), // This should not happen since above it is checked
                        };
                        match BigUint::from_str_radix(&consume_alphanumeric(&mut it, None), radix) {
                            Ok(n) => Token::Number(n),
                            Err(_) => {
                                return Err(CalcError::InvalidToken(index));
                            }
                        }
                    }
                    _ => return Err(CalcError::InvalidToken(index)),
                }
            }
            RES_VAR => Token::ResultVariable,
            c if c.is_ascii_digit() => {
                // Consume a regular number token (i.e. not binary or hex).
                // Numbers cannot start with 0.
                let Ok(n) = BigUint::from_str(&consume_alphanumeric(&mut it, Some(&c.to_string()))) else {
                    return Err(CalcError::InvalidToken(index));
                };
                Token::Number(n)
            }
            c if c.is_ascii_alphabetic() => {
                // Consume a variable name.
                // Must start with a letter but then can contain numbers
                let var = consume_alphanumeric(&mut it, Some(&c.to_string()));

                // Cannot use a special function name for a variable
                if SPECIAL_FUNCTIONS.contains_key(&var) {
                    return Err(CalcError::SpecialVariableInvalidUse(var));
                }

                Token::Variable(var)
            }
            c if c.is_whitespace() => {
                continue;
            }
            _ => {
                return Err(CalcError::InvalidToken(index));
            }
        };

        tokens.push(token);
    }

    Ok(tokens)
}

/// Consumes all the alphanumeric characters from the given iterator and
/// returns as a string. Doesn't consume any succeeding non-alphanumeric character.
/// The string is built with the given prefix if one was passed.
fn consume_alphanumeric<I>(it: &mut Peekable<I>, prefix: Option<&str>) -> String
where
    I: Iterator<Item = (usize, char)>,
{
    let mut digits = match prefix {
        Some(p) => String::from(p),
        None => String::new(),
    };

    while let Some((_, peeked_char)) = it.peek() {
        if !peeked_char.is_alphanumeric() {
            break;
        }
        digits.push(*peeked_char);
        it.next();
    }

    digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_basic() {
        assert_eq!(
            tokenize("1+123*(12/234)"),
            Ok(vec![
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
        );
    }

    #[test]
    fn test_tokenize_variable() {
        assert_eq!(
            tokenize("asd=sdf+(ghj/2)"),
            Ok(vec![
                Token::Variable("asd".to_string()),
                Token::Equals,
                Token::Variable("sdf".to_string()),
                Token::Plus,
                Token::LeftPar,
                Token::Variable("ghj".to_string()),
                Token::Div,
                Token::Number(BigUint::from(2usize)),
                Token::RightPar
            ])
        );
    }

    #[test]
    fn test_tokenize_invalid_number() {
        assert_eq!(tokenize("1+1asd*(12/234)"), Err(CalcError::InvalidToken(2)));
    }

    #[test]
    fn test_tokenize_invalid_character() {
        assert_eq!(tokenize("1+asd;*(12/234)"), Err(CalcError::InvalidToken(5)));
    }

    #[test]
    fn test_tokenize_variable_with_digit() {
        assert_eq!(
            tokenize("asdf1a"),
            Ok(vec![Token::Variable("asdf1a".to_string())])
        );
    }

    #[test]
    fn test_tokenize_res_variable() {
        assert_eq!(
            tokenize(&RES_VAR.to_string()),
            Ok(vec![Token::ResultVariable])
        )
    }

    #[test]
    fn test_tokenize_zero() {
        // Make sure the hex/binary logic still allows a normal zero
        assert_eq!(
            tokenize("0"),
            Ok(vec![Token::Number(BigUint::from(0usize))])
        )
    }

    #[test]
    fn test_tokenize_binary() {
        assert_eq!(
            tokenize("0b1100"),
            Ok(vec![Token::Number(BigUint::from(0b1100usize))])
        )
    }

    #[test]
    fn test_tokenize_hex() {
        assert_eq!(
            tokenize("0xAAAA"),
            Ok(vec![Token::Number(BigUint::from(0xAAAAusize))])
        )
    }

    #[test]
    fn test_tokenize_invalid_hex() {
        assert_eq!(tokenize("0x"), Err(CalcError::InvalidToken(0)))
    }

    #[test]
    fn test_consume_alphanumeric_prefix() {
        assert!(consume_alphanumeric(&mut std::iter::empty().peekable(), None).is_empty());
        assert_eq!(
            &consume_alphanumeric(&mut std::iter::empty().peekable(), Some("asdf")),
            "asdf"
        );
        assert_eq!(
            &consume_alphanumeric(
                &mut String::from("123").chars().enumerate().peekable(),
                Some("asdf")
            ),
            "asdf123"
        );
    }

    #[test]
    fn test_consume_alphanumeric() {
        // ignore non-alphanumeric
        assert_eq!(
            &consume_alphanumeric(
                &mut String::from("123*").chars().enumerate().peekable(),
                None
            ),
            "123"
        );
    }

    #[test]
    fn test_tokenize_special_function() {
        // Special function names cannot be used as variables
        // so tokenizer should reject them
        assert_eq!(
            tokenize("exit"),
            Err(CalcError::SpecialVariableInvalidUse(String::from("exit")))
        )
    }
}
