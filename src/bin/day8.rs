use advent_of_code::computer;
use advent_of_code::computer::{ProgramState, single_step};

pub fn main() {
    let program = computer::read_from_file("input");
    let mut program_state = ProgramState::new();

    //part 1
    let mut already_executed = vec![false; program.len()];
    while !already_executed[program_state.pc] {
        already_executed[program_state.pc] = true;
        program_state = single_step(&program, &program_state);
    }

    print!("Program Acc before running an isntruction twice is {}", program_state.acc);
}

