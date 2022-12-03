use std::str::FromStr;

#[derive(Debug)]
struct Input {

}

#[derive(Debug)]
struct InputParseErr;

impl FromStr for Input {
    type Err = InputParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

fn parse_input(s: &str) -> Input {
    s.parse().expect("Unable to parse input")
}

#[test]
fn part1_sample_works() {
    let input_str = include_str!("day3.sample");
    let input = parse_input(input_str);

}

pub fn part1() -> u32 {
    let input_str = include_str!("day3.input");
    let input = parse_input(input_str);

    0
}

pub fn part2() -> u32 {
    let input_str = include_str!("day3.input");
    let input = parse_input(input_str);

    0
}