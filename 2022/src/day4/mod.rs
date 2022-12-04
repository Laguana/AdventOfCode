use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct Interval {
    low: u32,
    high: u32
}

impl Interval {
    fn contains(&self, other: &Interval) -> bool {
        self.low <= other.low && self.high >= other.high
    }

    fn overlaps(&self, other: &Interval) -> bool {
        !(self.low > other.high || self.high < other.low)
    }
}

#[derive(Debug)]
struct Input {
    pairs: Vec<(Interval, Interval)>
}

impl FromStr for Input {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pairs: Result<Vec<(Interval, Interval)>, Self::Err> = s.replace("\r","").trim().split("\n")
            .map(|l| {
                let mut parts = l.split(",").map(
                    |v| {
                        let mut parts = v.split("-").map(|e|e.parse::<u32>());
                        let low = parts.nth(0).expect("Missing low bound");
                        let high = parts.nth(0).expect("Missing high bound");
                        match (low, high) {
                            (Err(e), _) => Err(e),
                            (_, Err(e)) => Err(e),
                            (Ok(l), Ok(r)) => Ok(Interval{ low: l, high: r})
                        }
                    }
                );
                let left = parts.nth(0).expect("Missing part0");
                let right = parts.nth(0).expect("Missing part1");
                match (left, right) {
                    (Err(e), _) => Err(e),
                    (_, Err(e)) => Err(e),
                    (Ok(l), Ok(r)) => Ok((l,r))
                }
            })
            .collect();

        match pairs {
            Ok(l) => Ok(Input {pairs: l}),
            Err(e) => Err(e)
        }
    }
}

fn parse_input(s: &str) -> Input {
    s.parse().expect("Unable to parse input")
}

fn count_contains(input: &Input) -> usize {
    input.pairs.iter().filter(|(a,b)| a.contains(b) || b.contains(a)).count()
}

fn count_overlaps(input: &Input) -> usize {
    input.pairs.iter().filter(|(a,b)| a.overlaps(b)).count()
}

#[test]
fn part1_sample_works() {
    let input_str = include_str!("day4.sample");
    let input = parse_input(input_str);

    let result = count_contains(&input);
    assert_eq!(result, 2)
}

pub fn part1() -> usize {
    let input_str = include_str!("day4.input");
    let input = parse_input(input_str);

    count_contains(&input)
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 528)
}

#[test]
fn part2_sample_works() {
    let input_str = include_str!("day4.sample");
    let input = parse_input(input_str);

    let result = count_overlaps(&input);
    assert_eq!(result, 4)
}

pub fn part2() -> usize {
    let input_str = include_str!("day4.input");
    let input = parse_input(input_str);

    count_overlaps(&input)
}

#[test]
fn part2_works() {
    assert_eq!(part2(), 881)
}