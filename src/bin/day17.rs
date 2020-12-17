use std::collections::HashSet;
use std::fs;

type State = HashSet<(i32, i32, i32)>;

fn get_initial_state() -> State {
    let input = fs::read_to_string("input").expect("Could not read file");
    let mut state: State = HashSet::new();

    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c == '#' {
                state.insert((x as i32, y as i32, 0));
            }
        }
    }

    state
}

fn find_candidates_for_changing(state: &State) -> HashSet<(i32, i32, i32)> {
    let mut candidates_for_changing = HashSet::new();

    for (x, y, z) in state { // for each active state, all cubes inside the 3x3x3 cube around can change
        for x in x - 1..=x + 1 {
            for y in y - 1..=y + 1 {
                for z in z - 1..=z + 1 {
                    candidates_for_changing.insert((x, y, z));
                }
            }
        }
    }

    candidates_for_changing
}

fn count_neighbours(state: &State, (x, y, z): (i32, i32, i32)) -> u32 {
    let mut count = 0;
    for xi in x - 1..=x + 1 {
        for yi in y - 1..=y + 1 {
            for zi in z - 1..=z + 1 {
                if xi == x && yi == y && zi == z {
                    continue;
                }
                if state.contains(&(xi, yi, zi)) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn simulate_step(state: &State) -> State {
    let candidates = find_candidates_for_changing(state);
    let mut new_state = State::new();

    for c in candidates {
        let neighbour_count = count_neighbours(&state, c);

        match state.contains(&c) {
            true if neighbour_count == 2 || neighbour_count == 3 => { new_state.insert(c); }
            true => { /* become inactive */ }
            false if neighbour_count == 3 => { new_state.insert(c); }
            false => { /* remain inactive */ }
        }
    }

    new_state
}

fn main() {
    let mut state = get_initial_state();
    for _ in 0..6 {
        state = simulate_step(&state);
    }
    println!("Active after 6 iterations: {}", state.len());
}