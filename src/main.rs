use evaluator::eval_assignment;
use parser::parse_assignment;
use special_function::{RuntimeVariables, SPECIAL_FUNCTIONS};

use crate::tokenizer::tokenize;
use std::{collections::HashMap, process::exit};

mod error;
mod evaluator;
mod parser;
mod special_function;
mod tokenizer;

fn main() {
    let input = std::io::stdin();

    let mut runtime = RuntimeVariables {
        vars: HashMap::new(),
    };

    loop {
        let mut line = String::new();
        if let Err(e) = input.read_line(&mut line) {
            eprintln!("Input Error: {}", e);
            exit(1)
        };
        let line_trimmed = line.trim();

        match SPECIAL_FUNCTIONS.get(line_trimmed) {
            Some(f) => f(&mut runtime),
            None => {
                if let Err(err) = tokenize(line_trimmed)
                    .and_then(|tokens| parse_assignment(&tokens))
                    .and_then(|expr| eval_assignment(expr, &mut runtime.vars))
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
