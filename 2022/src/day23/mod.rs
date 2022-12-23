use std::{str::FromStr, collections::{HashSet, HashMap}};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Coordinate {
    x: i64,
    y: i64,
}

impl Coordinate {
    pub fn any_adjacent(&self) -> Vec<Coordinate> {
        let mut result = self.north_adjacent();
        result.append(&mut self.south_adjacent());
        result.push(self.west());
        result.push(self.east());
        result
    }
    pub fn north(&self) -> Coordinate {
        Coordinate{x: self.x, y: self.y-1}
    }
    pub fn north_adjacent(&self) -> Vec<Coordinate> {
        let n = self.north();
        let ne = n.east();
        vec![n.west(), n, ne]
    }

    pub fn south(&self) -> Coordinate {
        Coordinate{x: self.x, y: self.y+1}
    }
    pub fn south_adjacent(&self) -> Vec<Coordinate> {
        let s = self.south();
        let se = s.east();
        vec![s.west(), s, se]
    }

    pub fn west(&self) -> Coordinate {
        Coordinate{x: self.x-1, y: self.y}
    }
    pub fn west_adjacent(&self) -> Vec<Coordinate> {
        let w = self.west();
        let sw = w.south();
        vec![w.north(), w, sw]
    }

    pub fn east(&self) -> Coordinate {
        Coordinate{x: self.x+1, y: self.y}
    }
    pub fn east_adjacent(&self) -> Vec<Coordinate> {
        let e = self.east();
        let se = e.south();
        vec![e.north(), e, se]
    }
}

#[derive(Debug)]
struct Input {
    elves: HashSet<Coordinate>
}

#[derive(Debug)]
struct InputParseError;

impl FromStr for Input {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Input{elves: s.trim().lines().enumerate().flat_map(|(y,l)| {
            l.chars().enumerate().filter_map(move |(x, c)| {
                if c == '.' {
                    None
                } else {
                    let x = x as i64;
                    let y = y as i64;
                    Some(Coordinate{x, y})
                }
            })
        }).collect()})
    }
}

fn parse_input(s: &str) -> Input {
    s.parse().expect("Unable to parse input")
}

fn evolve(before: &HashSet<Coordinate>, iteration: usize) -> (HashSet<Coordinate>, usize) {
    let mut intermediate = HashMap::new();
    let mut collisions = HashSet::new();
    let mut result = HashSet::new();
    for k in before {
        let mut found = false;
        if k.any_adjacent().into_iter().all(|p| !before.contains(&p)) {
            result.insert(*k);
            continue;
        }
        for dir in iteration..iteration+4 {
            match dir%4 {
                0 => {
                    if k.north_adjacent().into_iter().all(|p| !before.contains(&p)) {
                        found = true;
                        //println!("Proposing north {:?}", k);
                        let p = k.north();
                        if collisions.contains(&p) { 
                            result.insert(*k);
                            break;
                        }
                        match intermediate.insert(p, k) {
                            None => (),
                            Some(other) => {
                                collisions.insert(p);
                                intermediate.remove(&p);
                                result.insert(*k);
                                result.insert(*other);
                            }
                        }
                        break;
                    }
                },
                1 => {
                    if k.south_adjacent().into_iter().all(|p| !before.contains(&p)) {
                        //println!("Proposing south {:?}", k);
                        found = true;
                        let p = k.south();
                        if collisions.contains(&p) { 
                            result.insert(*k);
                            break;
                        }
                        match intermediate.insert(p, k) {
                            None => (),
                            Some(other) => {
                                collisions.insert(p);
                                intermediate.remove(&p);
                                result.insert(*k);
                                result.insert(*other);
                            }
                        }
                        break;
                    }
                },
                2 => {
                    if k.west_adjacent().into_iter().all(|p| !before.contains(&p)) {
                        //println!("Proposing west {:?}", k);
                        found = true;
                        let p = k.west();
                        if collisions.contains(&p) { 
                            result.insert(*k);
                            break;
                        }
                        match intermediate.insert(p, k) {
                            None => (),
                            Some(other) => {
                                collisions.insert(p);
                                intermediate.remove(&p);
                                result.insert(*k);
                                result.insert(*other);
                            }
                        }
                        break;
                    }
                },
                3 => {
                    if k.east_adjacent().into_iter().all(|p| !before.contains(&p)) {
                        //println!("Proposing east {:?}", k);
                        found = true;
                        let p = k.east();
                        if collisions.contains(&p) { 
                            result.insert(*k);
                            break;
                        }
                        match intermediate.insert(p, k) {
                            None => (),
                            Some(other) => {
                                collisions.insert(p);
                                intermediate.remove(&p);
                                result.insert(*k);
                                result.insert(*other);
                            }
                        }
                        break;
                    }
                },
                _ => panic!("Invalid input")
            }
        }
        if !found {
            result.insert(*k);
        }
    };
    let moved = intermediate.len();
    result.extend(intermediate.into_keys());
    (result, moved)
}

fn bounds(field: &HashSet<Coordinate>) -> (i64, i64, i64, i64) {
    field.iter()
    .fold((i64::MAX, i64::MIN, i64::MAX, i64::MIN), |(xmin, xmax, ymin, ymax), c| {
    (xmin.min(c.x), xmax.max(c.x), ymin.min(c.y), ymax.max(c.y))})
}

fn _print_field(field: &HashSet<Coordinate>) {
    let (xmin, xmax, ymin, ymax) = bounds(field);
    println!("{},{}", xmin, ymin);
    for y in ymin..=ymax {
        println!("{}", String::from_iter((xmin..=xmax).into_iter().map(|x| {
            if field.contains(&Coordinate{x, y}) {
                '#'
            } else {
                '.'
            }
        })));
    }
    println!("----");
}

fn score(field: &HashSet<Coordinate>) -> i64 {
    let (xmin, xmax, ymin, ymax) = bounds(field);
    let width = xmax-xmin+1;
    let height = ymax-ymin+1;
    height * width - field.len() as i64
}

#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("sample.txt"));
    let mut state = input.elves;
    for i in 0..10 {
        //_print_field(&state);
        (state, _) = evolve(&state, i);
    }
    //_print_field(&state);
    let result = score(&state);
    assert_eq!(result, 110)
}

pub fn part1() -> i64 {
    let input = parse_input(include_str!("input.txt"));
    let mut state = input.elves;
    for i in 0..10 {
        //_print_field(&state);
        (state, _) = evolve(&state, i);
    }
    //_print_field(&state);
    score(&state)
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 3874)
}

#[test]
fn part2_sample_works() {
    let input = parse_input(include_str!("sample.txt"));
    let mut state = input.elves;
    let mut i = 0;
    loop {
        //_print_field(&state);
        let (new_state, moved) = evolve(&state, i);
        state = new_state;
        i+= 1;
        if moved == 0 {
            break;
        }
    }
    //_print_field(&state);
    assert_eq!(i, 20)
    
}

pub fn part2() -> usize {
    let input = parse_input(include_str!("input.txt"));
    let mut state = input.elves;
    let mut i = 0;
    loop {
        //_print_field(&state);
        let (new_state, moved) = evolve(&state, i);
        state = new_state;
        i+= 1;
        if moved == 0 {
            return i;
        }
    }
}

#[test]
fn part2_works() {
    assert_eq!(part2(), 948)
}
