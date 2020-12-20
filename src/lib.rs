use std::fs;
use std::str::FromStr;
use std::fmt::Debug;

pub mod computer;

pub fn get_input<F: FromStr>() -> Vec<F>
    where F: FromStr, <F as FromStr>::Err: Debug {
    let lines = fs::read_to_string("input").expect("Could not open file");
    lines.lines()
        .map(|line| line.parse::<F>().expect("Could not parse input element"))
        .collect::<Vec<F>>()
}

pub mod bitvector;