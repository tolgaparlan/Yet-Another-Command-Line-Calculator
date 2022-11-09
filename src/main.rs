use evaluator::eval_assignment;
use num_bigint::BigInt;
use parser::parse_assignment;

use crate::tokenizer::tokenize;
use std::collections::HashMap;
use std::process::exit;

mod error;
mod evaluator;
mod parser;
mod tokenizer;

fn main() {
    let input = std::io::stdin();
    let mut variables: HashMap<String, BigInt> = HashMap::new();

    loop {
        let mut line = String::new();
        if let Err(e) = input.read_line(&mut line) {
            eprintln!("Input Error: {}", e);
            exit(1)
        };
        let line_trimmed = line.trim();

        match line_trimmed {
            "exit" => exit(0),
            "vars" => {
                for (var, val) in variables.iter() {
                    println!(" \\>{} = {}", var, val);
                }
            }
            tokens => {
                if let Err(err) = tokenize(tokens)
                    .and_then(|tokens| parse_assignment(&tokens))
                    .and_then(|expr| eval_assignment(expr, &mut variables))
                    .map(|res| {
                        println!("\\> {}", res);
                    })
                {
                    eprintln!("{}", err);
                }
            }
        }
    }
}
