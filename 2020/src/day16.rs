use std::collections::HashSet;

#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("inputs/day16.sample"));
    let result = sum_invalid(&input);
    assert_eq!(result, 71)
}

type Ticket = Vec<u64>;

#[derive(Debug)]
struct Input {
    rules: Vec<(String, (u64, u64, u64, u64))>,
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
                .map(|r| r.split("-").map(|n| n.parse::<u64>().unwrap()))
                .flatten()
                .collect::<Vec<_>>();

            (name, (ranges[0], ranges[1], ranges[2], ranges[3]))
        })
        .collect::<Vec<(String, (u64, u64, u64, u64))>>();
    let my_ticket = parts[1]
        .split("\n")
        .nth(1)
        .expect("Should have a header")
        .split(",")
        .map(|e| e.parse::<u64>().unwrap())
        .collect::<Ticket>();
    let other_tickets = parts[2]
        .split("\n")
        .skip(1)
        .map(|l| {
            l.split(",")
                .map(|e| e.parse::<u64>().unwrap())
                .collect::<Ticket>()
        })
        .collect();

    Input {
        rules: rules,
        my_ticket: my_ticket,
        other_tickets: other_tickets,
    }
}

fn sum_invalid(input: &Input) -> u64 {
    input.other_tickets.iter().fold(0, |acc, e| {
        e.iter().fold(acc, |acc, v| {
            if input.rules.iter().any(|(_, (a, b, c, d))| {
                //println!("{} <?> {} {}, {} {}", v, a, b, c, d);
                (a <= v && v <= b) || (c <= v && v <= d)
            }) {
                acc
            } else {
                acc + v
            }
        })
    })
}

pub fn part1() -> u64 {
    let input = parse_input(include_str!("inputs/day16.txt"));
    sum_invalid(&input)
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 24980)
}

fn ticket_valid(rules: &Vec<(String, (u64, u64, u64, u64))>, ticket: &Ticket) -> bool {
    ticket.iter().all(|v| {
        rules
            .iter()
            .any(|(_, (a, b, c, d))| (a <= v && v <= b) || (c <= v && v <= d))
    })
}

fn decode_ticket(input: &Input) -> Vec<usize> {
    let mut candidates = input
        .my_ticket
        .iter()
        .map(|_| (0..input.rules.len()).collect::<HashSet<usize>>())
        .collect::<Vec<_>>();

    let valid_tickets = input
        .other_tickets
        .iter()
        .filter(|t| ticket_valid(&input.rules, t));

    for t in valid_tickets {
        for (i, &v) in t.iter().enumerate() {
            if candidates[i].len() > 1 {
                candidates[i] = candidates[i]
                    .iter()
                    .filter(|&&j| {
                        let (_, (a, b, c, d)) = input.rules[j];
                        (a <= v && v <= b) || (c <= v && v <= d)
                    })
                    .map(|&x|x)
                    .collect();
            }
        }
    }

    loop {
        {
            if candidates.iter().all(|s| s.len() == 1) {
                break;
            }
        }
        let mut removed_any = false;
        for i in 0..candidates.len() {
            if candidates[i].len() == 1 {
                let to_remove = *candidates[i].iter().nth(0).unwrap();
                for j in 0..candidates.len() {
                    if j != i {
                        removed_any |= candidates[j].remove(&to_remove);
                    }
                }
            }
        }
        assert_eq!(removed_any, true);
    }

    //println!("Candidates: {:?}", candidates);

    assert_eq!(candidates.iter().all(|s| s.len() == 1), true);

    return candidates.iter().map(|s| *s.iter().nth(0).unwrap()).collect()
}

fn departure_prod(input: &Input) -> u64 {
    let decoded = decode_ticket(input);

    input.my_ticket.iter().enumerate().fold(1, |acc,(i,e)| {
        let rule_index = decoded[i];
        let (k,_) = &input.rules[rule_index];
        if k.len() > 9 && &k[..9] == "departure" {
            //println!("{}: {} * {}", k, acc, e);
            acc * e
        } else {
            acc
        }
    })
}

pub fn part2() -> u64 {
    let input = parse_input(include_str!("inputs/day16.txt"));
    departure_prod(&input)
}

#[test]
fn part2_works() {
    assert_eq!(part2(), 809376774329)
}