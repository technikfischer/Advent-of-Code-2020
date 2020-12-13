use std::fs;
use itertools::{Itertools};

fn main() {
    let lines = fs::read_to_string("input").expect("Could not read file");
    let (departure, busses) = lines.lines().collect_tuple::<(&str, &str)>().unwrap();
    let departure: isize = departure.parse().expect("Could not parse departure time");
    let busses = busses.split(',')
        .filter(|s| *s != "x")
        .map(|s| s.parse::<isize>().expect("Could not parse bus id"))
        .sorted()
        .collect_vec();

    println!("Dep : {} Busses: {:?}", departure, busses);

    // part 1
    let first_available_bus = busses.iter().copied().min_by_key(|&id| (id - departure).rem_euclid(id)).unwrap();
    let waiting_time =  (first_available_bus - departure).rem_euclid(first_available_bus);
    println!("Waiting time * bus id = {}", waiting_time * first_available_bus);
}