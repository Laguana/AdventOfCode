use std::collections::HashMap;

#[test]
fn part1_sample_works() {
    let input = parse_input("3,1,2");
    let result = compute_2020th(&input);
    assert_eq!(result, 1836);
}

fn parse_input(input: &str) -> Vec<u64> {
    input.split(",").map(|s| s.parse::<u64>()).collect::<Result<_,_>>().expect("Should parse")
}

fn compute_2020th(input: &Vec<u64>) -> u64 {
    let mut visited : HashMap<u64, u64> = HashMap::new();
    let mut step = 0;
    let mut prev = 0;
    let mut prev_said = 0;
    for &i in input {
        step += 1;
        visited.insert(i, step);
        prev = i;
    }
    loop {
        let next = match prev_said {
            0 => 0,
            n => step-n
        };
        visited.insert(prev, step);
        step += 1;

        if step == 2020 {
            return next;
        }
        prev = next;
        prev_said = *visited.get(&prev).unwrap_or(&0);
    }
}

pub fn part1() -> u64 {
    let input = parse_input("14,8,16,0,1,17");
    compute_2020th(&input)
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 240)
}

fn compute_30000000th(input: &Vec<u64>) -> u64 {
    let mut visited : HashMap<u64, u64> = HashMap::new();
    let mut step = 0;
    let mut prev = 0;
    let mut prev_said = 0;
    for &i in input {
        step += 1;
        visited.insert(i, step);
        prev = i;
    }
    loop {
        let next = match prev_said {
            0 => 0,
            n => step-n
        };
        visited.insert(prev, step);
        step += 1;

        if step % 10000 == 0 {
            println!("{}: {}", step, next);
        }

        if step == 30_000_000 {
            return next;
        }
        prev = next;
        prev_said = *visited.get(&prev).unwrap_or(&0);
    }
}

#[allow(dead_code)]
//#[test]
fn part2_sample_works() {
    let input = parse_input("0,3,6");
    assert_eq!(compute_30000000th(&input), 175594)
}

//#[allow(dead_code)]
pub fn part2() -> u64 {
    let input = parse_input("14,8,16,0,1,17");
    compute_30000000th(&input)
}

#[allow(dead_code)]
//#[test]
fn part2_works() {
    assert_eq!(part2(), 505);       
}