use std::cmp::Ordering;
use std::str::FromStr;

fn find_sum(expense_report: &[u32], target_sum: u32) -> Option<(u32, u32)>  {
    let mut sorted: Vec<u32> = Vec::new();
    sorted.extend_from_slice(expense_report);
    sorted.sort_unstable();
    let mut i = 0;
    let mut j = expense_report.len() - 1;
    while i < j {
        let a = sorted[i];
        let b = sorted[j];
        let sum = a + b;

        match sum.cmp(&target_sum) {
            Ordering::Less => {
                i += 1;
                continue;
            }
            Ordering::Greater => {
                j -= 1;
                continue;
            }
            Ordering::Equal => return Some((a, b))
        }
    }
    return None
}

#[test]
fn part1_sample_works() {
    let input = [1721, 979, 366, 299, 675, 1456];
    let (a, b) = find_sum(&input[..], 2020)
        .expect("Should find an answer");
    assert_eq!(a * b, 514579)
}

struct Input {
    expense_report: Vec<u32>,
}

impl FromStr for Input {
    fn from_str(s: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        let entries: Result<Vec<u32>, std::num::ParseIntError> = 
            s.split("\n")
                .filter(|s| !s.is_empty())
                .map(|s| s.trim().parse())
                .collect();

        match entries {
            Ok(e) => Ok(Input{expense_report: e}),
            Err(e) => Err(e)
        }
    }
    type Err = std::num::ParseIntError;
}

fn parse_input() -> Input {
    let input = include_str!("inputs/day1.txt");
    return input.parse().expect("Unable to parse input");
}

pub fn part1() -> u32 {
    let input = parse_input();
    let (a, b) = find_sum(&input.expense_report[..], 2020)
        .expect("Should find an answer");
    return a * b;
}

#[test]
fn part1_works() {
    let result = part1();
    assert_eq!(result, 838624);
}

fn find_3_sum(input: &Input, target_sum: u32) -> Option<(u32, u32, u32)> {
    let slice = &input.expense_report[..];
    for i in slice {
        if *i > target_sum {
            continue;
        }
        match find_sum(slice, target_sum-i) {
            Some((a,b)) => return Some((a,b,*i)),
            None => ()
        }
    }
    return None
}

#[test]
fn part2_sample_works() {
    let input = vec![1721, 979, 366, 299, 675, 1456];
    let (a, b,c) = find_3_sum(&Input{expense_report: input}, 2020)
        .expect("Should find an answer");
    assert_eq!(a * b * c, 241861950)
}

pub fn part2() -> u32{
    let input = parse_input();
    let (a, b, c) = find_3_sum(&input, 2020)
        .expect("Should find an answer");
    return a * b * c;
}

#[test]
fn part2_works() {
    let result = part2();
    assert_eq!(result, 52764180)
}