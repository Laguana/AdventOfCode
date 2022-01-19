use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("inputs/day7.sample"));
    let result = count_contains(&input, "shiny gold");
    assert_eq!(result, 4)
}

struct Input {
    rules: HashMap<String, Vec<(u32, String)>>,
}

#[derive(Debug)]
struct ParseError;

impl FromStr for Input {
    type Err = ParseError;
    fn from_str(s: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        s.trim()
            .replace("\r", "")
            .split("\n")
            .map(|l| {
                let parts: Vec<&str> = l.split("contain").collect();
                if parts.len() != 2 {
                    //println!("Wrong len {:?}", parts);
                    return Err(ParseError);
                }
                let lhs_parts: Vec<&str> = parts[0].split(" ").collect();
                let lhs_name = format!("{} {}", lhs_parts[0], lhs_parts[1]);

                if parts[1].trim() == "no other bags." {
                    return Ok((lhs_name, vec![]));
                }

                parts[1]
                    .trim()
                    .split(",")
                    .map(|e| {
                        let content: Vec<&str> = e.trim().split(" ").collect();
                        let rhs_name = format!("{} {}", content[1], content[2]);
                        //println!("Parsing {:?}", content);
                        content[0]
                            .parse()
                            .map(|num| (num, rhs_name))
                            .map_err(|_| ParseError)
                    })
                    .collect::<Result<Vec<(u32, String)>, _>>()
                    .map(|list| (lhs_name, list))
            })
            .collect::<Result<HashMap<String, Vec<(u32, String)>>, _>>()
            .map(|s| Input { rules: s })
    }
}

fn parse_input(input: &str) -> Input {
    input.parse().expect("Should be parsable")
}

fn reverse_map(input: &Input) -> HashMap<&str, Vec<&str>> {
    let mut result: HashMap<&str, Vec<&str>> = HashMap::new();

    for (key, values) in input.rules.iter() {
        for (_, v) in values {
            if result.contains_key(&v[..]) {
                let v = result
                    .get_mut(&v[..])
                    .expect("we just checked it had the key");
                v.push(key);
            } else {
                result.insert(v, vec![key]);
            }
        }
    }

    return result;
}

fn count_contains(input: &Input, target: &str) -> usize {
    let revmap = reverse_map(input);
    let mut to_consider: HashSet<&str> = HashSet::new();
    let mut considered: HashSet<&str> = HashSet::new();
    to_consider.insert(target);

    while to_consider.len() > 0 {
        to_consider = to_consider
            .drain()
            .map(|candidate| {
                if considered.contains(candidate) {
                    return None;
                }
                considered.insert(candidate);

                return revmap.get(candidate).map(|v| {
                    v.iter()
                        .filter(|e| !considered.contains(e as &str))
                        .map(|e| *e)
                        .collect()
                });
            })
            .map(|v| v.unwrap_or_else(|| vec![]))
            .flatten()
            .collect();
    }

    //println!("{:?}", considered);
    return considered.len()-1;
}

pub fn part1() -> usize {
    let input = parse_input(include_str!("inputs/day7.txt"));
    return count_contains(&input, "shiny gold");
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 197)
}

fn bags_contained(input: &Input, target: &str) -> u32 {
    input.rules[target].iter().fold(0, |acc, (c, t)| acc + c*(1+bags_contained(input, t)))
}

#[test]
fn part2_sample_works() {
    let input = parse_input(include_str!("inputs/day7.sample"));
    assert_eq!(bags_contained(&input, "shiny gold"), 32)
}

pub fn part2() -> u32 {
    let input = parse_input(include_str!("inputs/day7.txt"));
    return bags_contained(&input, "shiny gold");
}

#[test]
fn part2_works() {
    assert_eq!(part2(), 85324)
}