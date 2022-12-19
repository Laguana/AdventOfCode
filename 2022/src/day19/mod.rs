use std::str::FromStr;

#[derive(Debug)]
struct Blueprint {
    id: u64,
    ore_per_ore: u64,
    ore_per_clay: u64,
    ore_per_obsidian: u64,
    clay_per_obsidian: u64,
    ore_per_geode: u64,
    obsidian_per_geode: u64,
}

#[derive(Debug)]
struct Input {
    blueprints: Vec<Blueprint>
}

#[derive(Debug)]
struct InputParseError;

impl FromStr for Input {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Input{ blueprints: s.trim().lines()
            .map(|l| {
                let parts: Vec<_> = l.split(" ").collect();
                Blueprint {
                    id: parts[1][..parts[1].len()-1].parse().unwrap(),
                    ore_per_ore: parts[6].parse().unwrap(),
                    ore_per_clay: parts[12].parse().unwrap(),
                    ore_per_obsidian: parts[18].parse().unwrap(),
                    clay_per_obsidian: parts[21].parse().unwrap(),
                    ore_per_geode: parts[27].parse().unwrap(),
                    obsidian_per_geode: parts[30].parse().unwrap(),
                }
            }).collect()})
    }
}

fn parse_input(s: &str) -> Input {
    s.parse().expect("Unable to parse input")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
struct State {
    ore: u64,
    ore_robots: u64,
    clay: u64,
    clay_robots: u64,
    obsidian: u64,
    obsidian_robots: u64,
    geodes: u64,
    geode_robots: u64
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.geodes.cmp(&other.geodes) {
            std::cmp::Ordering::Equal => self.partial_cmp(other).unwrap(),
            c => c
        }
    }
}

impl State {
    pub fn new() -> State {
        State { ore: 0, ore_robots: 1, clay: 0, clay_robots: 0, obsidian: 0, obsidian_robots: 0, geodes: 0, geode_robots: 0}
    }

    pub fn tick(&mut self) {
        self.ore      += self.ore_robots;
        self.clay     += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geodes   += self.geode_robots;
    }
}

fn can_make_geode_bot(blueprint: &Blueprint, state: &State, time: u64) -> bool {
    // Some very broad heuristics; each turn we get a robot and we don't spend anything?
    // n + (n+1) + (n+2) + ... + (n+k) for time k
    // (n+k+1)(n+k)/2 - n(n+1)/2
    let obsidian_bots = state.obsidian_robots;
    let max_obsidian_output = 
        (obsidian_bots + time + 1) * (obsidian_bots + time)/2
        - obsidian_bots * (obsidian_bots+1)/2;
    let max_obsidian = state.obsidian + max_obsidian_output;
    
    let ore_bots = state.ore_robots;
    let max_ore_output =
        (ore_bots + time + 1) * (ore_bots + time)/2
        - ore_bots * (ore_bots+1)/2;
    let max_ore = state.ore + max_ore_output;

    max_obsidian >= blueprint.obsidian_per_geode && max_ore >= blueprint.ore_per_geode
}

fn evaluate_blueprint_rec(blueprint: &Blueprint, state: State, time: u64) -> State {
    if time == 0 {
        return state;
    }
    if !can_make_geode_bot(blueprint, &state, time) {
        let mut final_state = state;
        final_state.geodes += time * state.geode_robots;
        return final_state;
    }

    //println!("{}:{:?}", time, state);
    let mut max = state;
    if state.ore >= blueprint.ore_per_geode && state.obsidian >= blueprint.obsidian_per_geode {
        let mut candidate_state = state.clone();
        candidate_state.ore -= blueprint.ore_per_geode;
        candidate_state.obsidian -= blueprint.obsidian_per_geode;
        candidate_state.tick();
        candidate_state.geode_robots += 1;
        // If you can make a geode bot, probably do it.
        // Except this can bite you in the long run
        max = max.max(evaluate_blueprint_rec(blueprint, candidate_state, time-1));
    }

    if state.ore >= blueprint.ore_per_obsidian && state.clay >= blueprint.clay_per_obsidian
            && state.obsidian_robots < blueprint.obsidian_per_geode {
        let mut candidate_state = state.clone();
        candidate_state.ore -= blueprint.ore_per_obsidian;
        candidate_state.clay -= blueprint.clay_per_obsidian;
        candidate_state.tick();
        candidate_state.obsidian_robots += 1;
        max = max.max(evaluate_blueprint_rec(blueprint, candidate_state, time-1));
        // Get started on obsidian asap
        if state.obsidian_robots < blueprint.obsidian_per_geode/2 {
            return max;
        }
    }

    if state.ore >= blueprint.ore_per_clay && state.clay_robots < blueprint.clay_per_obsidian {
        // Don't make more clay bots if we have enough ish
        let mut candidate_state = state.clone();
        candidate_state.ore -= blueprint.ore_per_clay;
        candidate_state.tick();
        candidate_state.clay_robots += 1;
        max = max.max(evaluate_blueprint_rec(blueprint, candidate_state, time-1));
        /*
        // We may actually need to hold off on getting clay 
        if state.clay_robots == 0 {
            return max;
        }
        */
    }

    let max_ore_needed = 
        blueprint.ore_per_clay
        .max(blueprint.ore_per_geode)
        .max(blueprint.ore_per_obsidian);
    if state.ore >= blueprint.ore_per_ore && state.ore_robots < max_ore_needed-1 {
        let mut candidate_state = state.clone();
        candidate_state.ore -= blueprint.ore_per_ore;
        candidate_state.tick();
        candidate_state.ore_robots += 1;
        max = max.max(evaluate_blueprint_rec(blueprint, candidate_state, time-1));
    }

    // also do nothing
    let mut candidate_state = state;
    candidate_state.tick();
    max = max.max(evaluate_blueprint_rec(blueprint, candidate_state, time-1));

    max
}

fn evaluate_blueprint(blueprint: &Blueprint, time: u64) -> u64 {
    let state = State::new();
    println!("{:?}", blueprint);
    let final_state = evaluate_blueprint_rec(blueprint, state, time);
    println!("{:?}", final_state);
    final_state.geodes
}

#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("sample.txt"));
    let results: Vec<_> = input.blueprints.iter().map(|b| evaluate_blueprint(b, 24) *b.id).collect();
    assert_eq!(results[0], 9);
    assert_eq!(results[1], 24);
    let score: u64 = results.into_iter().sum();
    assert_eq!(score, 33);
}

pub fn part1() -> u64 {
    let input = parse_input(include_str!("input.txt"));
    input.blueprints.iter().map(|b|evaluate_blueprint(b, 24) * b.id).sum()
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 1613)
}

#[test]
fn part2_sample_works() {
    let _input = parse_input(include_str!("sample.txt"));
    let input = parse_input(include_str!("sample.txt"));
    let results: Vec<_> = input.blueprints.iter().map(|b| evaluate_blueprint(b, 32)).collect();
    assert_eq!(results[0], 56);
    assert_eq!(results[1], 62);
}

pub fn part2() -> u32 {
    let _input = parse_input(include_str!("input.txt"));
    0
}

#[test]
fn part2_works() {
    assert_eq!(part2(), 0)
}
