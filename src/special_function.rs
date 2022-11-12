use num_bigint::BigInt;
use phf::phf_map;
use std::{collections::HashMap, process::exit};

pub enum DisplayMode {
    Binary,
    Decimal,
    Hex,
}

pub struct RuntimeVariables {
    pub display_mode: DisplayMode,
    pub vars: HashMap<String, BigInt>,
}

pub static SPECIAL_FUNCTIONS: phf::Map<&'static str, fn(&mut RuntimeVariables)> = phf_map! {
    "exit" => exit_function,
    "vars" => vars_print_function,
    "bin" => bin_display_mode,
    "dec" => dec_display_mode,
    "hex" => hex_display_mode,
};

fn exit_function(_settings: &mut RuntimeVariables) {
    exit(0);
}

fn vars_print_function(runtime_vars: &mut RuntimeVariables) {
    for (var, val) in runtime_vars.vars.iter() {
        print!("\\> ");
        match runtime_vars.display_mode {
            DisplayMode::Binary => println!("{} = 0b{:b}", var, val),
            DisplayMode::Decimal => println!("{} = {}", var, val),
            DisplayMode::Hex => println!("{} = 0x{:X}", var, val),
        }
    }
}

fn dec_display_mode(runtime_vars: &mut RuntimeVariables) {
    runtime_vars.display_mode = DisplayMode::Decimal;
}

fn hex_display_mode(runtime_vars: &mut RuntimeVariables) {
    runtime_vars.display_mode = DisplayMode::Hex;
}

fn bin_display_mode(runtime_vars: &mut RuntimeVariables) {
    runtime_vars.display_mode = DisplayMode::Binary;
}
