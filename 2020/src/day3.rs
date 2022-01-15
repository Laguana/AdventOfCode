use std::str::FromStr;

#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("inputs/day3.sample"));
    let trees_hit = trees_hit(&input, 3, 1);
    assert_eq!(trees_hit, 7)
}

#[derive(Debug)]
struct Input {
    width: usize,
    height: usize,
    map: Vec<Vec<bool>>,
}

#[derive(Debug)]
struct ParseError;

impl FromStr for Input {
    type Err = ParseError;
    fn from_str(s: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        let map: Vec<Vec<bool>> = s.split("\n")
            .filter(|l| !l.is_empty())
            .map(|l| l.trim().chars().map(|c| c == '#').collect()
            ).collect();
        return Ok(Input{width: map[0].len(), height:map.len(), map:map})
    }
}

fn parse_input(input: &str) -> Input {
    input.parse().expect("Should be able to parse input")
}

fn trees_hit(input: &Input, dx: usize, dy: usize) -> u32 {
    let mut x = 0;
    let mut y = 0;
    let mut count = 0;
    //println!("{:?}", *input);
    while y < input.height {
        //println!("{},{}: {}", x, y, input.map[y][x]);
        if input.map[y][x] {
            count += 1;
        }
        x += dx;
        x %= input.width;
        y += dy;
    }
    return count;
}

pub fn part1() -> u32 {
    let input = parse_input(include_str!("inputs/day3.txt"));
    trees_hit(&input, 3, 1)
}

#[test]
fn part1_works() {
    let result = part1();
    assert_eq!(result, 181)
}

#[test]
fn part2_sample_works() {
    let input = parse_input(include_str!("inputs/day3.sample"));
    let slopes = [(1,1), (3,1), (5,1), (7,1), (1,2)];
    let product = slopes.iter()
        .map(|(dx,dy)| trees_hit(&input, *dx, *dy))
        .fold(1, |acc, next| acc*next);
    assert_eq!(product, 336)
}

pub fn part2() -> u32 {
    let input = parse_input(include_str!("inputs/day3.txt"));
    let slopes = [(1,1), (3,1), (5,1), (7,1), (1,2)];
    slopes.iter()
        .map(|(dx,dy)| trees_hit(&input, *dx, *dy))
        .fold(1, |acc, next| acc*next)
}

#[test]
fn part2_works() {
    assert_eq!(part2(), 1260601650)
}