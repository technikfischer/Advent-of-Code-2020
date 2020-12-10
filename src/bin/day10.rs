use advent_of_code::get_input;
use counter::Counter;

fn main() {
    let mut adapters: Vec<usize> = get_input();
    adapters.push(0);
    adapters.push(adapters.iter().max().unwrap() + 3);
    adapters.sort_unstable();

    // part 1
    let joltage_differences = adapters
        .windows(2)
        .map(|w| w[1] - w[0])
        .collect::<Counter<_>>();

    println!("Joltage differences of 1 and 3 multiplied: {}", joltage_differences[&1] * joltage_differences[&3]);
}

