use std::str::FromStr;
#[derive(Debug)]
struct Input {

}

impl FromStr for Input {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

fn parse_input(input: &str) -> Input {
    input.parse().expect("unable to parse input")
}

#[test]
fn part1_sample_works() {
    let input_str = include_str!("day2.sample");
    let input = parse_input(input_str);

    todo!()
}

pub fn part1() -> u32 {
    let input_str = include_str!("day2.input");
    let input = parse_input(input_str);

    0
}

pub fn part2() -> u32 {
    let input_str = include_str!("day2.input");
    let input = parse_input(input_str);
    0
}