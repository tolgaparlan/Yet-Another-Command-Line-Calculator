use num_bigint::BigInt;
use parser::parse_expr;

use crate::evaluator::eval_expr;
use crate::tokenizer::tokenize;
use std::collections::HashMap;
use std::process::exit;

mod error;
mod evaluator;
mod parser;
mod tokenizer;

fn main() {
    let input = std::io::stdin();
    let mut variables: HashMap<&str, BigInt> = HashMap::new();

    loop {
        let mut line = String::new();
        if let Err(e) = input.read_line(&mut line) {
            eprintln!("Input Error: {}", e);
            exit(1)
        };
        let line_trimmed = line.trim();

        if line_trimmed == "exit" {
            exit(0)
        }

        if let Err(err) = tokenize(line_trimmed)
            .and_then(|tokens| parse_expr(&tokens))
            .and_then(|expr| eval_expr(expr, &mut variables))
            .map(|res| {
                println!("\\> {}", res);
            })
        {
            eprintln!("{}", err);
        }
    }
}
