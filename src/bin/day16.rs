use std::fs;
use std::collections::HashSet;
use pom::parser::*;
use pom::char_class::{multispace, alpha};
use itertools::Itertools;
use bimap::BiMap;

type Ticket = Vec<u64>;


#[derive(Debug)]
struct Input {
    rules: Vec<HashSet<u64>>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

fn integer<'a>() -> Parser<'a, u8, u64> {
    one_of(b"0123456789").repeat(1..).convert(String::from_utf8).convert(|s| s.parse::<u64>())
}

fn space<'a>() -> Parser<'a, u8, ()> {
    is_a(multispace).repeat(1..).discard()
}

fn range<'a>() -> Parser<'a, u8, (u64, u64)> {
    integer() - sym(b'-') + integer()
}

fn rule<'a>() -> Parser<'a, u8, (String, HashSet<u64>)> {
    let rule_name = (is_a(alpha) | is_a(pom::char_class::space)).repeat(1..).convert(String::from_utf8);
    let range = || range().map(|(start, end)| (start..=end).collect::<HashSet<_>>());
    let ranges = range() - seq(b" or ") + range();

    let rule = rule_name - sym(b':') - space() + ranges;
    rule.map(|(name, (range0, range1))| (name, range0.union(&range1).copied().collect()))
}

fn ticket<'a>() -> Parser<'a, u8, Ticket> {
    let least_one = integer() - sym(b',') + list(integer(), sym(b','));
    least_one.map(|(first, mut rest)| {
        rest.insert(0, first);
        rest
    })
}

fn input<'a>() -> Parser<'a, u8, Input> {
    let rules = list(rule(), space());
    let my_ticket = seq(b"your ticket:") * space() * ticket();
    let nearby_tickets = seq(b"nearby tickets:\n") * list(ticket(), space());

    let input = rules - space().opt() + my_ticket - space().opt() + nearby_tickets - space().opt() - end();

    input.map(|((rules, my_ticket), nearby_tickets)| Input {
        rules: rules.into_iter().map(|(_, valid)| valid).collect(),
        my_ticket,
        nearby_tickets,
    })
}

fn does_rule_fit_col(rules: &Vec<HashSet<u64>>, tickets: &Vec<Ticket>, rule: usize, target_col: usize) -> bool {
    tickets.iter().enumerate().all(|t| rules[rule].contains(&t.1[target_col]))
}

fn test_loose_rules(rules: &Vec<HashSet<u64>>, tickets: &Vec<Ticket>, matched_rules_to_col: &mut BiMap<usize, usize>) {
    // test for each rule if it fits to a column, and if it is the only option
    'outer: for rule_index in 0..20 { // just hardcoded value to speed up development
        if matched_rules_to_col.contains_left(&rule_index) {
            // skip rules which are already matched
            continue;
        }
        let mut matched_col = None;
        // find all matching columns, exclude search in columns who already got a rule matched
        for col_index in 0..20 { // just hardcoded value to speed up development
            // skips cols already matched
            if matched_rules_to_col.contains_right(&col_index) {
                continue;
            }
            if does_rule_fit_col(&rules, &tickets, rule_index, col_index) {
                match matched_col {
                    Some(_) => { continue 'outer; }
                    None => { matched_col = Some(col_index); }
                }
            }
        }

        // when the loop was not terminated, there should be one column found
        match matched_col {
            Some(col) => {
                matched_rules_to_col.insert(rule_index, col);
            }
            None => panic!("No col found, there should be one though")
        }
    }
}

fn main() {
    let file = fs::read_to_string("input").expect("Could not open file");
    let input = input().parse(file.as_bytes()).unwrap();

    // part 1
    let mut valid_numbers = HashSet::new();
    for valid in input.rules.iter() {
        valid_numbers = valid_numbers.union(valid).copied().collect();
    }

    let error_rate: u64 = input.nearby_tickets.iter()
        .flat_map(|ticket| ticket.iter())
        .filter(|number| !valid_numbers.contains(number))
        .copied()
        .sum();

    println!("Ticket Scanning Error Rate is {}", error_rate);
    let mut nearby_tickets = input.nearby_tickets.into_iter()
        .filter(|ticket| ticket.iter().all(|n| valid_numbers.contains(n)))
        .collect_vec();

    nearby_tickets.insert(0, input.my_ticket);
    let mut rules_to_cols = BiMap::new();
    for _ in 0..20 {
        test_loose_rules(&input.rules, &nearby_tickets, &mut rules_to_cols);
        println!("{:?}", rules_to_cols);
    }

    let mut product = 1;
    for rule in 0..6 { // those starting with departure
        product *= nearby_tickets[0][*rules_to_cols.get_by_left(&rule).unwrap()];
    }

    println!("Product of all departure fields is {}", product);
}