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

fn is_valid(passport: &HashMap<String, String>, complex: bool) -> bool {
    if !complex {
        return REQUIRED_KEYS
        .iter()
        .all(|key| passport.contains_key(&key.to_string()))
    } else {
        return REQUIRED_KEYS
        .iter()
        .all(|key| {
            let key = key.to_string();
            let result =  passport.contains_key(&key)
                && validate_key(&key[..], &passport[&key][..]);
            //println!("> {:?} {:?} {}", key, passport, result);
            return result;
        })
    }
}

fn count_valid(input: &Input) -> usize {
    input.passports.iter().filter(|p| is_valid(p, false)).count()
}

pub fn part1() -> usize {
    let input = parse_input(include_str!("inputs/day4.txt"));
    count_valid(&input)
}

#[test]
fn part1_works() {
    assert_eq!(part1(),204)
}

static VALID_EYES: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn validate_key(key: &str, value: &str) -> bool {
    return match key {
        "byr" => {
            match value.parse::<u32>() {
                Err(_) => false,
                Ok(v) => v >= 1920 && v <= 2002
            }
        },
        "iyr" => {
            match value.parse::<u32>() {
                Err(_) => false,
                Ok(v) => v >= 2010 && v <= 2020
            }
        },
        "eyr" => {
            match value.parse::<u32>() {
                Err(_) => false,
                Ok(v) => v >= 2020 && v <= 2030
            }
        },
        "hgt" => {
            let len = value.len();
            match &value[len-2..] {
                "in" => 
                    match value[..len-2].parse::<u32>() {
                        Err(_) => false,
                        Ok(v) => v >= 59 && v <= 76
                    },
                "cm" => 
                    match value[..len-2].parse::<u32>() {
                        Err(_) => false,
                        Ok(v) => v >= 150 && v <= 193
                    },
                _ => false
            }
        },
        "hcl" => {
            value.len() == 7 
                && &value[0..1] == "#"
                && value[1..].chars().all(|c| (c >= '0' && c <= '9')||(c >= 'a' && c <= 'f'))
        },
        "ecl" => {
            VALID_EYES.contains(&value)
        },
        "pid" => {
            value.len() == 9 && value.chars().all(|c| c >= '0' && c <= '9')
        },
        _ => false
    }
}

fn count_valid_2(input: &Input) -> usize {
    input.passports.iter().filter(|p| is_valid(p, true)).count()
}

pub fn part2() -> usize {
    let input = parse_input(include_str!("inputs/day4.txt"));
    count_valid_2(&input)
}

#[test]
fn part2_works() {
    assert_eq!(part2(),179)
}