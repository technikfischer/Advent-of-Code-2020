use std::fs;
use std::collections::HashSet;
use reduce::Reduce;

fn union_of_answers(group_answers: &str) -> HashSet<char> {
    group_answers.chars()
        .filter(|c| c.is_alphabetic())
        .collect::<HashSet<char>>()
}

fn intersection_of_answers(group_answers: &str) -> HashSet<char> {
    group_answers.lines()
        .map(|line| line.chars().collect::<HashSet<_>>())
        .reduce(|a, b| a.intersection(&b).map(|c| *c).collect())
        .unwrap()
}

fn main() {
    let data = fs::read_to_string("input").expect("Unable to read file");

    //part 1
    let total_sum_of_yes: usize = data.split("\n\n")
        .map(union_of_answers)
        .map(|hs| hs.len())
        .sum();

    println!("Sum of single groups is {}", total_sum_of_yes);

    //part 2
    let total_sum_of_yes: usize = data.split("\n\n")
        .map(intersection_of_answers)
        .map(|hs| hs.len())
        .sum();

    println!("Sum of answers answered with yes from anyone in group {}", total_sum_of_yes);
}
