use std::cell::RefCell;

#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("inputs/day8.sample"));
    let result = get_acc_when_looping(&input);
    assert_eq!(result, 5)
}

#[derive(Debug)]
enum Instruction {
    Nop,
    Acc(i32),
    Jmp(i32),
}

#[derive(Debug)]
struct ParseError;

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .trim()
        .replace("\r", "")
        .split("\n")
        .map(|l| match &l[..3] {
            "nop" => Ok(Instruction::Nop),
            "jmp" => l[4..].parse().map(|i| Instruction::Jmp(i)),
            "acc" => l[4..].parse().map(|i| Instruction::Acc(i)),
            _ => "bad instruction".parse::<i32>().map(|_| Instruction::Nop),
        })
        .collect::<Result<_, _>>()
        .expect("Should be parsable")
}

fn get_acc_when_looping(input: &Vec<Instruction>) -> i32 {
    let mut acc: i32 = 0;
    let mut ip: usize = 0;
    let mut state = input
        .iter()
        .map(|e| RefCell::new((e, false)))
        .collect::<Vec<_>>();

    while state[ip].borrow().1 == false {
        //println!("{}: {:?} {}", ip, state[ip].borrow(), acc);
        state[ip].get_mut().1 = true;
        match state[ip].borrow().0 {
            Instruction::Nop => {
                ip += 1;
            }
            Instruction::Acc(v) => {
                acc += v;
                ip += 1;
            }
            Instruction::Jmp(v) => {
                let i32_ip = i32::try_from(ip).expect("Shouldn't be huge");
                ip = usize::try_from(i32_ip + v).expect("Shouldn't be negative");
            }
        }
    }

    return acc;
}

pub fn part1() -> i32 {
    let input = parse_input(include_str!("inputs/day8.txt"));
    return get_acc_when_looping(&input)
}


#[test]
fn part1_works() {
    assert_eq!(part1(), 1394)
}