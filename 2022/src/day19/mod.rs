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
    time: u64,
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
    pub fn new(time: u64) -> State {
        State { time, ore: 0, ore_robots: 1, clay: 0, clay_robots: 0, obsidian: 0, obsidian_robots: 0, geodes: 0, geode_robots: 0}
    }

    pub fn tick(&mut self) {
        self.ore      += self.ore_robots;
        self.clay     += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geodes   += self.geode_robots;
        self.time -= 1;
    }

    pub fn afford_geode(&self, blueprint: &Blueprint) -> bool {
        self.ore >= blueprint.ore_per_geode && self.obsidian >= blueprint.obsidian_per_geode
    }

    pub fn afford_obsidian(&self, blueprint: &Blueprint) -> bool {
        self.ore >= blueprint.ore_per_obsidian && self.clay >= blueprint.clay_per_obsidian
    }
    pub fn enough_obsidian(&self, blueprint: &Blueprint) -> bool {
        if self.obsidian_robots >= blueprint.obsidian_per_geode {
            return true;
        }
        let defecit = blueprint.obsidian_per_geode - self.obsidian_robots;
        if defecit * self.time <= self.obsidian {
            return true;
        }
        return false;
    }

    pub fn afford_clay(&self, blueprint: &Blueprint) -> bool {
        self.ore >= blueprint.ore_per_clay
    }

    pub fn enough_clay(&self, blueprint: &Blueprint) -> bool {
        if self.clay_robots >= blueprint.clay_per_obsidian {
            return true;
        }
        let defecit = blueprint.clay_per_obsidian - self.clay_robots;
        if defecit * self.time <= self.clay {
            return true;
        }
        if self.enough_obsidian(blueprint) {
            return true;
        }

        return false;
    }

    pub fn afford_ore(&self, blueprint: &Blueprint) -> bool {
        self.ore >= blueprint.ore_per_ore
    }
    pub fn enough_ore(&self, blueprint: &Blueprint) -> bool {
        let max_ore_needed = 
            blueprint.ore_per_clay
            .max(blueprint.ore_per_geode)
            .max(blueprint.ore_per_obsidian);
        if self.ore_robots >= max_ore_needed {
            return true;
        }
        let defecit = max_ore_needed - self.ore_robots;
        if defecit * self.time <= self.ore {
            return true;
        }
        return false;
    }


}

#[derive(Debug)]
enum BotType {
    Ore,
    Clay,
    Obsidian,
    Geode
}


fn evaluate_blueprint_rec(blueprint: &Blueprint, state: State) -> State {
    if state.time == 0 {
        return state;
    }
    let mut wait_it_out = state.clone();
    wait_it_out.geodes += state.time * state.geode_robots;

    [BotType::Ore, BotType::Clay, BotType::Obsidian, BotType::Geode].into_iter().filter_map(|bt| {
        match bt {
            BotType::Ore => {
                if state.enough_ore(blueprint) {
                    return None
                }
                let mut candidate_state = state.clone();
                while candidate_state.time > 0 && !candidate_state.afford_ore(blueprint) {
                    candidate_state.tick()
                }
                if candidate_state.time == 0 || !candidate_state.afford_ore(blueprint) {
                    return None;
                }
                candidate_state.ore -= blueprint.ore_per_ore;
                candidate_state.tick();
                candidate_state.ore_robots += 1;
                Some(evaluate_blueprint_rec(blueprint, candidate_state))
            },
            BotType::Clay => {
                if state.enough_clay(blueprint) {
                    return None
                }
                let mut candidate_state = state.clone();
                while candidate_state.time > 0 && !candidate_state.afford_clay(blueprint) {
                    candidate_state.tick()
                }
                if candidate_state.time == 0 || !candidate_state.afford_clay(blueprint) {
                    return None;
                }
                candidate_state.ore -= blueprint.ore_per_clay;
                candidate_state.tick();
                candidate_state.clay_robots += 1;
                Some(evaluate_blueprint_rec(blueprint, candidate_state))
            },
            BotType::Obsidian => {
                if state.enough_obsidian(blueprint) {
                    return None
                }
                let mut candidate_state = state.clone();
                while candidate_state.time > 0 && !candidate_state.afford_obsidian(blueprint) {
                    candidate_state.tick()
                }
                if candidate_state.time == 0 || !candidate_state.afford_obsidian(blueprint) {
                    return None;
                }
                candidate_state.ore -= blueprint.ore_per_obsidian;
                candidate_state.clay -= blueprint.clay_per_obsidian;
                candidate_state.tick();
                candidate_state.obsidian_robots += 1;
                Some(evaluate_blueprint_rec(blueprint, candidate_state))
            },
            BotType::Geode => {
                let mut candidate_state = state.clone();
                while candidate_state.time > 0 && !candidate_state.afford_geode(blueprint) {
                    candidate_state.tick()
                }
                if candidate_state.time == 0 || !candidate_state.afford_geode(blueprint) {
                    return None;
                }
                candidate_state.ore -= blueprint.ore_per_geode;
                candidate_state.obsidian -= blueprint.obsidian_per_geode;
                candidate_state.tick();
                candidate_state.geode_robots += 1;
                Some(evaluate_blueprint_rec(blueprint, candidate_state))
            },
        }
    }).max().unwrap_or(wait_it_out).max(wait_it_out)

}

fn evaluate_blueprint(blueprint: &Blueprint, time: u64) -> u64 {
    let state = State::new(time);
    //println!("{:?}", blueprint);
    let final_state = evaluate_blueprint_rec(blueprint, state);
    //println!("{:?}", final_state);
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
    let input = parse_input(include_str!("sample.txt"));
    let results: Vec<_> = input.blueprints.iter().map(|b| evaluate_blueprint(b, 32)).collect();
    assert_eq!(results[0], 56);
    assert_eq!(results[1], 62);
}

pub fn part2() -> u64 {
    let input = parse_input(include_str!("input.txt"));
    input.blueprints[..3].iter().map(|b| evaluate_blueprint(b, 32)).product()
}

#[test]
fn part2_works() {
    assert_eq!(part2(), 46816)
}
