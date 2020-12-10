use advent_of_code::get_input;
use counter::Counter;
use itertools::Itertools;

fn main() {
    let mut adapters: Vec<usize> = get_input();
    let device_joltage = adapters.iter().max().unwrap() + 3;
    adapters.push(0);
    adapters.push(device_joltage);
    adapters.sort_unstable();

    // part 1
    let joltage_differences = adapters
        .windows(2)
        .map(|w| w[1] - w[0])
        .collect::<Counter<_>>();

    println!("Joltage differences of 1 and 3 multiplied: {}", joltage_differences[&1] * joltage_differences[&3]);

    let differences = adapters
        .windows(2) // 3
        .map(|w| w[1] - w[0])// w[2]
        .collect_vec();

    let arrangements = differences.split(|d| *d == 3).map(combinations).fold(1, |a, b| a * b);
    println!("Possible arrangements {}", arrangements);
}

fn combinations(differences: &[usize]) -> i64 {
    match differences {
        [] => 1,
        [3, ..] => combinations(&differences[1..]),
        [1, 1, 1, ..] => combinations(&differences[1..]) + combinations(&differences[2..]) + combinations(&differences[3..]),
        [1, 2, ..] | [2, 1, ..] | [1, 1, ..] => combinations(&differences[1..]) + combinations(&differences[2..]),
        _ => combinations(&differences[1..])
    }
}