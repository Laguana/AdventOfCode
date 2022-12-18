use std::{str::FromStr, collections::HashSet};

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

#[derive(Debug)]
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
    for _rock in 1..=steps {

        //println!("Rock {}", _rock);
        //field.render(tallest);
        while !next_rock.step(*jet_iter.next().unwrap(), &field) {
            //println!(" {:?}", next_rock.tiles());
        }
        // Rock has landed; add it to the field
        let new_tiles = next_rock.tiles();
        tallest = new_tiles.iter().map(|c| c.y).max().unwrap().max(tallest);
        //println!(" Landed {:?}", new_tiles);

        field.grid.extend(new_tiles);

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
    simulate(&input, 20);
    
}

pub fn part2() -> u32 {
    let _input = parse_input(include_str!("input.txt"));
    0
}

#[test]
fn part2_works() {
    assert_eq!(part2(), 0)
}
