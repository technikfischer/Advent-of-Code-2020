#[macro_use]
extern crate lazy_static;

extern crate regex;

use regex::{Regex};
use std::fs;
use std::collections::{HashMap};

fn parse_bag_from_input(line: &str) -> (String, Vec<(i32, String)>) {
    lazy_static! {
        static ref OUTER_PATTERN: Regex =  Regex::new(r"(?P<bag>\w+ \w+) bags contain (?P<contains>.+)\.").unwrap();
        static ref INNER_PATTERN: Regex =  Regex::new(r"(?P<amount>\d+) (?P<bag>\w+ \w+) bag").unwrap();
    }

    let outer = OUTER_PATTERN.captures(line).expect("Could not parse line");
    let bag = outer["bag"].to_string();
    let contains: Vec<(i32, String)> = INNER_PATTERN.captures_iter(&outer["contains"])
        .map(|c| {
            let amount: i32 = c["amount"].parse().expect("Could not parse amount");
            (amount, c["bag"].to_string())
        })
        .collect();

    return (bag, contains);
}

fn lead_to_shiny_bag(bag: &String, bags: &HashMap<String, Vec<(i32, String)>>) -> bool {
    for (_, ref bag) in &bags[bag] {
        if bag == "shiny gold" {
            return true;
        }

        if lead_to_shiny_bag(&bag, bags) {
            return true;
        }
    }
    return false;
}

fn compute_bags_inside(bag: &str, bags: &HashMap<String, Vec<(i32, String)>>) -> i32 {
    return bags[bag].iter()
        .map(|(amount, bag)| amount * (1 + compute_bags_inside(bag, bags)))
        .fold(0, |a, b| a + b);
}

fn main() {
    let lines = fs::read_to_string("input2").expect("Could not open input file");
    let bags = lines.lines()
        .map(parse_bag_from_input)
        .collect::<HashMap<String, Vec<(i32, String)>>>();

    // part 1
    let mut bag_count = 0;
    for bag in bags.keys() {
        if lead_to_shiny_bag(&bag, &bags) {
            bag_count += 1;
        }
    }
    println!("Bag count leading to golden shiny bag: {}", bag_count);

    // part 2
    let contained_bags = compute_bags_inside("shiny gold", &bags);
    println!("Golden shiny bag contains {} other bags", contained_bags);
}
