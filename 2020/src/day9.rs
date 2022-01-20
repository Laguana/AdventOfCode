use std::cmp::Ordering;

#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("inputs/day9.sample"));
    let result = find_non_sum(&input, 5);
    assert_eq!(result, 127)
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .trim()
        .replace("\r", "")
        .split("\n")
        .map(|l| l.parse::<u64>())
        .collect::<Result<_,_>>()
        .expect("Should be parsable")
}

fn find_non_sum(input: &Vec<u64>, preamble: usize) -> u64 {
    let mut index = preamble;
    while index < input.len() {
        let candidates = &input[index-preamble..index];
        let summand = input[index];
        if !is_summable(candidates, summand) {
            return summand
        }
        index += 1
    }
    panic!("Didn't find a candidate after reading whole list")
}

fn is_summable(preamble: &[u64], summand: u64) -> bool {
    let mut sorted: Vec<u64> = Vec::new();
    sorted.extend_from_slice(preamble);
    sorted.sort_unstable();
    let mut l_idx = 0;
    let mut r_idx = sorted.len()-1;
    while l_idx < r_idx {
        let a = sorted[l_idx];
        let b = sorted[r_idx];
        match (a+b).cmp(&summand) {
            Ordering::Equal => return true,
            Ordering::Less => {l_idx += 1;},
            Ordering::Greater => {r_idx -= 1;},
            
        }
    }
    return false;
}

pub fn part1() -> u64 {
    let input = parse_input(include_str!("inputs/day9.txt"));
    find_non_sum(&input, 25)
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 36845998)
}

#[test]
fn part2_sample_works() {
    let input = parse_input(include_str!("inputs/day9.sample"));
    let result = find_non_sum(&input, 5);
    let (min, max) = find_sum_range(&input, result);
    assert_eq!((min, max), (15, 40))
}

fn find_sum_range(input: &Vec<u64>, summand: u64) -> (u64, u64) {
    let mut l_idx = 0;
    let mut r_idx = 1;
    let mut sum = input[l_idx] + input[r_idx];

    while sum != summand {
        if sum < summand {
            r_idx += 1;
            sum += input[r_idx];
        } else {
            sum -= input[l_idx];
            l_idx += 1;
        }
    }

    return (input[l_idx], input[r_idx])
}

pub fn part2() -> u64 {
    let input = parse_input(include_str!("inputs/day9.txt"));
    let target = find_non_sum(&input, 25);
    let (a, b) = find_sum_range(&input, target);
    return a+b
}

#[test]
fn part2_works() {
    assert_eq!(part2(), 4830226)
}