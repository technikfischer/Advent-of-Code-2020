use std::fs;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

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

fn does_match<'a>(string: &'a str, pattern: &Rule, rules: &HashMap<usize, Rule>) -> HashSet<&'a str> {
    if string.len() == 0 {
        return HashSet::new();
    }
    match pattern {
        Rule::String(p) => if string.starts_with(p) { vec![&string[p.len()..]].into_iter().collect() } else { HashSet::new() },
        Rule::Sequence(seq) => {
            let mut possibilites = vec![string].into_iter().collect();
            for rule in seq.iter().map(|id| &rules[id]) {
                let mut generated = HashSet::new();
                for p in possibilites {
                    generated.extend(does_match(p, rule, rules));
                }
                possibilites = generated
            }
            possibilites
        }
        Rule::Alternation(left, right) => {
            let mut left = does_match(string, left, rules);
            let right = does_match(string, right, rules);
            left.extend(right);
            left
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
        println!("{}", i);
        let matches = does_match(i, &rules[&0], &rules);
        if matches.iter().any(|s| s.len() == 0) {
            amount += 1;
        }
    }

    println!("Matching amount is {}", amount);
}