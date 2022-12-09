use std::{str::FromStr, collections::HashSet};

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up(u8),
    Down(u8),
    Left(u8),
    Right(u8),
}

fn get_delta(dir: &Direction) -> (i16, i16) {
    match dir {
        Direction::Up(_) => (0,-1),
        Direction::Down(_) => (0,1),
        Direction::Left(_) => (-1,0),
        Direction::Right(_) => (1,0)
    }
}

fn get_dist(dir: &Direction) -> u8 {
    match dir {
        Direction::Up(v) => *v,
        Direction::Down(v) => *v,
        Direction::Left(v) => *v,
        Direction::Right(v) => *v
    }
}

#[derive(Debug)]
struct Input {
    steps: Vec<Direction>
}

#[derive(Debug)]
struct InputParseError;

impl FromStr for Input {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok( Input { steps: s.replace("\r","").trim().split("\n")
            .map(|l| {
                let dist: u8 = l[2..].parse().unwrap();
                match l.chars().nth(0).unwrap() {
                    'U' => Direction::Up(dist),
                    'D' => Direction::Down(dist),
                    'L' => Direction::Left(dist),
                    'R' => Direction::Right(dist),
                    _ => panic!("Invalid input")
                }
            }).collect()})
    }
}

fn parse_input(s: &str) -> Input {
    s.parse().expect("Unable to parse input")
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    x: i16,
    y: i16
}

fn move_knot_internal(knots: &mut [Position], dx: i16, dy: i16) {
    if knots.len() == 0 {
        return;
    }
    knots[0].x += dx;
    knots[0].y += dy;
    
    if knots.len() == 1 {
        return;
    }
    // recursive case
    let diff = (knots[0].x-knots[1].x).abs() + (knots[0].y-knots[1].y).abs();
    if diff < 2 {
        return;
    } else if diff == 2 {
        if knots[0].x == knots[1].x {
            // move toward in y direction, should be dy
            move_knot_internal(&mut knots[1..], 0, dy);
            return;
        }
        if knots[0].y == knots[1].y {
            // move toward in x direction, should be dx
            move_knot_internal(&mut knots[1..], dx, 0);
        }
        // both x and y differ, so it must actually be adjacent; no move necessary
        return;
    } else {
        // We need to do a diagonal move of 2 to get closer
        let dx = if knots[0].x < knots[1].x { -1 } else { 1 };
        let dy = if knots[0].y < knots[1].y { -1 } else { 1 };
        move_knot_internal(&mut knots[1..], dx, dy);
    }
}

fn count_tail_positions(input: &Input, n_knots: u8) -> usize {
    let mut knots: Vec<Position> = vec![];
    for _ in 0..n_knots {
        knots.push(Position{x: 0, y: 0});
    }
    let mut visited = HashSet::new();

    visited.insert(knots.last().unwrap().clone());

    for step in &input.steps {
        let (dx, dy) = get_delta(step);
        let count = get_dist(step);
        for _ in 0..count {
            move_knot_internal(&mut knots[..], dx, dy);
            visited.insert(knots.last().unwrap().clone());
        }
    }

    visited.len()
}

#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("sample.txt"));

    let result = count_tail_positions(&input, 2);
    assert_eq!(result, 13)
}

pub fn part1() -> usize {
    let input = parse_input(include_str!("input.txt"));
    count_tail_positions(&input, 2)
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 5878)
}

#[test]
fn part2_sample_works() {
    let input = parse_input(include_str!("sample.txt"));
    let result = count_tail_positions(&input, 10);
    assert_eq!(result, 1)
    
}

pub fn part2() -> usize {
    let input = parse_input(include_str!("input.txt"));
    count_tail_positions(&input, 10)
}

#[test]
fn part2_works() {
    assert_eq!(part2(), 2405)
}
