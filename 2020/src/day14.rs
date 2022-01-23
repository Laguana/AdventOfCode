use std::collections::HashMap;

#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("inputs/day14.sample"));
    let result = sum_addresses(&input);
    assert_eq!(result, 165)
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum MaskBit {
    X,
    On,
    Off,
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Instruction {
    Mask([MaskBit; 36]),
    Assign(u64, u64),
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.trim().split("\n").map(|l| {
        let parts = l.trim().split(" = ").collect::<Vec<_>>();
        let lhs = parts[0];
        if lhs == "mask" {
            let mut mask = [MaskBit::X;36];
            for (i,c) in parts[1].chars().enumerate() {
                mask[i] = match c {
                    'X' => MaskBit::X,
                    '1' => MaskBit::On,
                    '0' => MaskBit::Off,
                    _ => panic!("Invalid input")
                }
            }
            Instruction::Mask(mask)
        } else if &lhs[..4] != "mem[" {
            panic!("Invalid input")
        } else {
            let addr = lhs[4..lhs.len()-1].parse::<u64>().expect("Should be an int");
            let dest = parts[1].parse().expect("Should be an int");
            Instruction::Assign(addr,dest)
        }
    }).collect()
}

fn sum_addresses(input: &Vec<Instruction>) -> u64 {
    let mut memory_map = HashMap::new();
    let mut mask_on: u64 = 0;
    let mut mask_off: u64 = !0;

    for i in input {
        //println!("{:?}, {}, {}",i, mask_on, !mask_off);
        match i {
            Instruction::Mask(bits) => {
                for (i,v) in bits.iter().enumerate() {
                    let index = 35-i;
                    let bit = 1<<index;
                    match v {
                        MaskBit::X => {
                            mask_on &= !bit;
                            mask_off |= bit;
                        },
                        MaskBit::On => {
                            mask_on |= bit;
                            mask_off |= bit;
                        },
                        MaskBit::Off => {
                            mask_on &= !bit;
                            mask_off &= !bit;
                        }
                    }
                };
            },
            Instruction::Assign(to, value) => {
                memory_map.insert(to, (value | mask_on) & mask_off);
            }
        }
    }
    
    memory_map.values().fold(0, |acc, v| acc+v)
}

pub fn part1() -> u64 {
    let input = parse_input(include_str!("inputs/day14.txt"));
    sum_addresses(&input)
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 6317049172545)
}