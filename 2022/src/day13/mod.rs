use std::{str::FromStr, cmp::Ordering};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Int(u16),
    List(Vec<Packet>)
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::Int(l), Packet::Int(r)) => Some(l.cmp(r)),
            (Packet::List(l), Packet::List(r)) => {
                let mut l_i = l.iter();
                let mut r_i = r.iter();
                loop {
                    match (l_i.next(), r_i.next()) {
                        (None, None) => return Some(Ordering::Equal),
                        (None, Some(_)) => return Some(Ordering::Less),
                        (Some(_), None) => return Some(Ordering::Greater),
                        (Some(l), Some(r)) => {
                            match l.partial_cmp(r) {
                                Some(Ordering::Equal) => continue,
                                Some(v) => return Some(v),
                                None => return None,
                            }
                        },
                    }
                }
            },
            (Packet::Int(l), r) => Packet::List(vec![Packet::Int(*l)]).partial_cmp(r),
            (l, Packet::Int(r)) => l.partial_cmp(&Packet::List(vec![Packet::Int(*r)])),            
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(other) {
            Some(v) => v,
            None => panic!("Disorder")
        }
    }
}

fn parse_int_packet_from_iter(it: &mut std::str::Chars, v: u16) -> (Packet, Option<char>) {
    let mut v = v;
    loop {
        let c = it.next();
        if c == None {
            return (Packet::Int(v), None)
        }
        let c = c.unwrap();
        match c {
            '0'..='9' => v = v * 10 + (c as u16 - '0' as u16),
            ','|']' => return (Packet::Int(v), Some(c)),
            _ => panic!("Bad input")
        }
    }
}

fn parse_list_from_iter(it: &mut std::str::Chars) -> Packet {
    let mut v = vec![];
    let mut next = it.next();
    if next == Some(']') {
        return Packet::List(v);
    }
    loop {
        let (p, following) = parse_packet_from_iter(it, next);
        v.push(p);
        match following {
            None => panic!("Unterminated list? {:?} {:?}", v, it),
            Some(',') => {
                next = it.next();
                continue;
            },
            Some(']') => return Packet::List(v),
            _ => panic!("Bad list input; {:?}", following)
        }
    }
}

fn parse_packet_from_iter(it: &mut std::str::Chars, next: Option<char>) -> (Packet, Option<char>) {
    match next {
        Some('[') => {
            let p = parse_list_from_iter(it);
            (p, it.next())
        },
        Some(v) => {
            parse_int_packet_from_iter(it, v as u16-'0' as u16)
        }
        None => panic!("Need input"),
    }
}

fn parse_packet(s: &str) -> Packet {
    //println!("Parsing {}", s);
    let s = s.trim();
    let mut it = s.chars();
    let first = it.next();
    let (p, _) = parse_packet_from_iter(&mut it, first);
    p
}

#[derive(Debug)]
struct Input {
    packets: Vec<Packet>
}

#[derive(Debug)]
struct InputParseError;

impl FromStr for Input {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Input{ packets: s.lines().filter_map(|l| {
            if l.trim().is_empty() { None } else { Some(parse_packet(l))}
        }).collect()})
    }
}

fn parse_input(s: &str) -> Input {
    s.parse().expect("Unable to parse input")
}

fn score_indices(input: &Input) -> usize {
    let mut idx = 1;
    let mut score = 0;
    for pair in input.packets[..].chunks(2) {
        let comparison = pair[0].partial_cmp(&pair[1]);
        //println!("{:?} {:?} {:?}", pair[0], comparison, pair[1]);
        match comparison {
            Some(Ordering::Less) => {
                score += idx;
                //println!("  {} {}", score, idx);
            },
            _ => (),
        }
        idx += 1;
    }
    score
}

#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("sample.txt"));

    let result = score_indices(&input);
    assert_eq!(result, 13)
}

pub fn part1() -> usize {
    let input = parse_input(include_str!("input.txt"));
    score_indices(&input)
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 5350)
}

fn sort_and_find_divider(input: &Input) -> usize {
    let dividers = vec![
        Packet::List(vec![Packet::List(vec![Packet::Int(2)])]),
        Packet::List(vec![Packet::List(vec![Packet::Int(6)])])];

    let mut sorted = [input.packets.clone(), dividers.clone()].concat();
    sorted.sort();
    let mut result = 1;
    let mut idx = 1;
    for p in sorted {
        if p == dividers[0] || p == dividers[1] {
            result *= idx;
        }
        idx += 1;
    }
    result
}

#[test]
fn part2_sample_works() {
    let input = parse_input(include_str!("sample.txt"));

    let result = sort_and_find_divider(&input);
    assert_eq!(result, 140);
}

pub fn part2() -> usize {
    let input = parse_input(include_str!("input.txt"));
    sort_and_find_divider(&input)
}

#[test]
fn part2_works() {
    assert_eq!(part2(), 19570)
}
