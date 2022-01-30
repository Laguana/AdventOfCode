use std::collections::HashMap;

#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("inputs/day19.sample"));

    let result = input.strings.iter().filter(|s| matches_rule(&input.rules, 0, &s[..])).count();
    assert_eq!(result, 2)
}

#[derive(Debug)]
enum Rule {
    Literal(char),
    Sequence(Vec<Vec<u16>>),
}

#[derive(Debug)]
struct Input {
    rules: HashMap<u16, Rule>,
    strings: Vec<String>,
}

fn parse_rule(s: &str) -> Rule {
    let mut chars = s.chars();
    match chars.next() {
        Some('"') => Rule::Literal(chars.next().unwrap()),
        Some(_) => Rule::Sequence(
            s.split(" | ")
                .map(|l| {
                    l.split(" ")
                        .map(|n| n.parse().expect("Should be int"))
                        .collect()
                })
                .collect(),
        ),
        _ => panic!("rules must be non-empty"),
    }
}

fn parse_rule_line(line: &str) -> (u16, Rule) {
    let segments = line.trim().split(": ").collect::<Vec<_>>();
    let number = segments[0].parse::<u16>().expect("Should be an int");
    let rule = parse_rule(segments[1]);
    (number, rule)
}

fn parse_input(input: &str) -> Input {
    let trimmed = input
        .trim()
        .replace("\r", "");
    let parts = trimmed.split("\n\n")
        .collect::<Vec<_>>();
    Input {
        rules: parts[0].lines().map(parse_rule_line).collect(),
        strings: parts[1].lines().map(|l| l.trim().to_string()).collect(),
    }
}

fn matches_rule_(rules: &HashMap<u16, Rule>, rule_idx: u16, from: usize, s: &str) -> Vec<usize> {
    match &rules[&rule_idx] {
        Rule::Literal(c) => match s[from..=from].chars().nth(0) {
            None => vec![],
            Some(d) => if *c == d {
                vec![from+1]
            } else {
                vec![]
            }
        },
        Rule::Sequence(v) => {
            v.iter().flat_map(|seq| {
                let mut v = vec![from];
                for x in seq {
                    v = v.into_iter().flat_map(|j| matches_rule_(rules, *x, j, s)).collect();
                }
                v
            }).collect()
        }
    }
}

fn matches_rule(rules: &HashMap<u16, Rule>, rule_idx: u16, s: &str) -> bool {
    matches_rule_(rules, rule_idx, 0, s).iter().any(|i| *i == s.len())
}

pub fn part1() -> usize {
    let input = parse_input(include_str!("inputs/day19.txt"));

    input.strings.iter().filter(|s| matches_rule(&input.rules, 0, &s[..])).count()
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 104)
}