use std::str::FromStr;

#[derive(Debug)]
struct Input {
    numbers: Vec<i64>
}

#[derive(Debug)]
struct InputParseError;

impl FromStr for Input {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Input{ numbers: s.trim().lines().map(|l| {
            l.chars().fold(0, |acc, c| {
                acc * 5 + 
                match c {
                    '2' => 2,
                    '1' => 1,
                    '0' => 0,
                    '-' => -1,
                    '=' => -2,
                    _ => panic!("Bad input")
                }
            })
        }).collect()})
    }
}

fn parse_input(s: &str) -> Input {
    s.parse().expect("Unable to parse input")
}

fn to_snafu(i: i64) -> String {
    let mut result: Vec<char> = vec![];

    let mut n = i;
    while n != 0 {
        let c = 
            match n%5 {
                0 => {
                    n = n/5;
                    '0'
                },
                1 => {
                    n = n/5;
                    '1'
                },
                2 => {
                    n = n/5;
                    '2'
                },
                3 => {
                    n = (n+2)/5;
                    '='
                },
                4 => {
                    n = (n+1)/5;
                    '-'
                },
                _ => panic!("Bad input")

            };
        result.push(c);
    }

    String::from_iter(result.into_iter().rev())
}

#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("sample.txt"));
    let result: i64 = input.numbers.iter().sum();
    assert_eq!(result, 4890);
    assert_eq!(to_snafu(result), "2=-1=0")
    
}

pub fn part1() -> String {
    let input = parse_input(include_str!("input.txt"));
    let result = input.numbers.iter().sum();
    to_snafu(result)
}

#[test]
fn part1_works() {
    assert_eq!(part1(), "2-=0-=-2=111=220=100")
}