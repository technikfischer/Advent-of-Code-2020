use std::fs;
use itertools::Itertools;
use std::collections::HashMap;

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

fn parse_rule(rule: &str) -> (usize, Rule) {
    let (id, rule): (&str, &str) = rule.split(": ").collect_tuple().unwrap();
    let id = id.parse::<usize>().unwrap();

    (id, if let Some((left, right)) = rule.split('|').collect_tuple() {
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
    })
}

fn does_match<'a>(string: &'a str, pattern: &Rule, rules: &HashMap<usize, Rule>, must_consume: bool) -> (bool, &'a str) {
    if string.len() == 0 {
        return (false, string);
    }
    match pattern {
        Rule::String(p) => (string.starts_with(p) && (!must_consume || p.len() == string.len()), &string[p.len()..]),
        Rule::Sequence(seq) => {
            let mut string = string;
            for (idx, rule) in seq.iter().map(|id| &rules[id]).enumerate() {
                let (matches, rest) = does_match(string, rule, rules, must_consume && (idx == seq.len() - 1));
                if matches {
                    string = rest;
                } else {
                    return (false, string);
                }
            }
            (true, string)
        }
        Rule::Alternation(left, right) => {
            let (left_match, rest) = does_match(string, left, rules, true);
            if left_match {
                return (true, rest);
            }
            does_match(string, right, rules, true)
        }
    }
}

fn main() {
    let rules = fs::read_to_string("rules").expect("Could not open file");
    let mut rules = rules.lines().map(parse_rule).collect::<HashMap<_, _>>();

    // replace rules
    rules.insert(8, Rule::Alternation(Box::new(Rule::Sequence(vec![42])), Box::new(Rule::Sequence(vec![42, 8]))));
    rules.insert(11, Rule::Alternation(Box::new(Rule::Sequence(vec![42, 31])), Box::new(Rule::Sequence(vec![42, 11, 31]))));

    let input = fs::read_to_string("input").expect("Could not open file");
    let input = input.lines().collect_vec();

    let mut amount = 0;
    for i in input {
        if let (true, "") = does_match(i, &rules[&0], &rules, true) {
            amount += 1;
        }
    }

    println!("Matching amount is {}", amount);
}