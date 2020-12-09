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
            println!("Pos 26 of invalid slice {} @ {}", invalid_number, window_start);
            break;
        }
    }

    // part 2
    let mut left = 0;
    let mut right = 1;
    let mut sum = numbers[0];

    loop {
        while sum < invalid_number /*&& right < numbers.len()*/ {
            sum += numbers[right];
            right += 1;
        }

        while sum > invalid_number /*&& left < numbers.len()*/ {
            sum -= numbers[left];
            left += 1;
        }

        if sum == invalid_number {
            let min = numbers[left..=right].iter().min().unwrap();
            let max = numbers[left..=right].iter().max().unwrap();
            println!("Found weakness to be {}, length {}, {} -> {}", min + max, right - left + 1, left, right);
            break;
        }
    }
}
