#[macro_use]
extern crate lazy_static;

use regex::{Regex};
use std::str::FromStr;
use crate::Instruction::{SetMask, WriteVal};
use advent_of_code::get_input;
use std::collections::HashMap;

enum Instruction {
    SetMask(String),
    WriteVal { addr: String, val: String },
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
            let addr: u64 = c["addr"].parse().expect("Could not parse address of memory instruction");
            let val: u64 = c["value"].parse().expect("Could not parse value of memory instruction");
            Ok(WriteVal { addr: format!("{:036b}", addr), val: format!("{:036b}", val) })
        } else {
            Err(s.to_owned())
        }
    }
}

fn write_val(memory: &mut HashMap<String, String>, pos: isize, addr: String, val: &String) {
    if pos < 0 {
        memory.insert(addr, val.to_owned());
        return;
    } else {
        if addr.chars().nth(35 - pos as usize).unwrap() != 'X' {
            write_val(memory, pos - 1, addr, val);
        } else {
            write_val(memory, pos - 1,  addr.replacen('X', "0", 1), val);
            write_val(memory, pos - 1,  addr.replacen('X', "1", 1), val);
        }
    }
}

fn main() {
    let instructions: Vec<Instruction> = get_input();

    // part 1
    let mut mask = String::new();
    let mut memory: HashMap<String, String> = HashMap::new();

    for inst in instructions.iter() {
        match inst {
            SetMask(m) => mask = m.to_string(),
            Instruction::WriteVal { addr, val } => {
                let newval = mask.chars()
                    .zip(val.chars())
                    .map(|(m, v)| if m == 'X' { v } else { m })
                    .collect::<String>();

                memory.insert(addr.to_string(), newval);
            }
        }
    }

    let sum: u64 = memory.values().map(|s| u64::from_str_radix(s, 2).unwrap()).sum();
    println!("Sum of all non-zero memory cells (part 1) is {}", sum);

    // part 2
    let mut mask = String::new();
    let mut memory: HashMap<String, String> = HashMap::new();

    for inst in instructions {
        match inst {
            SetMask(m) => mask = m,
            Instruction::WriteVal { addr, val } => {
                let mask_applied = mask.chars()
                    .zip(addr.chars())
                    .map(|(m, v)| if m == '0' { v } else { m })
                    .collect::<String>();

                //println!("addr: {} val: {} cur mask: {} mask @ addr: {}", addr, val, mask, mask_applied);
                write_val(&mut memory, 35, mask_applied, &val);
            }
        }
    }

    let sum: u64 = memory.values().map(|s| u64::from_str_radix(s, 2).unwrap()).sum();
    println!("Sum of all non-zero memory cells (part2) is {}", sum);
}