use num_bigint::BigInt;
use phf::phf_map;
use std::{collections::HashMap, process::exit};

pub struct RuntimeVariables {
    pub vars: HashMap<String, BigInt>,
}

pub static SPECIAL_FUNCTIONS: phf::Map<&'static str, fn(&mut RuntimeVariables)> = phf_map! {
    "exit" => exit_function,
    "vars" => vars_print_function,
};

fn exit_function(_settings: &mut RuntimeVariables) {
    exit(0);
}

fn vars_print_function(settings: &mut RuntimeVariables) {
    for (var, val) in settings.vars.iter() {
        println!("\\> {} = {}", var, val);
    }
}
