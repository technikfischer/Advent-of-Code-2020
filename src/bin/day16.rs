use std::fs;
use std::collections::{HashMap, HashSet};
use pom::parser::*;
use pom::char_class::{multispace, alpha};

type Ticket = Vec<u32>;


#[derive(Debug)]
struct Input {
    rules: HashMap<String, [(u32, u32); 2]>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

fn integer<'a>() -> Parser<'a, u8, u32> {
    one_of(b"0123456789").repeat(1..).convert(String::from_utf8).convert(|s| s.parse::<u32>())
}

fn space<'a>() -> Parser<'a, u8, ()> {
    is_a(multispace).repeat(1..).discard()
}

fn range<'a>() -> Parser<'a, u8, (u32, u32)> {
    integer() - sym(b'-') + integer()
}

fn rule<'a>() -> Parser<'a, u8, (String, [(u32, u32); 2])> {
    let rule_name = (is_a(alpha) | is_a(pom::char_class::space)).repeat(1..).convert(String::from_utf8);
    let ranges = range() - seq(b" or ") + range();

    let rule = rule_name - sym(b':') - space() + ranges;
    rule.map(|(name, (range0, range1))| (name, [range0, range1]))
}

fn ticket<'a>() -> Parser<'a, u8, Ticket> {
    list(integer(), sym(b','))
}

fn input<'a>() -> Parser<'a, u8, Input> {
    let rules = list(rule(), space());
    let my_ticket = seq(b"your ticket:") * space() * ticket();
    let nearby_tickets = seq(b"nearby tickets:\n") * list(ticket(), space());

    let input = rules - space().opt() + my_ticket - space().opt() + nearby_tickets - space().opt() - end();

    input.map(|((rules, my_ticket), nearby_tickets)| Input {
        rules: rules.into_iter().collect(),
        my_ticket,
        nearby_tickets,
    })
}

fn main() {
    let file = fs::read_to_string("input").expect("Could not open file");
    let input = input().parse(file.as_bytes()).unwrap();

    // part 1
    let mut valid_numbers = HashSet::new();
    for (_, &ranges) in input.rules.iter() {
        for range in 0..ranges.len() {
            for number in ranges[range].0..=ranges[range].1 {
                valid_numbers.insert(number);
            }
        }
    }

    let error_rate : u32 = input.nearby_tickets.iter()
        .flat_map(|ticket| ticket.iter())
        .filter(|number| !valid_numbers.contains(number))
        .copied()
        .sum();

    println!("Ticket Scanning Error Rate is {}", error_rate);
}