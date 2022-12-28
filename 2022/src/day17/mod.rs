use std::{str::FromStr, collections::HashMap};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Input {
    jets: Vec<Direction>
}

#[derive(Debug)]
struct InputParseError;

impl FromStr for Input {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Input{ jets: s.trim().chars().map(|c| if c == '<' {Direction::Left} else {Direction::Right}).collect()})
    }
}

fn parse_input(s: &str) -> Input {
    s.parse().expect("Unable to parse input")
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum RockType {
    Flat,
    Plus,
    Corner,
    Tall,
    Square
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate {
    x: i64,
    y: i64
}

impl Coordinate {
    pub fn new(x: i64, y: i64) -> Coordinate {
        Coordinate{x, y}
    }

    pub fn left(&self) -> Coordinate {
        Coordinate{x: self.x-1, y: self.y}
    }

    pub fn right(&self) -> Coordinate {
        Coordinate{x: self.x+1, y: self.y}
    }

    pub fn down(&self) -> Coordinate {
        Coordinate{x: self.x, y: self.y-1}
    }
}

impl RockType {
    pub fn next(r: &RockType) -> RockType {
        match r {
            RockType::Flat => RockType::Plus,
            RockType::Plus => RockType::Corner,
            RockType::Corner => RockType::Tall,
            RockType::Tall => RockType::Square,
            RockType::Square => RockType::Flat,
        }
    }

    pub fn tiles(r: &RockType, c: &Coordinate) -> Vec<Coordinate> {
        match r {
            RockType::Flat => vec![Coordinate::new(c.x, c.y), Coordinate::new(c.x+1, c.y), Coordinate::new(c.x+2, c.y), Coordinate::new(c.x+3, c.y)],
            RockType::Plus => vec![Coordinate::new(c.x+1, c.y), Coordinate::new(c.x, c.y+1), Coordinate::new(c.x+1, c.y+1), Coordinate::new(c.x+2, c.y+1), Coordinate::new(c.x+1, c.y+2)],
            RockType::Corner => vec![Coordinate::new(c.x, c.y), Coordinate::new(c.x+1, c.y), Coordinate::new(c.x+2, c.y), Coordinate::new(c.x+2, c.y+1), Coordinate::new(c.x+2, c.y+2)],
            RockType::Tall => vec![Coordinate::new(c.x, c.y), Coordinate::new(c.x, c.y+1), Coordinate::new(c.x, c.y+2), Coordinate::new(c.x, c.y+3)],
            RockType::Square => vec![Coordinate::new(c.x, c.y), Coordinate::new(c.x+1, c.y), Coordinate::new(c.x, c.y+1), Coordinate::new(c.x+1, c.y+1)],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Rock {
    pos: Coordinate,
    kind: RockType,
}

struct Field {
    grid: Vec<u8>,
    maxy: i64
}

impl Field {
    pub fn new() -> Field {
        Field {grid: vec![], maxy: -1}
    }

    pub fn _render(&self) {
        for r in self.grid.iter().rev() {
            println!("|{}|",String::from_iter((0..7).map(|x| if r & (1<<x) != 0 {'#'} else {'.'})));
        }
        println!("---------");
    }

    pub fn set(&mut self, c: &Coordinate) {
        while self.maxy < c.y {
            self.grid.push(0);
            self.maxy += 1;
        }
        assert!(c.y >= 0);
        self.grid[c.y as usize] |= 1<<c.x;
    }

    pub fn get(&self, c: &Coordinate) -> bool {
        if c.y > self.maxy {
            return false;
        }
        return (self.grid[c.y as usize] & (1<<c.x)) != 0
    }

    pub fn top(&self) -> u128 {
        self.grid.iter().rev().take(17).fold(0, |acc, r| (acc << 7) | (*r as u128))
    }
}

impl Rock {
    pub fn initial() -> Rock {
        Rock{pos: Coordinate::new(2, 3), kind: RockType::Flat}
    }

    pub fn next(prev: &Rock, tallest: i64) -> Rock {
        Rock{pos: Coordinate::new(2, tallest+4), kind: RockType::next(&prev.kind)}
    }

    pub fn tiles(&self) -> Vec<Coordinate> {
        RockType::tiles(&self.kind, &self.pos)
    }

    pub fn step(&mut self, dir: Direction, field: &Field) -> bool {
        
        let left_right_candidate_coord = match dir {
            Direction::Left => self.pos.left(),
            Direction::Right => self.pos.right(),
        };
        let left_right_tiles = RockType::tiles(&self.kind, &left_right_candidate_coord);
        let left_right_coord = if left_right_tiles.into_iter().any(|p| {
            p.x >= 7  || p.x < 0 || field.get(&p)
            }) {
                self.pos
            } else {
                left_right_candidate_coord
            };
        let down_coord = left_right_coord.down();
        let down_tiles = RockType::tiles(&self.kind, &down_coord);
        if down_tiles.into_iter().any(|p| p.y < 0 || field.get(&p) ) {
            // tile is now set
            self.pos = left_right_coord;
            return true;
        } else {
            self.pos = down_coord;
            return false;
        }
    }
}

fn simulate(input: &Input, steps: usize) -> i64 {
    let mut jet_iter = input.jets.iter().cycle();
    let mut field = Field::new();
    let mut next_rock = Rock::initial();

    let mut jet_idx = 0;
    let jet_len = input.jets.len();

    let cycle_find = true;

    let mut history = vec![];
    let mut history_idx: HashMap<(u128, usize, RockType), usize> = HashMap::new();
    
    //let mut _fall_duration = 0;
    //let mut _check_duration = 0;
    //let mut _key_duration = 0;
    //let mut _set_duration = 0;
    //let mut _iter_duration = 0;

    for rock in 1..=steps {
        //let iter_start = Instant::now();
        //println!("Rock {}", rock);
        //field.render(tallest);
        //let now = Instant::now();
        while !next_rock.step(*jet_iter.next().unwrap(), &field) {
            //println!(" {:?}", next_rock.tiles());
            jet_idx += 1;
        }
        jet_idx %= jet_len;
        //_fall_duration += now.elapsed().as_nanos();

        //let now = Instant::now();
        // Rock has landed; add it to the field
        next_rock.tiles().into_iter().for_each(|p| field.set(&p));
        //_set_duration += now.elapsed().as_nanos();

        let tallest = field.maxy;

        //let now = Instant::now();
        let top_bits = field.top();
        //_key_duration += now.elapsed().as_nanos();

        if cycle_find {
            
            let history_key = (top_bits, jet_idx, next_rock.kind);
            //let now = Instant::now();
            match history_idx.insert(history_key, history.len()) {
                Some(prev_idx) => {
                    let (cycle_start, height_start) = history[prev_idx];
                    //println!("Found cycle! {} {} -> {} {}", cycle_start, height_start, rock, tallest);
                    //println!("fall time {}, key time {}, set time {},  check time {}, iter time {}", _fall_duration, _key_duration, _set_duration, _check_duration, _iter_duration);
                    //field._render(tallest);
                    let cycle_length = rock-cycle_start;
                    let height_delta = tallest - height_start;
                    let cycle_repeat = (steps-rock)/cycle_length;
                    let remainder = (steps-rock)%cycle_length;
                    let before_remainder = tallest + (cycle_repeat as i64) * height_delta;
                    let (_, height_remainder) = history[prev_idx + remainder];
                    let height_remainder = height_remainder - height_start;
                    //println!("{} {}, {} {}, {} {}", cycle_length, height_delta, cycle_repeat, remainder, before_remainder, height_remainder);
                    return before_remainder + height_remainder+1;
                },
                None => {
                    history.push((rock, tallest));
                },
            }
            //_check_duration += now.elapsed().as_nanos();
        }

        next_rock = Rock::next(&next_rock, tallest);
        //_iter_duration += iter_start.elapsed().as_nanos();
    }

    field.maxy + 1
}

#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("sample.txt"));
    let result = simulate(&input, 2022);
    assert_eq!(result, 3068);
}

pub fn part1() -> i64 {
    let input = parse_input(include_str!("input.txt"));
    simulate(&input, 2022)
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 3059)
}

#[test]
fn part2_sample_works() {
    let input = parse_input(include_str!("sample.txt"));
    // We need something O(f(input)) rather than O(1_000_000_000_000)...
    let result = simulate(&input, 1_000_000_000_000);
    assert_eq!(result, 1514285714288);
    
}

pub fn part2() -> i64 {
    let input = parse_input(include_str!("input.txt"));

    //let now = Instant::now();
    let result = simulate(&input, 1_000_000_000_000);
    //println!("part2 took {}us", now.elapsed().as_nanos());
    result
}

#[test]
fn part2_works() {
    assert_eq!(part2(), 1500874635587);
}
