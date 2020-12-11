use std::fs;
use itertools::Itertools;


type SeatType = usize;

const FLOOR: SeatType = 0;
const EMPTY: SeatType = 1;
const OCCUPIED: SeatType = 2;

type Counts = [usize; 3];

fn line_to_vector(line: &str) -> Vec<SeatType> {
    line.chars().map(|c| match c {
        '.' => FLOOR,
        'L' => EMPTY,
        '#' => OCCUPIED,
        _ => unreachable!()
    }).collect()
}

fn get_seat_type(seat_layout: &SeatLayout, r: isize, c: isize) -> Option<SeatType> {
    if r < 0 || c < 0 || r as usize >= seat_layout.len() || c as usize >= seat_layout[0].len() {
        None
    } else {
        Some(seat_layout[r as usize][c as usize])
    }
}

fn count_neighbours_adjacent(seat_layout: &SeatLayout, r: isize, c: isize) -> Counts {
    let mut counts: Counts = [0, 0, 0];
    for rd in -1..=1 {
        for cd in -1..=1 {
            if rd == 0 && cd == 0 { continue; }
            if let Some(seat_type) = get_seat_type(&seat_layout, r + rd, c + cd) {
                counts[seat_type] += 1;
            }
        }
    }
    counts
}

fn count_neighbours_visible(seat_layout: &SeatLayout, r: isize, c: isize) -> Counts {
    let mut counts: Counts = [0, 0, 0];
    for rd in -1..=1 {
        for cd in -1..=1 {
            if rd == 0 && cd == 0 { continue; }
            let mut a = 1;
            's: loop {
                match get_seat_type(&seat_layout, r + a * rd, c + a * cd) {
                    Some(st) if st == EMPTY || st == OCCUPIED => {
                        counts[st] += 1;
                        break 's;
                    }
                    Some(_) => {}
                    None => { break 's; }
                }
                a += 1;
            }
        }
    }
    counts
}

fn next_state_part1(seat_layout: &SeatLayout) -> (SeatLayout, bool) {
    let mut changed = false;
    let new_state =
        (0isize..seat_layout.len() as isize).map(|row|
            (0isize..seat_layout[0].len() as isize).map(|col| {
                let counts = count_neighbours_adjacent(&seat_layout, row, col);
                let seat = seat_layout[row as usize][col as usize];
                if seat == EMPTY && counts[OCCUPIED] == 0 {
                    changed = true;
                    OCCUPIED
                } else if seat == OCCUPIED && counts[OCCUPIED] >= 4 {
                    changed = true;
                    EMPTY
                } else {
                    seat
                }
            }).collect_vec()
        ).collect_vec();

    (new_state, changed)
}

fn next_state_part2(seat_layout: &SeatLayout) -> (SeatLayout, bool) {
    let mut changed = false;
    let new_state =
        (0isize..seat_layout.len() as isize).map(|row|
            (0isize..seat_layout[0].len() as isize).map(|col| {
                let counts = count_neighbours_visible(&seat_layout, row, col);
                let seat = seat_layout[row as usize][col as usize];
                if seat == EMPTY && counts[OCCUPIED] == 0 {
                    changed = true;
                    OCCUPIED
                } else if seat == OCCUPIED && counts[OCCUPIED] >= 5 {
                    changed = true;
                    EMPTY
                } else {
                    seat
                }
            }).collect_vec()
        ).collect_vec();

    (new_state, changed)
}

fn print_seat_layout(seat_layout: &SeatLayout) {
    for row in seat_layout {
        for c in row {
            print!("{}", match *c {
                FLOOR => '.',
                EMPTY => 'L',
                OCCUPIED => '#',
                _ => unreachable!()
            });
        }
    }
}

type SeatLayout = Vec<Vec<SeatType>>;

fn main() {
    let data = fs::read_to_string("input").expect("Unable to read file");
    let seat_layout: SeatLayout = data.lines()
        .map(line_to_vector)
        .collect();

    // part 1
    let mut new_seat_layout = seat_layout.clone();
    loop {
        let (new_state, changed) = next_state_part1(&new_seat_layout);
        new_seat_layout = new_state;
        if !changed {
            break;
        }
    }

    let occupied_seats: usize = new_seat_layout.iter().map(|row| row.iter().filter(|c| **c == OCCUPIED).count()).sum();
    println!("Occupied seats part 1 {}", occupied_seats);

    // part 2
    let mut simulated = seat_layout.clone();
    loop {
        let (new_state, changed) = next_state_part2(&simulated);
        simulated = new_state;
        if !changed {
            break;
        }
    }

    let occupied_seats: usize = simulated.iter().map(|row| row.iter().filter(|c| **c == OCCUPIED).count()).sum();
    println!("Occupied seats part 2 {}", occupied_seats);
}