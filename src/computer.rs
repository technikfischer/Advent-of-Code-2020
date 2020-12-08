use std::fs;
use itertools::Itertools;

#[derive(Copy, Clone)]
pub struct ProgramState {
    pub pc: usize,
    pub acc: isize,
}

impl ProgramState {
    pub fn new() -> ProgramState {
        ProgramState {
            pc: 0,
            acc: 0,
        }
    }
}

pub enum Instruction {
    Acc(isize),
    Nop(isize),
    Jmp(isize),
}

pub type Program = Vec<Instruction>;

pub fn read_from_file(file: &str) -> Program {
    let file_content = fs::read_to_string(file).expect("Could not read file");

    let mut program = Program::new();
    for line in file_content.lines() {
        if let Some((opcode, argument)) = line.split(' ').collect_tuple::<(&str, &str)>() {
            // convert the argument to an integer
            let argument = argument.parse::<isize>().expect(format!("Could not parse argument {} of instruction {}", argument, line).as_str());

            let instruction = match (opcode, argument) {
                ("Acc", arg) => Instruction::Acc(arg),
                ("Nop", arg) => Instruction::Nop(arg),
                ("Jmp", arg) => Instruction::Jmp(arg),

                (opcode, _) => panic!("Invalid opcode {}", opcode)
            };

            program.push(instruction);
        } else {
            panic!("Could not parse instruction {}", line);
        }
    }

    return program;
}

pub fn single_step(program: &Program, &state: &ProgramState) -> ProgramState {
    let instruction: &Instruction = program.get(state.pc).expect(format!("Invalid program address {}", state.pc).as_str());

    match instruction {
        Instruction::Acc(arg) => ProgramState {
            acc: state.acc + arg,
            pc: state.pc + 1,
        },

        Instruction::Nop(_) => ProgramState {
            pc: state.pc + 1,
            ..state
        },

        Instruction::Jmp(arg) => ProgramState {
            //https://www.reddit.com/r/rust/comments/3mcwf7/adding_unsigned_and_signed_integers/cve0p1w?utm_source=share&utm_medium=web2x&context=3
            pc: (if *arg < 0 { state.pc - (-arg) as usize } else { state.pc + *arg as usize }),
            ..state
        }
    }
}