use std::collections::HashMap;
use std::str::FromStr;

#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("inputs/day4.sample"));
    let valid = count_valid(&input);
    assert_eq!(valid, 2)
}

#[derive(Debug)]
struct Input {
    passports: Vec<HashMap<String, String>>,
}

#[derive(Debug)]
struct ParseError;

impl FromStr for Input {
    type Err = ParseError;
    fn from_str(s: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        let v: Result<Vec<HashMap<String, String>>, ParseError> = s
            .replace("\r","")
            .split("\n\n")
            .map(|l| {
                l.trim()
                    .split_whitespace()
                    .map(|e| {
                        let parts: Vec<&str> = e.split(":").collect();
                        if parts.len() != 2 {
                            return Err(ParseError);
                        }

                        Ok((parts[0].to_string(), parts[1].to_string()))
                    })
                    .collect()
            })
            .collect();
        v.map(|v| Input { passports: v })
    }
}

fn parse_input(s: &str) -> Input {
    s.parse().expect("Should be parsable")
}

static REQUIRED_KEYS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn is_valid(passport: &HashMap<String, String>) -> bool {
    REQUIRED_KEYS
        .iter()
        .all(|key| passport.contains_key(&key.to_string()))
}

fn count_valid(input: &Input) -> usize {
    input.passports.iter().filter(|p| is_valid(p)).count()
}

pub fn part1() -> usize {
    let input = parse_input(include_str!("inputs/day4.txt"));
    count_valid(&input)
}

#[test]
fn part1_works() {
    assert_eq!(part1(),0)
}