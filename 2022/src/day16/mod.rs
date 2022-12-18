use std::{str::FromStr, collections::HashMap};

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

#[derive(Debug, Clone)]
struct LocationI {
    rate: usize,
    connections: Vec<(usize, usize)>
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

fn floyd_warshal(graph: &HashMap<usize, LocationI>) -> HashMap<(usize, usize), usize> {
    let mut result = HashMap::new();
    for (k,v) in graph.iter() {
        for (d, t) in &v.connections {
            result.insert((*k, *t), *d);
        }
    }
    for &k in graph.keys() {
        for &i in graph.keys() {
            for &j in graph.keys() {
                if i == j {
                    continue;
                }
                let new_d = {
                    let ik = result.get(&(i,k));
                    let kj = result.get(&(k,j));
                    let ij = result.get(&(i,j));
                    match (ik, kj) {
                        (None, _) => None,
                        (_, None) => None,
                        (Some(d1), Some(d2)) => {
                            let dc = *d1 + *d2;
                            match ij {
                                None => Some(dc),
                                Some(d) => {
                                    if dc < *d {
                                        Some(dc)
                                    } else {
                                        None
                                    }
                                }
                            }
                        }
                    }
                };
                if new_d.is_some() {
                    result.insert((i,j), new_d.unwrap());
                }
            }
        }
    }

    result
}

fn maximise_flow(graph: &HashMap::<usize, LocationI>, start: usize, part2: bool) -> usize {
    let shortest_paths = floyd_warshal(graph);
    let mut max_open = 0u64;
    for (k,v) in graph.iter() {
        if v.rate != 0 {
            max_open |= 1<<k;
        }
    }
    //println!("{:?}", shortest_paths);

    // try each ordering
    if part2 {
        max_flow_rec2(graph, &shortest_paths, start, start, 0, 0, max_open, 26 ,0, 0)
    } else {
        max_flow_rec2(graph, &shortest_paths, start, start, 30, 0, max_open, 30, 0, 0)
        //max_flow_rec(graph, &shortest_paths, start, max_open, 30)
    }
}

fn _max_flow_rec(
    graph: &HashMap<usize, LocationI>,
    shortest_paths: &HashMap<(usize, usize), usize>,
    loc: usize,
    open: u64,
    time: usize) -> usize {
    if time <= 1 {
        return 0;
    }
    let loc_mask = 1<<loc;
    assert_eq!(open & loc_mask, 0);
    (0..63).into_iter()
        .filter(|&i| 
                i != loc 
                && (open & (1<<i)) != 0 
                && *shortest_paths.get(&(loc, i)).unwrap() < time)
        .map(|dest| {
            // Go from loc to dest and turn it on
            let d = shortest_paths.get(&(loc, dest)).unwrap();
            let new_open = open & !(1<<dest);
            let new_time = time - d - 1;
            let action_score = graph.get(&dest).unwrap().rate * new_time;
            action_score + _max_flow_rec(graph, shortest_paths, dest, new_open, new_time)
        }).max().unwrap_or_default()
}

fn upper_bound(
    graph: &HashMap<usize, LocationI>,
    open: u64,
    time: usize) -> usize {
    // Suppose we magically opened all the things right now
    (0..63).into_iter()
        .filter(|&i| (open & (1<<i)) != 0)
        .map(|i| {
            graph.get(&i).unwrap().rate * (time-1)
        }).sum()
}

fn max_flow_rec2(
    graph: &HashMap<usize, LocationI>,
    shortest_paths: &HashMap<(usize, usize), usize>,
    loc: usize,
    loc2: usize,
    delay: usize,
    delay2: usize,
    open: u64,
    time: usize,
    best_so_far: usize,
    score: usize ) -> usize {
    if time <= 1 {
        return score;
    }
    if score + upper_bound(graph, open, time) < best_so_far {
        //println!("Can't beat {}: {},{},{}", best_so_far, score, open, time);
        return best_so_far;
    }
    match (delay, delay2) {
        (0, t) => {
            let mut best_so_far = best_so_far.max(score);
            for dest in 0..63 {
                if dest == loc 
                    || (open & (1<<dest)) == 0 
                    || *shortest_paths.get(&(loc, dest)).unwrap() >= time {
                    continue;
                }

                // Go from loc to dest and turn it on
                let d = shortest_paths.get(&(loc, dest)).unwrap();
                let new_open = open & !(1<<dest);
                let new_time = time - d - 1;
                let action_score = graph.get(&dest).unwrap().rate * new_time;
                if t == 0 {
                    for dest2 in 0..63 {
                        if dest2 == loc2 || dest2 == dest || dest2 == loc
                            || (new_open & (1<<dest2)) == 0
                            || *shortest_paths.get(&(loc2, dest2)).unwrap() >= time {
                            continue;
                        }
                        // Go from loc2 to dest2 and turn it on
                        let d2 = shortest_paths.get(&(loc2, dest2)).unwrap();
                        let new_open = new_open & !(1<<dest2);
                        let new_time = time - d2 - 1;
                        let action_score2 = graph.get(&dest2).unwrap().rate * new_time;
                        let step = d.min(d2)+1;
                        let recursive_result = max_flow_rec2(graph, shortest_paths, dest, dest2, d+1-step, d2+1-step, new_open, time - step, best_so_far, action_score + action_score2 + score);
                        best_so_far = best_so_far.max(recursive_result);
                    }
                } else {
                    let step = t.min(d+1);
                    let recursive_result = max_flow_rec2(graph, shortest_paths, dest, loc2, d+1-step, delay2-step, new_open, time-step, best_so_far, action_score + score);
                    best_so_far = best_so_far.max(recursive_result);
                }
            };
            best_so_far
        },
        (t, 0) => {
            let mut best_so_far = best_so_far.max(score);
            for dest in 0..63 {
                if dest == loc2 
                    || (open & (1<<dest)) == 0 
                    || *shortest_paths.get(&(loc2, dest)).unwrap() >= time {
                    continue;
                }

                // Go from loc to dest and turn it on
                let d = shortest_paths.get(&(loc2, dest)).unwrap();
                let new_open = open & !(1<<dest);
                let new_time = time - d - 1;
                let action_score = graph.get(&dest).unwrap().rate * new_time;
                let step = t.min(d+1);
                let recursive_result = max_flow_rec2(graph, shortest_paths, loc, dest, t-step, d+1-step, new_open, time - step, best_so_far, action_score + score);
                best_so_far = best_so_far.max(recursive_result);
            }
            best_so_far
        },
        _ => panic!("Nobody is ready!")
    }
}

#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("sample.txt"));
    let (start, graph) = construct_graph(&input);
    let result = maximise_flow(&graph, start, false);
    assert_eq!(result, 1651)
}

pub fn part1() -> usize {
    let input = parse_input(include_str!("input.txt"));
    let (start, graph) = construct_graph(&input);
    maximise_flow(&graph, start, false)
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 1474)
}

#[test]
fn part2_sample_works() {
    let input = parse_input(include_str!("sample.txt"));
    let (start, graph) = construct_graph(&input);
    let result = maximise_flow(&graph, start, true);
    assert_eq!(result, 1707);
}

pub fn part2() -> usize {
    let input = parse_input(include_str!("input.txt"));
    let (start, graph) = construct_graph(&input);
    maximise_flow(&graph, start, true)
}

#[test]
fn part2_works() {
    assert_eq!(part2(), 2100)
}
