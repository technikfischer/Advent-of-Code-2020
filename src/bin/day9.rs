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
    let mut numbers = lines.lines()
        .map(|line| line.parse::<u64>().expect("Cannot parse integer"));

    // part 1
    let mut v: Vec<u64> = Vec::new();
    for i in 0..25 {
        v.push(numbers.next().expect("Not enough numbers in vector"))
    }

    while let Some(e) = numbers.next()
    {
        v.push(e);
        if !valid_number_slice(&v[..]) {
            println!("Pos 26 of invalid slice {}", v[25])
        }
        v.remove(0);
    }
}
