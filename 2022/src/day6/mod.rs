use std::{str::FromStr, collections::HashSet};

#[derive(Debug)]
struct Input {
    stream: Vec<u8>
}

#[derive(Debug)]
struct InputParseError;

impl FromStr for Input {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Input {stream: s.trim().as_bytes().iter().copied().collect()})
    }
}

fn parse_input(s: &str) -> Input {
    s.parse().expect("Unable to parse input")
}

fn find_n_unique(input: &Input, n: usize) -> usize {
    for index in n..input.stream.len() {
        let prev = &input.stream[index-n..index];
        let x = prev.iter().collect::<HashSet<&u8>>();
        if x.len() == n {
            return index;
        }
    }

    assert!(false, "Found no start");
    0
}

#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("sample.txt"));

    let result = find_n_unique(&input, 4);
    assert_eq!(result, 7)
}

pub fn part1() -> usize {
    let input = parse_input(include_str!("input.txt"));
    find_n_unique(&input, 4)
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 1953)
}

#[test]
fn part2_sample_works() {
    let input = parse_input(include_str!("sample.txt"));

    let result = find_n_unique(&input, 14);
    assert_eq!(result, 19)
}

pub fn part2() -> usize {
    let input = parse_input(include_str!("input.txt"));
    find_n_unique(&input, 14)
}

#[test]
fn part2_works() {
    assert_eq!(part2(), 2301)
}
