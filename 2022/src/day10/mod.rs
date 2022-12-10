use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Noop,
    AddX(i8),
}

#[derive(Debug)]
struct Input {
    sequence: Vec<Instruction>
}

#[derive(Debug)]
struct InputParseError;

impl FromStr for Input {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Input { sequence: s.replace("\r", "").trim().split("\n")
            .map(|l| {
                if l == "noop" {
                    return Instruction::Noop
                }
                let operand: i8 = l[5..].parse().expect("Couldn't parse");
                Instruction::AddX(operand)
            }).collect()})
    }
}

fn parse_input(s: &str) -> Input {
    s.parse().expect("Unable to parse input")
}

fn signal_strength(input: &Input) -> i16 {
    let mut cycle: i16 = 1;
    let mut score = 0;
    let mut register = 1;

    for inst in &input.sequence {
        match inst {
            Instruction::Noop => {
                if cycle % 40 == 20 {
                    //println!("Noop {} {} {}", register, cycle, score);
                    score += register * cycle;
                }
                cycle += 1;
            },
            Instruction::AddX(v) => {
                let phase = cycle % 40;
                if phase == 19 {
                    //println!("End of add {} {} {} {}", v, register, cycle, score);
                    score += register * (cycle+1)
                } else if phase == 20 {
                    //println!("Start of add {} {} {} {}", v, register, cycle, score);
                    score += register * cycle;
                }

                register += *v as i16;
                cycle += 2;
            }
        }
    }

    assert!(cycle >= 220);

    score
}

#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("sample.txt"));

    let result = signal_strength(&input);
    assert_eq!(result, 13140)
}

pub fn part1() -> i16 {
    let input = parse_input(include_str!("input.txt"));
    signal_strength(&input)
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 15140)
}

fn render_image(input: &Input) -> Vec<bool> {
    let mut cycle: i16 = 0;
    let mut register = 1;
    let mut result = vec![];

    for inst in &input.sequence {
        match inst {
            Instruction::Noop => {
                result.push(((cycle%40) - register).abs() <= 1);
                cycle += 1;
            },
            Instruction::AddX(v) => {
                //println!("AddX {} {} {}", v, cycle, register);
                result.push(((cycle%40) - register).abs() <= 1);
                cycle += 1;
                result.push(((cycle%40) - register).abs() <= 1);
                cycle += 1;
                register += *v as i16;
            },
        }
    }
    result
}

#[test]
fn part2_sample_works() {
    let input = parse_input(include_str!("sample.txt"));

    let result = render_image(&input);
    let s = String::from_iter(result.into_iter()
        .map(|c| if c { '#' } else {'.'}));
    assert_eq!(s, include_str!("sample_expected.txt"));
}

pub fn part2() -> String {
    let input = parse_input(include_str!("input.txt"));
    let result = render_image(&input);
    result.into_iter().as_slice().chunks(40)
        .map(|chunk| String::from_iter(chunk.iter().map(|c| if *c { '#' } else {'.'})))
        .reduce(|a, b| a + "\n" + &b).expect("Should have multiple lines")
}



#[test]
fn part2_works() {
    assert_eq!(part2(), include_str!("input_expected.txt").replace("\r", ""))
}
