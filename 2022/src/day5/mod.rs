use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct Instruction {
    count: u8,
    from: usize,
    to: usize
}

#[derive(Debug)]
struct Input {
    stacks: Vec<Vec<char>>,

    instructions: Vec<Instruction>
}

#[derive(Debug)]
struct InputParseError;

fn parse_stacks(s: &str) -> Vec<Vec<char>> {
    let mut lines: Vec<&str> = s.split("\n").collect();
    lines.reverse();
    
    let mut stacks: Vec<Vec<char>> = lines[0].chars()
        .filter(|c| *c != ' ')
        .map(| _| Vec::new())
        .collect();

    let num_stacks = stacks.len();
    for line in &lines[1..] {
        //println!("{}", line);
        //println!("{}", line.chars().count());
        let mut chars = line.chars();
        chars.next();
        for index in 0..num_stacks {
            let c = chars.next().unwrap();
            if c != ' ' {
                stacks[index].push(c);
            }
            //println!("{}:{}", index, c);
            chars.next();
            chars.next();
            chars.next();
        }

        //println!("{:?}", stacks);
    }

    stacks
}

impl FromStr for Input {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let no_cr = s.replace("\r", "");
        let mut major_parts = no_cr.split("\n\n");
        let initial_state = major_parts.nth(0).unwrap();
        let instruction_str = major_parts.nth(0).unwrap();

        let stacks: Vec<Vec<char>> = parse_stacks(initial_state);

        let instructions: Vec<Instruction> = instruction_str.trim().split("\n")
            .map(|l| {
                let parts: Vec<&str> = l.split(" ").collect();
                let count: u8 = parts[1].parse().unwrap();
                let from: usize = parts[3].parse().unwrap();
                let to: usize = parts[5].parse().unwrap();
                Instruction {count, from, to}
            }).collect();

        Ok(Input{stacks, instructions})
    }
}

fn parse_input(s: &str) -> Input {
    s.parse().expect("Unable to parse input")
}

fn run_stacks(input: &Input) -> Vec<Vec<char>> {
    let mut result = input.stacks.clone();
    for instruction in &input.instructions {
        //println!("{:?}", result);
        //println!("{:?}", instruction);
        for _ in 0..instruction.count {
            let v = result[instruction.from-1].pop().expect("Pulled more than exist");
            //println!(" {:?}", v);
            result[instruction.to-1].push(v);
        }
        //println!("{:?}\n\n", result);
    }

    result
}

fn run_stacks_2(input: &Input) -> Vec<Vec<char>> {
    let mut result = input.stacks.clone();
    for instruction in &input.instructions {
        //println!("{:?}", result);
        //println!("{:?}", instruction);
        let mut temp = Vec::new();
        for _ in 0..instruction.count {
            let v = result[instruction.from-1].pop().expect("Pulled more than exist");
            temp.push(v);
        }
        for _ in 0..instruction.count {
            let v = temp.pop().expect("We just made this!");
            result[instruction.to-1].push(v);
        }
        //println!("{:?}\n\n", result);
    }

    result
}

fn get_top_stacks(stacks: &Vec<Vec<char>>) -> String {
    String::from_iter(stacks.iter().map(|s| s.last().unwrap()))
}

#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("sample.txt"));

    assert_eq!(input.stacks.len(), 3);

    let stacks = run_stacks(&input);
    let result = get_top_stacks(&stacks);
    assert_eq!(result, "CMZ");

    
}

pub fn part1() -> String {
    let input = parse_input(include_str!("input.txt"));
    
    let stacks = run_stacks(&input);
    get_top_stacks(&stacks)
}

#[test]
fn part1_works() {
    assert_eq!(part1(), "TQRFCBSJJ")
}

#[test]
fn part2_sample_works() {
    let input = parse_input(include_str!("sample.txt"));

    let stacks = run_stacks_2(&input);
    let result = get_top_stacks(&stacks);
    assert_eq!(result, "MCD");
}

pub fn part2() -> String {
    let input = parse_input(include_str!("input.txt"));
    let stacks = run_stacks_2(&input);
    get_top_stacks(&stacks)
}

#[test]
fn part2_works() {
    assert_eq!(part2(), "RMHFJNVFP")
}
