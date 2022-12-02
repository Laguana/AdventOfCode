use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy)]
enum Response {
    X,
    Y,
    Z,
}

#[derive(Debug)]
struct Input {
    sequence: Vec<(RPS, Response)>
}

#[derive(Debug)]
struct BadInputError;

impl FromStr for Input {
    type Err = BadInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let list: Result<Vec<(RPS, Response)>, BadInputError> = s.trim()
            .split("\n")
            .map(|l| {
                let left = match l.chars().nth(0) {
                    Some('A') => Ok(RPS::Rock),
                    Some('B') => Ok(RPS::Paper),
                    Some('C') => Ok(RPS::Scissors),
                    _ => Err(BadInputError)
                };
                let right = match l.chars().nth(2) {
                    Some('X') => Ok(Response::X),
                    Some('Y') => Ok(Response::Y),
                    Some('Z') => Ok(Response::Z),
                    _ => Err(BadInputError)
                };
                match (left, right) {
                    (Err(e), _) => Err(e),
                    (_, Err(e)) => Err(e),
                    (Ok(l), Ok(r)) => Ok((l,r))
                }
            })
            .collect();
        match list {
            Ok(l) => Ok(Input { sequence: l}),
            Err(e) => Err(e),
        }
        
    }
}

fn score_part1(matchup: (RPS, Response)) -> u32 {
    match matchup {
        (RPS::Rock, Response::X) => 4,
        (RPS::Rock, Response::Y) => 8,
        (RPS::Rock, Response::Z) => 3,
        (RPS::Paper, Response::X) => 1,
        (RPS::Paper, Response::Y) => 5,
        (RPS::Paper, Response::Z) => 9,
        (RPS::Scissors, Response::X) => 7,
        (RPS::Scissors, Response::Y) => 2,
        (RPS::Scissors, Response::Z) => 6,
    }
}

fn score_part2(matchup: (RPS, Response)) -> u32 {
    match matchup {
        (RPS::Rock, Response::X) => 3,
        (RPS::Rock, Response::Y) => 4,
        (RPS::Rock, Response::Z) => 8,
        (RPS::Paper, Response::X) => 1,
        (RPS::Paper, Response::Y) => 5,
        (RPS::Paper, Response::Z) => 9,
        (RPS::Scissors, Response::X) => 2,
        (RPS::Scissors, Response::Y) => 6,
        (RPS::Scissors, Response::Z) => 7,
    }
}

fn score_input_part1(input: &Input) -> u32 {
    input.sequence.iter().fold(0, |a, m| a + score_part1(*m))
}

fn score_input_part2(input: &Input) -> u32 {
    input.sequence.iter().fold(0, |a, m| a + score_part2(*m))
}

fn parse_input(input: &str) -> Input {
    input.parse().expect("unable to parse input")
}

#[test]
fn part1_sample_works() {
    let input_str = include_str!("day2.sample");
    let input = parse_input(input_str);

    let score = score_input_part1(&input);
    assert_eq!(score, 15)
}

pub fn part1() -> u32 {
    let input_str = include_str!("day2.input");
    let input = parse_input(input_str);

    score_input_part1(&input)
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 11063)
}

#[test]
fn part2_sample_works() {
    let input_str = include_str!("day2.sample");
    let input = parse_input(input_str);

    let score = score_input_part2(&input);
    assert_eq!(score, 12)
}

pub fn part2() -> u32 {
    let input_str = include_str!("day2.input");
    let input = parse_input(input_str);
    
    score_input_part2(&input)
}

#[test]
fn part2_works() {
    assert_eq!(part2(), 10349)
}