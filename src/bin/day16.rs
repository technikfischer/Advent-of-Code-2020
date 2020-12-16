use std::fs;
use std::collections::{HashMap};
use std::ops::Range;
use pom::parser::*;
use pom::char_class::{multispace, alpha};

type Ticket = Vec<u32>;


#[derive(Debug)]
struct Input {
    rules: HashMap<String, [Range<u32>; 2]>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

fn integer<'a>() -> Parser<'a, u8, u32> {
    one_of(b"0123456789").repeat(1..).convert(String::from_utf8).convert(|s| s.parse::<u32>())
}

fn space<'a>() -> Parser<'a, u8, ()> {
    is_a(multispace).repeat(1..).discard()
}

fn range<'a>() -> Parser<'a, u8, Range<u32>> {
    (integer() - sym(b'-') + integer()).map(|(start, end)| start..end + 1)
}

fn rule<'a>() -> Parser<'a, u8, (String, [Range<u32>; 2])> {
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
    let input = input().parse(file.as_bytes());
    println!("{:?}", input);
}