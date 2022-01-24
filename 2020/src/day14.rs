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

// So this is what I thought would work.
// And it did! But slow.
/*

#[derive(Debug, Eq, PartialEq, Clone)]
enum MemTree {
    Leaf(u64),
    X(Box<MemTree>),
    Branch(Box<MemTree>, Box<MemTree>)
}

fn initialize_memory() -> MemTree {
    let mut ret = MemTree::Leaf(0);

    for _ in 0..36 {
        let ret_next = MemTree::X(Box::new(ret));
        ret = ret_next;
    }

    return ret;
}

fn set_with_mask(memory: &MemTree, mask: &[MaskBit;36], address: u64, depth: usize, value: u64) -> MemTree {
    match memory {
        MemTree::Leaf(_) => {
            MemTree::Leaf(value)
        },
        MemTree::X(t) => {
            let t2 = set_with_mask(t, mask, address, depth+1, value);
            match mask[depth] {
                MaskBit::X => {
                    MemTree::X(Box::new(t2))
                },
                MaskBit::On => {
                    MemTree::Branch(t.clone(), Box::new(t2))
                },
                MaskBit::Off => {
                    let bit_index = 35-depth;
                    let bit = (address >> bit_index) & 1;
                    if bit == 0 {
                        MemTree::Branch(Box::new(t2),  t.clone())
                    } else {
                        MemTree::Branch(t.clone(),  Box::new(t2))
                    }
                }
            }
        },
        MemTree::Branch(l,r) => {
            match mask[depth] {
                MaskBit::X => {
                    let l2 = set_with_mask(l, mask, address, depth+1, value);
                    let r2 = set_with_mask(r, mask, address, depth+1, value);
                    MemTree::Branch(Box::new(l2), Box::new(r2))
                },
                MaskBit::On => {
                    let r2 = set_with_mask(r, mask, address, depth+1, value);
                    MemTree::Branch(l.clone(), Box::new(r2))
                },
                MaskBit::Off => {
                    let bit_index = 35-depth;
                    let bit = (address >> bit_index) & 1;
                    if bit == 0 {
                        let l2 = set_with_mask(l, mask, address, depth+1, value);
                        MemTree::Branch(Box::new(l2),  r.clone())
                    } else {
                        let r2 = set_with_mask(r, mask, address, depth+1, value);
                        MemTree::Branch(l.clone(),  Box::new(r2))
                    }
                }
            }
        }
    }
    // 
}

fn sum_tree_values(memory: &MemTree) -> u64 {
    match memory {
        MemTree::Leaf(x) => *x,
        MemTree::Branch(l,r) => sum_tree_values(l) + sum_tree_values(r),
        MemTree::X(t) => 2*sum_tree_values(t)
    }
}

fn sum_part2_addresses(input: &Vec<Instruction>) -> u64 {
    // mask[i] is 0 -> no change
    //            1 -> addr[i] is on
    //            X -> addr[i] is both
    // represent memory as a 36-deep tree

    let mut memory = initialize_memory();
    let mut mask = [MaskBit::Off; 36];
    
    for i in input {
        match i {
            Instruction::Mask(bits) => {
                mask = *bits;
            },
            Instruction::Assign(to, v) => {
                memory = set_with_mask(&memory, &mask, *to, 0, *v);
            }
        }
    }
    sum_tree_values(&memory)
}
*/

// This is the 'dumb' approach of just adding all the different values, but it worked much better
// probably because the masks tended to have few Xs
fn sum_part2_addresses_2(input: &Vec<Instruction>) -> u64 {
    let mut memory = HashMap::new();
    let mut mask = [MaskBit::Off; 36];

    for i in input {
        match i {
            Instruction::Mask(bits) => {
                mask = *bits;
            },
            Instruction::Assign(to, v) => {
                for k in (0..36).fold(vec![0], |acc: Vec<u64>, e| {
                    let address_bit = (to >> (35-e)) & 1;
                    match mask[e] {
                        MaskBit::Off => {
                            // bit is address_bit
                            acc.into_iter().map(|v| v<<1|address_bit).collect()
                        },
                        MaskBit::On => {
                            acc.into_iter().map(|v| v<<1|1).collect()
                        },
                        MaskBit::X => {
                            // bit is both
                            acc.into_iter().map(|v| vec![v<<1|0, v<<1|1]).flatten().collect()
                        }
                    }
                }) {
                    memory.insert(k,v);
                }
            }
        }
    }

    return memory.values().fold(0, |acc, &v| acc+v)
}

#[test]
fn part2_sample_works() {
    let input = parse_input(include_str!("inputs/day14.sample2"));
    let result= sum_part2_addresses_2(&input);
    assert_eq!(result, 208)
}

pub fn part2() -> u64 {
    let input = parse_input(include_str!("inputs/day14.txt"));
    sum_part2_addresses_2(&input)
}

#[test]
fn part2_works() {
    assert_eq!(part2(), 3434009980379)
}