use std::fs;
use itertools::Itertools;

pub struct ProgramState {
    pc: i32,
    acc: i32,
}

pub enum Instruction {
    acc(i32),
    nop(i32),
    jmp(i32),
}

pub type Program = Vec<Instruction>;

pub fn read_from_file(file: &str) -> Program {
    let file_content = fs::read_to_string(file).expect("Could not read file");

    let mut program = Program::new();
    for line in file_content.lines() {
        if let Some((opcode, argument)) = line.split(' ').collect_tuple::<(&str, &str)>() {
            // convert the argument to an integer
            let argument = argument.parse::<i32>().expect(format!("Could not parse argument {} of instruction {}", argument, line).as_str());

            let instruction = match (opcode, argument) {
                ("acc", arg) => Instruction::acc(arg),
                ("nop", arg) => Instruction::nop(arg),
                ("jmp", arg) => Instruction::jmp(arg),

                (opcode, _) => panic!("Invalid opcode {}", opcode)
            };

            program.push(instruction);
        } else {
            panic!("Could not parse instruction {}", line);
        }
    }

    return program;
}