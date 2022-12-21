use std::{str::FromStr, collections::HashMap};

#[derive(Debug, Hash)]
enum Operation {
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String,String),
    Const(i64)
}

#[derive(Debug)]
struct Input {
    monkies: HashMap<String, Operation>
}

#[derive(Debug)]
struct InputParseError;

impl FromStr for Input {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Input { monkies: s.trim().lines()
            .map(|l| {
                let key = l[..4].to_string();
                let parts: Vec<_> = l[5..].trim().split(" ").collect();
                let value = if parts.len() == 1 {
                    Operation::Const(parts[0].parse().unwrap())
                } else {
                    match parts[1] {
                        "+" => Operation::Add(parts[0].to_string(), parts[2].to_string()),
                        "-" => Operation::Sub(parts[0].to_string(), parts[2].to_string()),
                        "*" => Operation::Mul(parts[0].to_string(), parts[2].to_string()),
                        "/" => Operation::Div(parts[0].to_string(), parts[2].to_string()),
                        _ => panic!("Bad input"),
                    }
                };
                (key, value)
            }
            ).collect()})
    }
}

fn parse_input(s: &str) -> Input {
    s.parse().expect("Unable to parse input")
}

fn compute_value(input: &Input, key: &str, humn_value: i64) -> i64 {
    if key == "humn" {
        return humn_value;
    }
    match input.monkies.get(key) {
        Some(Operation::Const(v)) => *v,
        Some(Operation::Add(l, r)) => {
            let l = compute_value(input, l, humn_value);
            let r = compute_value(input, r, humn_value);
            l + r
        },
        Some(Operation::Sub(l, r)) => {
            let l = compute_value(input, l, humn_value);
            let r = compute_value(input, r, humn_value);
            l - r
        },
        Some(Operation::Mul(l, r)) => {
            let l = compute_value(input, l, humn_value);
            let r = compute_value(input, r, humn_value);
            l * r
        },
        Some(Operation::Div(l, r)) => {
            let l = compute_value(input, l, humn_value);
            let r = compute_value(input, r, humn_value);
            l / r
        },
        None => todo!(),
    }
}

#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("sample.txt"));
    let humn = match input.monkies.get("humn") {
        Some(Operation::Const(v)) => *v,
        _ => panic!("Invalid input")
    };
    let result = compute_value(&input, "root", humn);
    assert_eq!(result, 152);
    
}

pub fn part1() -> i64 {
    let input = parse_input(include_str!("input.txt"));
    let humn = match input.monkies.get("humn") {
        Some(Operation::Const(v)) => *v,
        _ => panic!("Invalid input")
    };
    compute_value(&input, "root", humn)
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 75147370123646)
}

fn get_parts(op: &Operation) -> (String, String) {
    match op {
        Operation::Add(l, r) => (l.clone(),r.clone()),
        Operation::Sub(l, r) => (l.clone(),r.clone()),
        Operation::Mul(l, r) => (l.clone(),r.clone()),
        Operation::Div(l, r) => (l.clone(),r.clone()),
        Operation::Const(_) => panic!("Not something with parts"),
    }
}

fn newton(input: &Input) -> i64 {
    let (left, right) = get_parts(input.monkies.get("root").unwrap());
    let mut vprev = 0;

    let get_delta_at = |v| {
        let l = compute_value(&input, left.as_str(), v);
        let r = compute_value(&input, right.as_str(), v);
        l-r
    };

    let mut vnext = 10;

    let mut dprev = get_delta_at(vprev);
    if dprev == 0 {
        return vprev;
    }

    loop {

        let dnext = get_delta_at(vnext);
        if dnext == 0 {
            // now minimize I guess?
            let dx = -1;
            loop {
                if get_delta_at(vnext+dx) == 0 {
                    vnext = vnext+dx;
                } else {
                    break;
                }
            }
            return vnext;
        }
        if dnext == dprev {
            vnext = (vnext-vprev)* 2 + vprev;
            continue;
        }

        let dy = vnext-vprev;

        //println!("{}:{}, {}:{} = {}", vprev, dprev, vnext, dnext, dy);

        if dnext.signum() != dprev.signum() {
            // we overshot; go in between?
            if dy.abs() > 1 {
                vnext = vprev + dy/2;
            }
        } else {
            if dnext.abs() < dprev.abs() {
                // right direction, keep going
                vprev = vnext;
                dprev = dnext;
                vnext = vnext + 2*dy;
            } else {
                // wrong way go back?
                vnext = vprev - dy;
            }
        }
    }
}

#[test]
fn part2_sample_works() {
    let input = parse_input(include_str!("sample.txt"));
    let result = newton(&input);

    assert_eq!(result, 301);
}

pub fn part2() -> i64 {
    let input = parse_input(include_str!("input.txt"));
    newton(&input)
}

#[test]
fn part2_works() {
    assert_eq!(part2(), 3423279932937)
}
