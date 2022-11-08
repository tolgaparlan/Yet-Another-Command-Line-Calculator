use std::fmt;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Token {
    Number(u64),
    Plus,
    Minus,
    Mult,
    Div,
    LeftPar,
    RightPar,
}

#[derive(PartialEq, Eq, Debug)]
pub struct TokenizeError {
    index: usize,
}

impl std::error::Error for TokenizeError {}

pub fn tokenize(line: &str) -> Result<Vec<Token>, TokenizeError> {
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
                let mut chars = vec![c];

                // peek while searching the boundary of the number
                while let Some((_, peeked_char)) = it.peek() {
                    if !peeked_char.is_ascii_digit() {
                        break;
                    }
                    chars.push(*peeked_char);
                    it.next();
                }
                let Ok(n) = chars.iter().collect::<String>().parse::<u64>() else {
                    return Err(TokenizeError { index });
                };
                Token::Number(n)
            }
            c if c.is_whitespace() => {
                continue;
            }
            _ => {
                return Err(TokenizeError { index });
            }
        };

        tokens.push(token);
    }

    Ok(tokens)
}

impl fmt::Display for TokenizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid token at index: {}", self.index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(
            tokenize("1+123*(12/234)").unwrap(),
            vec![
                Token::Number(1),
                Token::Plus,
                Token::Number(123),
                Token::Mult,
                Token::LeftPar,
                Token::Number(12),
                Token::Div,
                Token::Number(234),
                Token::RightPar
            ]
        );
    }

    #[test]
    fn test_error() {
        assert_eq!(tokenize("1+asd*(12/234)"), Err(TokenizeError { index: 2 }));
    }
}
