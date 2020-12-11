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

fn count_neighbours(seat_layout: &Vec<Vec<SeatType>>, r: usize, c: usize) -> Counts {
    let mut counts: Counts = [0, 0, 0];
    for rd in -1..=1 {
        for cd in -1..=1 {
            if rd == 0 && cd == 0 { continue; }
            if let Some(&seat_type) = seat_layout.get((r as isize + rd) as usize).and_then(|row| row.get((c as isize + cd) as usize)) {
                counts[seat_type] += 1;
            }
        }
    }
    counts
}

fn next_state(seat_layout: &SeatLayout) -> (SeatLayout, bool) {
    let mut changed = false;
    let new_state =
        (0..seat_layout.len()).map(|row|
            (0..seat_layout[0].len()).map(|col| {
                let counts = count_neighbours(&seat_layout, row, col);
                let seat = seat_layout[row][col];
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
        let (new_state, changed) = next_state(&new_seat_layout);
        new_seat_layout = new_state;
        if !changed {
            break;
        }
    }

    let occupied_seats: usize = new_seat_layout.iter().map(|row| row.iter().filter(|c| **c == OCCUPIED).count()).sum();
    print!("Occupied seats {}", occupied_seats);
}