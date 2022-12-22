use std::{str::FromStr, collections::{HashSet, HashMap}};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Facing {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Forward(u64),
    Left,
    Right,
}

impl Facing {
    pub fn score(&self) -> usize {
        match self {
            Facing::Up => 3,
            Facing::Down => 1,
            Facing::Left => 2,
            Facing::Right => 0,
        }
    }

    pub fn turn(&self, inst: &Instruction) -> Facing {
        match inst {
            Instruction::Forward(_) => self.clone(),
            Instruction::Left => match self {
                Facing::Up => Facing::Left,
                Facing::Down => Facing::Right,
                Facing::Left => Facing::Down,
                Facing::Right => Facing::Up,
            },
            Instruction::Right => match self {
                Facing::Up => Facing::Right,
                Facing::Down => Facing::Left,
                Facing::Left => Facing::Up,
                Facing::Right => Facing::Down,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    pub fn step(&self, facing: &Facing, input: &Input, part2: bool) -> Coordinate {
        match facing {
            Facing::Up => {
                let bound = input.col_bounds.get(&self.x).unwrap();
                if bound.low == self.y {
                    Coordinate{x: self.x, y: bound.high}
                } else {
                    Coordinate{x: self.x, y: self.y-1}
                }
            },
            Facing::Down => {
                let bound = input.col_bounds.get(&self.x).unwrap();
                if bound.high == self.y {
                    Coordinate{x: self.x, y: bound.low}
                } else {
                    Coordinate{x: self.x, y: self.y+1}
                }
            },
            Facing::Left => {
                let bound = input.row_bounds[self.y];
                if bound.low == self.x {
                    Coordinate{x: bound.high, y: self.y}
                } else {
                    Coordinate{x: self.x-1, y: self.y}
                }
            },
            Facing::Right => {
                let bound = input.row_bounds[self.y];
                if bound.high == self.x {
                    Coordinate{x: bound.low, y: self.y}
                } else {
                    Coordinate{x: self.x+1, y: self.y}
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Bound {
    low: usize,
    high: usize,
}

#[derive(Debug)]
struct Input {
    row_bounds: Vec<Bound>,
    col_bounds: HashMap<usize, Bound>,
    walls: HashSet<Coordinate>,
    instructions: Vec<Instruction>
}

#[derive(Debug)]
struct InputParseError;

impl FromStr for Input {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let mut col_start = HashMap::new();
        let mut col_stop = HashMap::new();
        let mut walls = HashSet::new();
        let mut max_x = 0;
        let grid_lines = lines.by_ref().take_while(|e| !e.trim().is_empty());
        let row_bounds: Vec<_> = grid_lines.enumerate().map(|(y, l)| {
            max_x = max_x.max(l.len());
            let bound = l.chars().enumerate()
                .skip_while(|(_,c)| *c == ' ')
                .take_while(|(_,c)| *c != ' ')
                .fold(Bound{low:l.len(), high: 0}, |bound, (x, c)| {
                    if c == '#' {
                        walls.insert(Coordinate{x, y});
                    }
                    if !col_start.contains_key(&x) {
                        col_start.insert(x, y);
                    }
                    Bound{ low: bound.low.min(x), high:bound.high.max(x) }
                });
            for x in 0..bound.low {
                if col_start.contains_key(&x) && !col_stop.contains_key(&x) {
                    col_stop.insert(x, y-1);
                }
            }
            for x in bound.high+1..max_x {
                if col_start.contains_key(&x) && !col_stop.contains_key(&x) {
                    col_stop.insert(x, y-1);
                }
            }
            bound
        }).collect();

        let mut col_bounds = HashMap::new();
        for (k,low) in col_start {
            match col_stop.get(&k) {
                Some(&high) => {
                    col_bounds.insert(k, Bound{low, high});
                },
                None => {
                    let high = row_bounds.len()-1;
                    col_bounds.insert(k, Bound{low, high});
                },
            }
            
        }

        let instruction_line = lines.next().unwrap();
        
        let (mut instructions, final_step) = instruction_line.chars().fold((vec![], 0), |(accv, acci), c| {
            let mut v = accv;
            match c {
                'L' => {
                    if acci == 0 {
                        v.push(Instruction::Left);
                        (v,0)
                    } else {
                        v.push(Instruction::Forward(acci));
                        v.push(Instruction::Left);
                        (v,0)
                    }
                },
                'R' => {
                    if acci == 0 {
                        v.push(Instruction::Right);
                        (v,0)
                    } else {
                        v.push(Instruction::Forward(acci));
                        v.push(Instruction::Right);
                        (v,0)
                    }
                },
                '0'..='9' => {
                    let d = c as u8 - b'0';
                    (v, acci * 10 + d as u64)
                },
                _ => panic!("Bad input")
            }
        });

        if final_step != 0 {
            instructions.push(Instruction::Forward(final_step));
        }

        Ok(Input {
            row_bounds,
            col_bounds,
            walls,
            instructions,
        })
    }
}

fn parse_input(s: &str) -> Input {
    s.parse().expect("Unable to parse input")
}

#[derive(Debug)]
struct Position {
    facing: Facing,
    position: Coordinate
}

impl Position {
    pub fn new(input: &Input) -> Position {
        let position = Coordinate {x:input.row_bounds[0].low , y:0};
        Position { facing: Facing::Right, position }
    }

    pub fn score(&self) -> usize {
        1000 * (self.position.y+1) + 4 * (self.position.x+1) + self.facing.score()
    }

    pub fn step(&mut self, instruction: &Instruction, input: &Input, part2: bool) {
        match instruction {
            Instruction::Forward(v) => {
                for _ in 0..*v {
                    let next = self.position.step(&self.facing, input, part2);
                    if input.walls.contains(&next) {
                        //println!("Wall @ {:?}", next);
                        return;
                    } else {
                        self.position = next;
                    }
                }
            },
            Instruction::Left => {
                self.facing = self.facing.turn(instruction)
            },
            Instruction::Right => {
                self.facing = self.facing.turn(instruction)
            },
        }
    }
}

#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("sample.txt"));
    let mut position = Position::new(&input);
    for i in &input.instructions {
        //println!("{:?}, {:?}", i, position);
        position.step(&i, &input);
    }
    let result = position.score();
    assert_eq!(result, 6032)
    
}

pub fn part1() -> usize {
    let input = parse_input(include_str!("input.txt"));
    let mut position = Position::new(&input);
    for i in &input.instructions {
        //println!("{:?}, {:?}", i, position);
        position.step(&i, &input);
    }
    position.score()
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 186128)
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
