
#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("inputs/day12.sample"));
    let result = execute_steps(&input);
    assert_eq!(result, (17,-8))
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Instruction {
    East(i16),
    West(i16),
    North(i16),
    South(i16),
    Forward(i16),
    Left(i16),
    Right(i16),
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.trim().split("\n").map(|l| {
        let v = l[1..].trim().parse::<i16>().expect("Should be valid input");
        match l.chars().nth(0).unwrap() {
            'E' => Instruction::East(v),
            'W' => Instruction::West(v),
            'N' => Instruction::North(v),
            'S' => Instruction::South(v),
            'F' => Instruction::Forward(v),
            'L' => Instruction::Left(v),
            'R' => Instruction::Right(v),
            _ => panic!("Invalid instruction"),
        }
    }).collect()
}

fn execute_steps(input: &Vec<Instruction>) -> (i16, i16) {
    let mut x = 0;
    let mut y = 0;
    let mut heading = 0;
    for i in input {
        match i {
            Instruction::East(v) => x += v,
            Instruction::West(v) => x -= v,
            Instruction::North(v) => y += v,
            Instruction::South(v) => y -= v,
            Instruction::Left(v) => heading = (heading + 360 - v)%360,
            Instruction::Right(v) => heading = (heading + v)%360,
            Instruction::Forward(v) => {
                let heading = (heading/90) % 4;
                match heading {
                    0 => x += v,
                    1 => y -= v,
                    2 => x -= v,
                    3 => y += v,
                    _ => panic!("arithmetic should have reduced to one of the previous cases")
                }
            }
        }
    }
    (x,y)
}

pub fn part1() -> i16 {
    let input = parse_input(include_str!("inputs/day12.txt"));
    let (x,y) = execute_steps(&input);
    return x.abs() + y.abs();
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 521)
}