use std::fmt::Debug;
use std::str::FromStr;

#[test]
fn part1_sample_works() {
    let input = include_str!("inputs/day2.sample");
    let input = parse_input(input);
    let n_valid = count_valid_passwords(&input);
    assert_eq!(n_valid, 2);
}

struct PasswordRule {
    min: u8,
    max: u8,
    character: char,
}

impl std::fmt::Display for PasswordRule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}-{} '{}'", self.min, self.max, self.character)
    }
}

#[test]
fn password_rule_display_works() {
    let rule = PasswordRule{min: 0, max: 2, character: 'x'};
    let displayed = format!("{}", rule);
    assert_eq!(displayed, "0-2 'x'")
}

struct Input {
    cases: Vec<(PasswordRule, String)>,
}

#[derive(Debug)]
struct ParseError;

impl FromStr for Input {
    type Err = ParseError;
    fn from_str(s: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        match s
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(|s| {
                let parts: Vec<&str> = s.split(" ").collect();
                if parts.len() != 3 {
                    return Err(ParseError);
                }
                let password = parts[2].trim().to_string();
                let character = parts[1].chars().nth(0);
                if character.is_none() {
                    return Err(ParseError);
                }
                let range_parts: Result<Vec<u8>, std::num::ParseIntError> =
                    parts[0].split("-").map(|s| s.parse()).collect();
                match range_parts {
                    Ok(v) => Ok((
                        PasswordRule {
                            character: character.unwrap(),
                            min: v[0],
                            max: v[1],
                        },
                        password,
                    )),
                    Err(_) => Err(ParseError),
                }
            })
            .collect()
        {
            Err(e) => Err(e),
            Ok(v) => Ok(Input { cases: v }),
        }
    }
}

fn parse_input(input: &str) -> Input {
    return input.parse().expect("Should have parsed the input");
}

fn password_is_valid(rule: &PasswordRule, password: &str) -> bool {
    let count = password.chars().filter(|c| *c == rule.character).count();
    return count >= rule.min.into() && count <= rule.max.into();
}

fn count_valid_passwords(input: &Input) -> u32 {
    input
        .cases
        .iter()
        .filter(|(rule, password)| password_is_valid(rule, password))
        .count()
        .try_into()
        .unwrap()
}

pub fn part1() -> u32 {
    let input = parse_input(include_str!("inputs/day2.txt"));
    count_valid_passwords(&input)
}

#[test]
fn part1_works() {
    let result = part1();
    assert_eq!(result, 580);
}

#[test]
fn part2_sample_works() {
    let input = include_str!("inputs/day2.sample");
    let input = parse_input(input);
    let n_valid = count_valid_passwords_2(&input);
    assert_eq!(n_valid, 1);
}

fn password_is_valid_2(rule: &PasswordRule, password: &str) -> bool {
    let c1 = password
        .chars()
        .nth((rule.min-1).into())
        .expect("This is supposed to be a valid index");
    let c2 = password
        .chars()
        .nth((rule.max-1).into())
        .expect("This is supposed to be a valid index");
    //println!("rule {} password {} c1 {} c2 {}", rule, password, c1, c2);
    return (c1 == rule.character) != (c2 == rule.character);
}

fn count_valid_passwords_2(input: &Input) -> u32 {
    input
        .cases
        .iter()
        .filter(|(rule, password)| password_is_valid_2(rule, password))
        .count()
        .try_into()
        .unwrap()
}

pub fn part2() -> u32 {
    let input = parse_input(include_str!("inputs/day2.txt"));
    count_valid_passwords_2(&input)
}

#[test]
fn part2_works() {
    let result = part2();
    assert_eq!(result, 611);
}
