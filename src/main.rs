use parser::parse_expr;

use crate::evaluator::eval_expr;
use crate::tokenizer::tokenize;
use std::process::exit;

mod evaluator;
mod parser;
mod tokenizer;

fn main() {
    let input = std::io::stdin();

    loop {
        let mut line = String::new();
        if let Err(e) = input.read_line(&mut line) {
            eprintln!("Input Error: {}", e);
            exit(1);
        };
        let line_trimmed = line.trim();

        match tokenize(line_trimmed) {
            Ok(tokens) => match parse_expr(&tokens) {
                Ok(expr) => match eval_expr(expr) {
                    Ok(res) => println!("\\> {}", res),
                    Err(err) => eprintln!("{}", err),
                },
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
