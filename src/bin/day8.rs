use advent_of_code::computer;
use advent_of_code::computer::{ProgramState, single_step, Instruction, Program};
use std::collections::HashSet;

pub fn main() {
    let program = computer::read_from_file("input");

    // part 1
    // collect list of executed instructions for part 2, the faulty one must be one of them
    let mut state = ProgramState::new();
    let mut executed: HashSet<usize> = HashSet::new();
    while executed.insert(state.pc) {
        state = single_step(&program, &state);
    }

    println!("Program acc before running an instructions twice is {}", state.acc);

    // part 2
    println!("{}", executed.len());
    for inst_index in executed {
        let mut program: Program = program.to_vec();

        program[inst_index] = match program[inst_index] {
            // flip Nop and Jmp instructions
            Instruction::Nop(arg) => Instruction::Jmp(arg),
            Instruction::Jmp(arg) => Instruction::Nop(arg),
            _ => { continue; }
        };

        // execute the program
        let mut state = ProgramState::new();
        let mut executed: HashSet<usize> = HashSet::new();

        let terminated = loop {
            if state.terminated(&program) { break true; }
            if !executed.insert(state.pc) { break false; }

            state = single_step(&program, &state);
        };

        // if it terminated, print the resulting acc
        if terminated {
            println!("Part 2 terminated with acc {}", state.acc);
            break;
        }
    };
}

