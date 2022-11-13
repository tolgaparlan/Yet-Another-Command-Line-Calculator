use commands::{DisplayMode, RuntimeVariables, COMMANDS};
use evaluator::eval_assignment;
use parser::parse_assignment;

use crate::tokenizer::tokenize;
use std::{collections::HashMap, process::exit};

mod commands;
mod error;
mod evaluator;
mod functions;
mod parser;
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

        match COMMANDS.get(line) {
            Some(f) => f(&mut runtime_vars),
            None => {
                if let Err(err) = tokenize(line)
                    .and_then(|tokens| parse_assignment(&tokens))
                    .and_then(|expr| eval_assignment(expr, &mut runtime_vars.vars))
                    .map(|var| {
                        // Print in correct display mode
                        print_variable(&runtime_vars, &var);
                    })
                {
                    eprintln!("{}", err);
                }
            }
        }
    }
}

/// Print the given variable with its value and correct display mode
fn print_variable(runtime_vars: &RuntimeVariables, var: &str) {
    let Some(val) = runtime_vars.vars.get(var) else {
        // This should never happen
        panic!("Tried to print non-existing variable {}. \nVariables:{:?}", var, runtime_vars.vars);
    };

    print!("\\> {} = ", var);
    match runtime_vars.display_mode {
        DisplayMode::Binary => println!("0b{:b}", val),
        DisplayMode::Decimal => println!("{}", val),
        DisplayMode::Hex => println!("0x{:X}", val),
    }
}
