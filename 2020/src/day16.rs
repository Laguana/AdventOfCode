use std::collections::HashMap;

#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("inputs/day16.sample"));
    let result = sum_invalid(&input);
    assert_eq!(result, 71)
}

type Ticket = Vec<u32>;

#[derive(Debug)]
struct Input {
    rules: HashMap<String, (u32, u32, u32, u32)>,
    my_ticket: Ticket,
    other_tickets: Vec<Ticket>,
}

fn parse_input(input: &str) -> Input {
    let t = input.trim().replace("\r", "");
    let parts = t.split("\n\n").collect::<Vec<_>>();
    if parts.len() != 3 {
        panic!("Invalid input, need 3 sections");
    }

    let rules = parts[0]
        .split("\n")
        .map(|l| {
            let parts = l.split(": ").collect::<Vec<_>>();
            let name = parts[0].to_string();
            let ranges = parts[1]
                .split(" or ")
                .map(|r| r.split("-").map(|n| n.parse::<u32>().unwrap()))
                .flatten()
                .collect::<Vec<_>>();

            (name, (ranges[0], ranges[1], ranges[2], ranges[3]))
        })
        .collect::<HashMap<String, (u32, u32, u32, u32)>>();
    let my_ticket = parts[1].split("\n")
        .nth(1).expect("Should have a header")
        .split(",")
        .map(|e| e.parse::<u32>().unwrap())
        .collect::<Ticket>();
    let other_tickets = parts[2]
        .split("\n")
        .skip(1)
        .map(|l| {
            l.split(",")
                .map(|e| e.parse::<u32>().unwrap())
                .collect::<Ticket>()
        })
        .collect();

    Input {
        rules: rules,
        my_ticket: my_ticket,
        other_tickets: other_tickets,
    }
}

fn sum_invalid(input: &Input) -> u32 {
    input.other_tickets.iter().fold(0, |acc, e| {
        e.iter().fold(acc, |acc, v| {
            if input.rules.iter().any(|(_, (a,b,c,d))| {
                //println!("{} <?> {} {}, {} {}", v, a, b, c, d);
                (a<=v && v <= b) || (c <= v && v <= d)
            }) {
                acc
            } else {
                acc + v
            }
        })
    })
}

pub fn part1() -> u32 {
    let input = parse_input(include_str!("inputs/day16.txt"));
    sum_invalid(&input)
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 24980)
}