use std::str::FromStr;

#[test]
fn part1_sample_works() {
    let input = parse_input("BFFFBBFRRR");
    assert_eq!(input.passes[0], 567)
}

struct Input {
    passes: Vec<u16>,
}


impl FromStr for Input {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        let v: Result<Vec<u16>, std::num::ParseIntError> = s.split("\n")
            .filter(|l| !l.is_empty())
            .map(|l| {
                let n = l.trim()
                    .replace("B","1").replace("F","0")
                    .replace("R","1").replace("L","0");
                    u16::from_str_radix(&n[..], 2)
            }).collect();
            v.map(|v| Input{passes: v})
    }
}

fn parse_input(input: &str) -> Input {
    input.parse().expect("Should be able to parse the input")
}

pub fn part1() -> u16 {
    let input = parse_input(include_str!("inputs/day5.txt"));
    *input.passes.iter().max().unwrap()
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 930)   
}

pub fn part2() -> u16 {
    let mut input = parse_input(include_str!("inputs/day5.txt"));
    input.passes.sort();
    let mut prev = input.passes[0];
    for &v in input.passes[1..].iter() {
        if v != prev+1 {
            return prev+1;
        } else {
            prev = v;
        }
    }
    panic!("Should have found the seat")
}

#[test]
fn part2_works() {
    assert_eq!(part2(), 515)
}