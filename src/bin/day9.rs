use std::fs;
use itertools::Itertools;

fn valid_number_slice(slice: &[u64]) -> bool {
    slice[0..25].iter()
        .combinations(2)
        .map(|f| f[0] + f[1])
        .any(|f| f == slice[25])
}

fn main() {
    let lines = fs::read_to_string("input").expect("Could not open input file");
    let numbers = lines.lines()
        .map(|line| line.parse::<u64>().expect("Cannot parse integer"))
        .collect_vec();

    // part 1
    let mut invalid_number = 0;
    for window_start in 0..(numbers.len() - 25)
    {
        if !valid_number_slice(&numbers[window_start..window_start + 26]) {
            invalid_number = numbers[window_start + 25];
            println!("Pos 26 of invalid slice {}", invalid_number);
            break;
        }
    }

    // part 2
    'outer: for start in 0..numbers.len() {
        for end in (start + 1)..numbers.len() {
            let sum: u64 = numbers[start..=end].iter().sum();
            if sum == invalid_number {
                let min = numbers[start..end].iter().min().unwrap();
                let max = numbers[start..end].iter().max().unwrap();
                println!("Found weakness to be {}, length {}", min + max, end - start + 1);
                break 'outer;
            }
        }
    }
}
