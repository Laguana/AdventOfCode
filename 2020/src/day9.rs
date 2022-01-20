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