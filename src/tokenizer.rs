use std::str::FromStr;

use num_bigint::BigUint;

use crate::error::ArithmeticError;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Token {
    Number(BigUint),
    Plus,
    Minus,
    Mult,
    Div,
    LeftPar,
    RightPar,
}

/// Makes a list of tokens from given string. Can fail given unrecognised characters
pub fn tokenize(line: &str) -> Result<Vec<Token>, crate::error::ArithmeticError> {
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
            c if c.is_ascii_digit() => {
                // Consume a number token
                let mut digits = String::from(c);

                // peek while searching the boundary of the number
                while let Some((_, peeked_char)) = it.peek() {
                    if !peeked_char.is_ascii_digit() {
                        break;
                    }
                    digits.push(*peeked_char);
                    it.next();
                }
                let Ok(n) = BigUint::from_str(&digits) else {
                    return Err(ArithmeticError::InvalidToken(index));
                };
                Token::Number(n)
            }
            c if c.is_whitespace() => {
                continue;
            }
            _ => {
                return Err(ArithmeticError::InvalidToken(index));
            }
        };

        tokens.push(token);
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(
            tokenize("1+123*(12/234)").unwrap(),
            vec![
                Token::Number(1usize.into()),
                Token::Plus,
                Token::Number(123usize.into()),
                Token::Mult,
                Token::LeftPar,
                Token::Number(12usize.into()),
                Token::Div,
                Token::Number(234usize.into()),
                Token::RightPar
            ]
        );
    }

    #[test]
    fn test_error_string() {
        assert_eq!(
            tokenize("1+asd*(12/234)"),
            Err(ArithmeticError::InvalidToken(2))
        );
    }
}
