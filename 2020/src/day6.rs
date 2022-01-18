use std::collections::HashSet;
use std::str::FromStr;

#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("inputs/day6.sample"));
    let result = sum_common(&input);
    assert_eq!(result, 11)
}

struct Input {
    groups: Vec<Vec<String>>,
}

#[derive(Debug)]
struct ParseError;

impl FromStr for Input {
    type Err = ParseError;
    fn from_str(s: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        let x = s
            .replace("\r", "")
            .split("\n\n")
            .map(|e| e.split("\n").map(|s| s.to_string()).collect())
            .collect();

        Ok(Input { groups: x })
    }
}

fn parse_input(s: &str) -> Input {
    s.parse().expect("Should be able to parse input")
}

fn sum_common(input: &Input) -> usize {
    input.groups.iter().map(|v| {
        let s = v.iter().fold(HashSet::new(), |acc, e| {
            let se: HashSet<char> = e.chars().collect();
            acc.union(&se).cloned().collect()
        });
        
        println!("{:?} -> {:?}", v, s);
        return s.iter().count()
    }).fold(0, |acc, b| acc + b)
}

pub fn part1() -> usize {
    let input = parse_input(include_str!("inputs/day6.txt"));
    sum_common(&input)
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 6768)
}