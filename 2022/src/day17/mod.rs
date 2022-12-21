use std::{str::FromStr, collections::{HashSet, HashMap}};

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
    grid: HashSet<Coordinate>
}

impl Field {
    pub fn _render(&self, top: i64) {
        for y in (0..=top).rev() {
            println!("|{}|",String::from_iter((0..7).map(|x| if self.grid.contains(&Coordinate { x, y}) {'#'} else {'.'})));
        }
        println!("---------");
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
        let left_right_coord = if left_right_tiles.iter().any(|p| {
                field.grid.contains(p) || p.x >= 7  || p.x < 0
            }) {
                self.pos
            } else {
                    left_right_candidate_coord
            };
        let down_coord = left_right_coord.down();
        let down_tiles = RockType::tiles(&self.kind, &down_coord);
        if down_tiles.iter().any(|p| field.grid.contains(p) || p.y < 0) {
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
    // This is not part2 compatible.
    let mut jet_iter = input.jets.iter().cycle();
    let mut field = Field{grid: HashSet::new()};
    let mut next_rock = Rock::initial();
    let mut tallest = 0;

    let mut jet_idx = 0;
    let jet_len = input.jets.len();

    let cycle_find = true;

    let mut history = vec![];
    let mut history_idx: HashMap<(u128, usize, RockType), usize> = HashMap::new();
    


    for rock in 1..=steps {

        //println!("Rock {}", rock);
        //field.render(tallest);
        while !next_rock.step(*jet_iter.next().unwrap(), &field) {
            //println!(" {:?}", next_rock.tiles());
            jet_idx += 1;
            jet_idx %= jet_len;
        }
        // Rock has landed; add it to the field
        let new_tiles = next_rock.tiles();
        tallest = new_tiles.iter().map(|c| c.y).max().unwrap().max(tallest);
        //println!(" Landed {:?}", new_tiles);

        field.grid.extend(new_tiles);

        if cycle_find {
            let top_bits = (tallest-17..=tallest)
                .fold(0, |acc, y| acc<<7 | (0..7).fold(0, 
                    |acc, x| acc<<1 | if field.grid.contains(&Coordinate::new(x, y)) {1} else {0}));
            let history_key = (top_bits, jet_idx, next_rock.kind);
            match history_idx.insert(history_key, history.len()) {
                Some(prev_idx) => {
                    let (cycle_start, height_start) = history[prev_idx];
                    //println!("Found cycle! {} {} -> {} {}", cycle_start, height_start, rock, tallest);
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
        }

        next_rock = Rock::next(&next_rock, tallest)
    }
    tallest+1
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
    simulate(&input, 1_000_000_000_000)
}

//#[test]
fn _part2_works() {
    assert_eq!(part2(), 1500874635587)
}
