use std::{str::FromStr, mem};

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(u32),
    Mul(u32),
    Square
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u32>,
    test: u32,
    operation: Operation,
    if_true: usize,
    if_false: usize,
    inspections: usize,
}

#[derive(Debug)]
struct Input {
    monkeys: Vec<Monkey>
}

#[derive(Debug)]
struct InputParseError;

impl FromStr for Input {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Input { monkeys: s.replace("\r", "").split("\n\n")
            .map(|m| {
                let lines: Vec<&str> = m.split("\n").map(|l| l.split(":").nth(1).unwrap()).collect();
                let items = lines[1].split(", ").map(|e| e.trim().parse().unwrap()).collect();
                let operation = match &lines[2][11..12] {
                    "+" => Operation::Add(lines[2][12..].trim().parse().unwrap()),
                    "*" => {
                            if &lines[2][13..] == "old" {
                                Operation::Square
                            } else {
                                Operation::Mul(lines[2][12..].trim().parse().unwrap())
                            }
                    }
                    _ => panic!("Bad input")
                };
                let test = lines[3][13..].trim().parse().unwrap();
                let if_true = lines[4][17..].trim().parse().unwrap();
                let if_false = lines[5][17..].trim().parse().unwrap();
                Monkey {
                    items,
                    test,
                    operation,
                    if_true,
                    if_false,
                    inspections: 0
                }
            }).collect()})
    }
}

fn parse_input(s: &str) -> Input {
    s.parse().expect("Unable to parse input")
}

fn count_activity(mut input: Input) -> usize {
    let n_monkeys = input.monkeys.len();
    for _round in 0..20 {
        //println!("ROUND {}", _round);
        /*for index in 0..n_monkeys {
            println!("Monkey {} {:?}", index, input.monkeys[index].items);
        }*/
        for index in 0..n_monkeys {
            let mut updates: Vec<Vec<u32>> = (0..n_monkeys).map(|_| vec![]).collect();
            {
                let cur_monkey = &mut input.monkeys[index];
                //println!("Monkey {} {:?}", index, cur_monkey.items);
                let mut items = vec![];
                {
                    mem::swap(&mut cur_monkey.items, &mut items);
                }
                for item in items {
                    //println!(" Inspecting {}", item);
                    cur_monkey.inspections += 1;
                    let post_op = match cur_monkey.operation {
                        Operation::Add(v) => item + v,
                        Operation::Mul(v) => item * v,
                        Operation::Square => item * item,
                    };
                    //println!("  post-op {}", post_op);
                    let post_relax = post_op / 3;
                    //println!("  post-relax {}", post_relax);
                    let destination = if post_relax % cur_monkey.test == 0 {
                        cur_monkey.if_true
                    } else {
                        cur_monkey.if_false
                    };
                    assert_ne!(index, destination);
                    //println!(" thrown to {}", destination);
                    updates[destination].push(post_relax);
                }
            }
            for i2 in 0..n_monkeys {
                input.monkeys[i2].items.extend(&updates[i2]);
            }
        }
    }
    let mut inspections: Vec<usize> = input.monkeys.iter().map(|m| m.inspections).collect();
    inspections.sort_by(|a,b| b.cmp(a));
    inspections[0] * inspections[1]
}

#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("sample.txt"));

    let result = count_activity(input);
    assert_eq!(result, 10605);
}

pub fn part1() -> usize {
    let input = parse_input(include_str!("input.txt"));
    count_activity(input)
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 101436)
}

#[test]
fn part2_sample_works() {
    let input = parse_input(include_str!("sample.txt"));

    
}

pub fn part2() -> u32 {
    let input = parse_input(include_str!("input.txt"));
    0
}

#[test]
fn part2_works() {
    assert_eq!(part2(), 0)
}
