use std::{str::FromStr, collections::{HashMap, HashSet}};

#[derive(Debug, Clone)]
struct Location {
    name: String,
    rate: usize,
    connections: Vec<String>
}

#[derive(Debug)]
struct Input {
    locations: Vec<Location>
}

#[derive(Debug)]
struct InputParseError;

impl FromStr for Input {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Input {locations: s.trim().lines()
            .map(|l| {
                let parts: Vec<_> = l.split(" ").collect();
                let name = parts[1].to_string();
                let rate_string = parts[4];
                let rate = rate_string[5..rate_string.len()-1].parse().unwrap();
                let connections = parts[9..].iter().map(|s| s.replace(",", "").to_string()).collect();
                Location {name, rate, connections}
            }).collect()})
    }
}

fn parse_input(s: &str) -> Input {
    s.parse().expect("Unable to parse input")
}

fn consolidate_graph_arc(
    graph: &mut HashMap::<usize, LocationI>,
    from: &usize,
    old_to: &usize,
    new_to: &usize,
    distance: usize) {
    let g_e = graph.get_mut(from).unwrap();
    for (d, l) in &mut g_e.connections {
        if l == old_to {
            *d = distance;
            *l = new_to.clone();
            return;
        }
    }
}

#[derive(Debug, Clone)]
struct LocationI {
    rate: usize,
    connections: Vec<(usize, usize)>
}


fn construct_graph(input: &Input) -> (usize, HashMap::<usize, LocationI>) {
    let mut name_lookup = HashMap::<String, usize>:: new();
    {
        let mut i = 0;
        for l in &input.locations {
            name_lookup.insert(l.name.clone(), i);
            i += 1;
        }
    }
    let mut graph = HashMap::<usize, LocationI>::new();

    for l in &input.locations {
        let li = LocationI {
            rate: l.rate, 
            connections: l.connections.iter().map(|e| (1,*name_lookup.get(e).unwrap())).collect()
        };
        graph.insert(*name_lookup.get(&l.name).unwrap(), li);
    }

    for l in &input.locations {
        if l.name != "AA" && l.rate == 0 && l.connections.len() == 2 {
            // location L is just a pit stop from A to B; replace all instances with "the other end" but slightly slower
            let l_idx = name_lookup.get(&l.name).unwrap();
            let g_l = graph.remove(l_idx).unwrap();
            let (d1, l1) = &g_l.connections[0];
            let (d2, l2) = &g_l.connections[1];
            let new_d = d1 + d2;
            //println!("Consolidating {:?}; {}:{}, {}:{}", l, d1, l1, d2, l2);
            consolidate_graph_arc(&mut graph, l1, l_idx, l2, new_d);
            consolidate_graph_arc(&mut graph, l2, l_idx, l1, new_d);
        }
    }

    //println!("{:?}", graph);
    (*name_lookup.get("AA").unwrap(), graph)
}

fn try_flow_recursive(graph: &HashMap::<usize, LocationI>, max_open: u64, open: u64, time: usize, location: usize) -> usize {
    println!("{} {}, {}, {:?}", location, time, open, graph);
    let loc = graph.get(&location).unwrap();
    if time == 0 || open == max_open {
        return 0;
    }
    let open_mask = 1<<location;
    let open_option = 
        if loc.rate != 0 && open & open_mask == 0 {
            // try opening it
            let time = time-1;
            (time * loc.rate) + try_flow_recursive(graph, max_open, open | open_mask, time, location)
        } else {
            0
        };
    let wander = loc.connections.iter().filter_map(|(d, i)| {
        if *d >= time {
            return None;
        }
        Some (try_flow_recursive(graph, max_open, open, time-d, *i))
    }).max().unwrap_or_default();
    wander.max(open_option)
}

fn maximise_flow(graph: HashMap::<usize, LocationI>, start: usize) -> usize {
    let mut max_open = 0;
    for (k,v) in graph.iter() {
        if v.rate != 0 {
            max_open |= 1<<k;
        }
    }
    try_flow_recursive(&graph, max_open, 0, 30, start)
}

#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("sample.txt"));
    let (start, graph) = construct_graph(&input);
    let result = maximise_flow(graph, start);
    assert_eq!(result, 1651)
}

pub fn part1() -> u32 {
    let input = parse_input(include_str!("input.txt"));
    let graph = construct_graph(&input);
    println!("{:?}", graph);
    0
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 0)
}

#[test]
fn part2_sample_works() {
    let input = parse_input(include_str!("sample.txt"));

    
}

pub fn part2() -> u32 {
    let input = parse_input(include_str!("input.txt"));
    0
}

#[test]
fn part2_works() {
    assert_eq!(part2(), 0)
}
