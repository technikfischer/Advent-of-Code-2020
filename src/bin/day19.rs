use std::fs;
use itertools::Itertools;

#[derive(Debug)]
enum Rule<'a> {
    String(&'a str),
    Sequence(Vec<usize>),
    Alternation(Box<Rule<'a>>, Box<Rule<'a>>),
}

fn parse_sequence<'a>(sequence: &str) -> Rule<'a> {
    let rules = sequence.split_ascii_whitespace().map(|id| id.parse::<usize>().expect(sequence)).collect_vec();
    Rule::Sequence(rules)
}

fn parse_rule(rule: &str) -> Rule {
    // split of number and :<space>
    let rule = &rule.trim_start_matches(char::is_numeric)[2..];

    if let Some((left, right)) = rule.split('|').collect_tuple() {
        // alternation
        let left = parse_sequence(left);
        let right = parse_sequence(right);
        Rule::Alternation(Box::new(left), Box::new(right))
    } else if rule.starts_with('"') {
        // string literal
        let rule = rule.trim_matches('"');
        Rule::String(rule)
    } else {
        // normal sequence
        parse_sequence(rule)
    }
}

fn does_match<'a>(string: &'a str, pattern: &Rule, rules: &Vec<Rule>) -> (bool, &'a str) {
    match pattern {
        Rule::String(p) => (string.starts_with(p), &string[p.len()..]),
        Rule::Sequence(seq) => {
            let mut string = string;
            for rule in seq.iter().map(|&id| &rules[id]) {
                let (matches, rest) = does_match(string, rule, rules);
                if matches {
                    string = rest;
                } else {
                    return (false, string);
                }
            }
            (true, string)
        }
        Rule::Alternation(left, right) => {
            let (left_match, rest) = does_match(string, left, rules);
            if left_match {
                return (true, rest);
            }
            does_match(string, right, rules)
        }
    }
}

fn main() {
    let rules = fs::read_to_string("rules").expect("Could not open file");
    let rules = rules.lines().sorted_by_key(|e| e.split(':').nth(0).map(|s| s.parse::<u32>().unwrap())).map(parse_rule).collect _vec();

    let input = fs::read_to_string("input").expect("Could not open file");
    let input = input.lines().collect_vec();

    let mut amount = 0;
    for i in input {
        if let (true, "") = does_match(i, &rules[0], &rules) {
            amount += 1;
        }
    }

    println!("Matching amount is {}", amount);
}