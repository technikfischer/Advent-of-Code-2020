#[macro_use]
extern crate lazy_static;

use regex::{Regex};
use std::str::FromStr;
use crate::Instruction::{SetMask, WriteVal};
use advent_of_code::get_input;
use std::collections::HashMap;

enum Instruction {
    SetMask(String),
    WriteVal { addr: u64, val: String },
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
        static ref SET_MASK_PATTERN: Regex = Regex::new(r"mask = (?P<mask>[01X]{36})").unwrap();
        static ref WRITE_MEMORY_PATTERN: Regex = Regex::new(r"mem\[(?P<addr>\d+)\] = (?P<value>\d+)").unwrap();
        }

        if let Some(c) = SET_MASK_PATTERN.captures(s) {
            Ok(SetMask(c["mask"].to_owned()))
        } else if let Some(c) = WRITE_MEMORY_PATTERN.captures(s) {
            let addr = c["addr"].parse().expect("Could not parse address of memory instruction");
            let val: u64 = c["value"].parse().unwrap();
            Ok(WriteVal { addr, val: format!("{:036b}", val) })
        } else {
            Err(s.to_owned())
        }
    }
}

fn main() {
    let instructions: Vec<Instruction> = get_input();

    // part 1
    let mut mask = String::new();
    let mut memory: HashMap<u64, String> = HashMap::new();

    for inst in instructions {
        match inst {
            SetMask(m) => mask = m,
            Instruction::WriteVal { addr, val } => {
                let newval = mask.chars()
                    .zip(val.chars())
                    .map(|(m, v)| if m == 'X' { v } else { m })
                    .collect::<String>();

                if u64::from_str_radix(&newval, 2).unwrap() == 0 {
                    memory.remove(&addr);
                } else {
                    memory.insert(addr, newval);
                }
            }
        }
    }

    let sum: u64 = memory.values().map(|s| u64::from_str_radix(s, 2).unwrap()).sum();
    println!("Sum of all non-zero memory cells is {}", sum);
}