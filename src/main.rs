use parser::parse_expr;

use crate::evaluator::eval_expr;
use crate::tokenizer::tokenize;
use std::process::exit;

mod evaluator;
mod parser;
mod tokenizer;

fn main() {
    let input = std::io::stdin();
    let mut line = String::new();

    loop {
        if let Err(e) = input.read_line(&mut line) {
            eprintln!("Input Error: {}", e);
            exit(1);
        };

        let line_trimmed = line.trim();

        match tokenize(line_trimmed) {
            Ok(tokens) => match parse_expr(&tokens) {
                Ok(expr) => println!("{}", eval_expr(expr)),
                Err(err) => {
                    eprintln!("{}", err);
                    continue;
                }
            },
            Err(err) => {
                eprintln!("{}", err);
                continue;
            }
        }
    }
}
