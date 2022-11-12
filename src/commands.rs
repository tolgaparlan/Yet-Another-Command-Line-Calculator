use num_bigint::BigInt;
use phf::phf_map;
use std::{collections::HashMap, process::exit};

use crate::print_variable;

pub enum DisplayMode {
    Binary,
    Decimal,
    Hex,
}

pub struct RuntimeVariables {
    pub display_mode: DisplayMode,
    pub vars: HashMap<String, BigInt>,
}

pub static COMMANDS: phf::Map<&'static str, fn(&mut RuntimeVariables)> = phf_map! {
    "exit" => exit_function,
    "vars" => vars_print_function,
    "bin" => bin_display_mode,
    "dec" => dec_display_mode,
    "hex" => hex_display_mode,
    "clear" => clear_function,
};

fn exit_function(_settings: &mut RuntimeVariables) {
    exit(0);
}

fn vars_print_function(runtime_vars: &mut RuntimeVariables) {
    for var in runtime_vars.vars.keys() {
        print_variable(runtime_vars, var);
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

fn clear_function(runtime_vars: &mut RuntimeVariables) {
    runtime_vars.vars.clear();
    // https://stackoverflow.com/a/62101709/7611589
    println!("\x1B[2J\x1B[1;1H");
}
