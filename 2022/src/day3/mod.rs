use std::{str::FromStr, collections::HashSet};

#[derive(Debug)]
struct Input {
    rucksacks: Vec<Vec<u8>>
}

#[derive(Debug)]
struct InputParseErr;

impl FromStr for Input {
    type Err = InputParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bags: Vec<Vec<u8>> = s.trim().replace("\r", "").split("\n")
            .map(|l| l.as_bytes().into())
            .collect();
        Ok(Input {rucksacks: bags})
    }
}

fn parse_input(s: &str) -> Input {
    s.parse().expect("Unable to parse input")
}

fn score_char(c: u8) -> u8 {
    if c >= b'a' && c <= b'z' {
        return (c-b'a') + 1
    }
    assert!(c >= b'A' && c <= b'Z', "c={}", c);
    
    (c-b'A') + 27
}

fn find_common(rucksack: &Vec<u8>) -> u8 {
    let len = rucksack.len();
    let first_half = &rucksack[0..len/2];
    let second_half = &rucksack[len/2..];
    let first_half_set: HashSet<u8> = first_half.iter().map(|e| *e).collect();
    for c in second_half.iter() {
        if first_half_set.contains(c) {
            return *c;
        }
    }
    println!("Halves '{:?}', '{:?}'", first_half, second_half);
    assert!(false, "Did not find any overlap!");
    0
}

fn score_rucksacks(input: Input) -> u32 {
    input.rucksacks.iter()
        .map(|b| find_common(b))
        .map(|c| score_char(c))
        .fold(0, |acc, v| acc+v as u32)
        
}

fn find_badge(group: &[Vec<u8>]) -> u8 {
    assert_eq!(group.len(), 3);
    let sets: Vec<HashSet<u8>> = group.iter()
        .map(|l| l.iter().map(|e|*e).collect())
        .collect();
    let candidates: HashSet<u8> = sets[0].intersection(&sets[1]).map(|e|*e)
        .collect::<HashSet<u8>>().intersection(&sets[2]).map(|e|*e)
        .collect();

    *candidates.iter().nth(0).expect("Should have been exactly one candidate")   
}

fn score_badges(input: &Input) -> u32 {
    input.rucksacks.iter().as_slice().chunks(3)
        .map(|c|find_badge(c))
        .fold(0, |acc, b| acc + score_char(b) as u32)
}

#[test]
fn part1_sample_works() {
    let input_str = include_str!("day3.sample");
    let input = parse_input(input_str);

    let result = score_rucksacks(input);
    assert_eq!(result, 157)
}

pub fn part1() -> u32 {
    let input_str = include_str!("day3.input");
    let input = parse_input(input_str);

    score_rucksacks(input)
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 8123)
}


#[test]
fn part2_sample_works() {
    let input_str = include_str!("day3.sample");
    let input = parse_input(input_str);

    let result = score_badges(&input);
    assert_eq!(result, 70)
}

pub fn part2() -> u32 {
    let input_str = include_str!("day3.input");
    let input = parse_input(input_str);

    score_badges(&input)
}

#[test]
fn part2_works() {
    assert_eq!(part2(), 2620)
}