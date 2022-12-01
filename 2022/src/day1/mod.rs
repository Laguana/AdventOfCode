use std::str::FromStr;

struct Input {
    elves: Vec<Vec<u32>>,
}

impl FromStr for Input {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        let entries: Result<Vec<Vec<u32>>, std::num::ParseIntError> = 
            s.replace("\r", "").split("\n\n")
                .map(|s| {
                        // println!("'{}'", s);
                        s.trim().split("\n")
                        .map(|s| s.trim().parse()).collect()
                    })
                .collect();

        match entries {
            Ok(e) => Ok(Input { elves: e}),
            Err(e) => Err(e)
        }
    }
}

fn parse_input(input: &str) -> Input {
    input.parse().expect("unable to parse input")
}

fn max_calories(input: &Input) -> u32 {
    input.elves.iter()
        .map(|e| e.iter().fold(0, |a,b| a+b))
        .max()
        .expect("Should have at least one entry")
}

fn top3_calories(input: &Input) -> u32 {
    let mut sums = input.elves.iter()
        .map(|e| e.iter().fold(0, |a,b| a+b))
        .collect::<Vec<u32>>();
    sums.sort_by(|a,b| b.cmp(a));
    return sums[0] + sums[1] + sums[2]

}

#[test]
fn part1_sample_works() {
    let input_str = include_str!("day1.sample");
    let input = parse_input(input_str);

    let result = max_calories(&input);
    assert_eq!(result, 24000)
}

pub fn part1() -> u32 {
    let input_str: &str = include_str!("day1.input");
    let input = parse_input(input_str);

    max_calories(&input)
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 67622);
}

#[test]
fn part2_sample_works() {
    let input_str = include_str!("day1.sample");
    let input = parse_input(input_str);

    let result = top3_calories(&input);
    assert_eq!(result, 45000)
}

pub fn part2() -> u32 {
    let input_str: &str = include_str!("day1.input");
    let input = parse_input(input_str);

    top3_calories(&input)
}

#[test]
fn part2_works() {
    assert_eq!(part2(), 201491)
}