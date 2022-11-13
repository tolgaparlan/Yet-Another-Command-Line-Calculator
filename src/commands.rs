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
    "exit" => exit_command,
    "vars" => vars_print_command,
    "bin" => bin_display_mode_command,
    "dec" => dec_display_mode_command,
    "hex" => hex_display_mode_command,
    "clear" => clear_function_command,
};

fn exit_command(_settings: &mut RuntimeVariables) {
    exit(0);
}

fn vars_print_command(runtime_vars: &mut RuntimeVariables) {
    for var in runtime_vars.vars.keys() {
        print_variable(runtime_vars, var);
    }
}

fn dec_display_mode_command(runtime_vars: &mut RuntimeVariables) {
    runtime_vars.display_mode = DisplayMode::Decimal;
}

fn hex_display_mode_command(runtime_vars: &mut RuntimeVariables) {
    runtime_vars.display_mode = DisplayMode::Hex;
}

fn bin_display_mode_command(runtime_vars: &mut RuntimeVariables) {
    runtime_vars.display_mode = DisplayMode::Binary;
}

fn clear_function_command(runtime_vars: &mut RuntimeVariables) {
    runtime_vars.vars.clear();
    // https://stackoverflow.com/a/62101709/7611589
    println!("\x1B[2J\x1B[1;1H");
}
