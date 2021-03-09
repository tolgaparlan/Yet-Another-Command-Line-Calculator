use std::process::exit;
use crate::tokenizer::{tokenize, TokenizeError, Token};

mod tokenizer;

fn main() {
    let input = std::io::stdin();

    let mut line = String::new();
    if let Err(e) = input.read_line(&mut line) {
        eprintln!("{}", e);
        exit(1);
    };

    let tokens = match tokenize(String::from(line.trim())) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };
}