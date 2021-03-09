use std::process::exit;
use crate::tokenizer::tokenize;
use crate::parser::parse;

mod tokenizer;
mod parser;
mod evaluater;

fn main() {
    let input = std::io::stdin();

    let mut line = String::new();
    if let Err(e) = input.read_line(&mut line) {
        eprintln!("{}", e);
        exit(1);
    };

    let tokens = tokenize(String::from(line.trim())).unwrap_or_else(|err| {
        eprintln!("{}", err);
        exit(1);
    });

    let expr = parse(tokens).unwrap_or_else(|err| {
        eprintln!("{}", err);
        exit(1);
    });
}