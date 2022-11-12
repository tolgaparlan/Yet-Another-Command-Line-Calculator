use evaluator::eval_assignment;
use parser::parse_assignment;
use special_function::{DisplayMode, RuntimeVariables, SPECIAL_FUNCTIONS};

use crate::tokenizer::tokenize;
use std::{collections::HashMap, process::exit};

mod error;
mod evaluator;
mod parser;
mod special_function;
mod tokenizer;

fn main() {
    let stdin = std::io::stdin();

    let mut runtime_vars = RuntimeVariables {
        vars: HashMap::new(),
        display_mode: DisplayMode::Decimal,
    };

    loop {
        let mut line = String::new();
        if stdin.read_line(&mut line).is_err() {
            eprintln!("Unexpected IO Error Occurred");
            exit(1)
        };
        let line = line.trim();

        match SPECIAL_FUNCTIONS.get(line) {
            Some(f) => f(&mut runtime_vars),
            None => {
                if let Err(err) = tokenize(line)
                    .and_then(|tokens| parse_assignment(&tokens))
                    .and_then(|expr| eval_assignment(expr, &mut runtime_vars.vars))
                    .map(|res| {
                        // Print in correct display mode
                        print!("\\> ");
                        match runtime_vars.display_mode {
                            DisplayMode::Binary => println!("0b{:b}", res),
                            DisplayMode::Decimal => println!("{}", res),
                            DisplayMode::Hex => println!("0x{:X}", res),
                        }
                    })
                {
                    eprintln!("{}", err);
                }
            }
        }
    }
}
