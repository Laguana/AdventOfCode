use std::str::FromStr;

#[derive(Debug)]
struct Input {

}

#[derive(Debug)]
struct InputParseError;

impl FromStr for Input {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

fn parse_input(s: &str) -> Input {
    s.parse().expect("Unable to parse input")
}

#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("sample.txt"));

    
}

pub fn part1() -> u32 {
    let input = parse_input(include_str!("input.txt"));
    0
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 0)
}

pub fn part2() -> u32 {
    let input = parse_input(include_str!("input.txt"));
    0
}

#[test]
fn part2_works() {
    assert_eq!(part2(), 0)
}
