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
    fn wrap_part2(&self, facing: &Facing, input: &Input) -> (Coordinate, Facing) {
        /*
        Principals are nice, but this is messing with my head.

        Sample:
          1
        432
          56

        1l <-> 3u
        1r <-> 6r
        1u <-> 4u

        2r <-> 6u

        3d <-> 5l

        4d <-> 5d
        4l <-> 6d
        */

        let face_idx_x = self.x / input.face_size;
        let sub_idx_x = self.x % input.face_size;
        let face_idx_y = self.y / input.face_size;
        let sub_idx_y = self.y % input.face_size;

        //println!("Wrapping: {},{} ~ {},{}/{},{}", self.x, self.y, face_idx_x, face_idx_y, sub_idx_x, sub_idx_y);

        if input.face_size == 4 {
            // sample
            match (face_idx_x, face_idx_y) {
                (2,0) => {
                    match facing {
                        Facing::Up => {
                            //1u <-> 4u
                            let x = input.face_size - (sub_idx_x+1);
                            let y = input.face_size;
                            return (Coordinate{x, y}, Facing::Down)
                        },
                        Facing::Down => panic!("Bad direction"),
                        Facing::Left => {
                            //1l <-> 3u
                            let x = input.face_size + sub_idx_y;
                            let y = input.face_size;
                            return (Coordinate{x, y}, Facing::Down)
                        },
                        Facing::Right => {
                            //1r <-> 6r
                            let x = input.face_size * 4-1;
                            let y = input.face_size * 3 - (sub_idx_y+1);
                            return (Coordinate{x, y}, Facing::Left)
                        },
                    }
                },
                (0,1) => {
                    match facing {
                        Facing::Up => {
                            //4u <-> 1u
                            let x = input.face_size*3 - (sub_idx_x+1);
                            let y = 0;
                            return (Coordinate{x, y}, Facing::Down)
                        },
                        Facing::Down => {
                            //4d <-> 5d
                            let x = input.face_size * 3 - (sub_idx_x+1);
                            let y = input.face_size * 3-1;
                            return (Coordinate{x, y}, Facing::Up)
                        },
                        Facing::Left => {
                            //4l <-> 6d
                            let x = input.face_size * 4 - (sub_idx_y+1);
                            let y = input.face_size * 3-1;
                            return (Coordinate{x, y}, Facing::Up)
                        },
                        Facing::Right => panic!("Bad direction"),
                    }
                },
                (1,1) => {
                    match facing {
                        Facing::Up => {
                            //3u <-> 1l
                            let x = input.face_size*2;
                            let y = sub_idx_x;
                            return (Coordinate{x, y}, Facing::Right)
                        },
                        Facing::Down => {
                            //3d <-> 5l
                            let x = input.face_size * 2;
                            let y = input.face_size * 3 - (sub_idx_y+1);
                            return (Coordinate{x, y}, Facing::Right)
                        },
                        Facing::Left => panic!("Bad direction"),
                        Facing::Right => panic!("Bad direction"),
                    }
                },
                (2,1) => {
                    match facing {
                        Facing::Up =>  panic!("Bad direction"),
                        Facing::Down => panic!("Bad direction"),
                        Facing::Left => panic!("Bad direction"),
                        Facing::Right => {
                            //2r <-> 6u
                            let x = input.face_size * 4 - (sub_idx_y+1);
                            let y = input.face_size * 2;
                            return (Coordinate{x, y}, Facing::Down)
                        },
                    }
                },
                (2,2) => {
                    match facing {
                        Facing::Up =>  panic!("Bad direction"),
                        Facing::Down => {
                            //5d <-> 4d
                            let x = input.face_size - (sub_idx_x+1);
                            let y = input.face_size * 2-1;
                            //println!("result: {},{}", x, y);
                            return (Coordinate{x, y}, Facing::Up)
                        },
                        Facing::Left => {
                            //5l <-> 3d
                            let x = input.face_size * 2 - (sub_idx_y+1);
                            let y = input.face_size * 2-1;
                            return (Coordinate{x, y}, Facing::Up)
                        },
                        Facing::Right => panic!("Bad direction"),
                    }
                },
                (3,2) => {
                    match facing {
                        Facing::Up => {
                            //6u <-> 2r
                            let x = input.face_size * 3 - 1;
                            let y = input.face_size * 2-(sub_idx_x+1);
                            return (Coordinate{x, y}, Facing::Left)
                        },
                        Facing::Down => {
                            //6d <-> 4l
                            let x = 0;
                            let y = input.face_size * 2-(sub_idx_x+1);
                            return (Coordinate{x, y}, Facing::Right)
                        },
                        Facing::Left => panic!("Bad direction"),
                        Facing::Right => {
                            //6r <-> 1r
                            let x = input.face_size * 3 -1;
                            let y = input.face_size - (sub_idx_x+1);
                            return (Coordinate{x, y}, Facing::Left)
                        },
                    }
                }
                _ => panic!("Invalid input")
            }
        } else {
            // input

            /*
            Input:
             21
             3
            54
            6

            1d <-> 3r
            1r <-> 4r
            1u <-> 6...d?

            2u <-> 6l?

            5u <-> 3l
            5l <-> 2l

            6r <-> 4d
         */
            match (face_idx_x, face_idx_y) {
                (1,0) => {
                    match facing {
                        Facing::Up => {
                            //2u <-> 6l
                            let x = 0;
                            let y = input.face_size * 3 +sub_idx_x;
                            return (Coordinate{x, y}, Facing::Right)
                        },
                        Facing::Down => panic!("Bad direction"),
                        Facing::Left => {
                            //2l <-> 5l
                            let x = 0;
                            let y = input.face_size * 3-(sub_idx_y+1);
                            return (Coordinate{x, y}, Facing::Right)
                        },
                        Facing::Right => panic!("Bad direction"),
                    }
                },
                (2,0) => {
                    match facing {
                        Facing::Up => {
                            //1u <-> 6d
                            let x = sub_idx_x;
                            let y = input.face_size * 4-1;
                            return (Coordinate{x, y}, Facing::Up)
                        },
                        Facing::Down => {
                            //1d <-> 3r
                            let x = input.face_size * 2-1;
                            let y = input.face_size + sub_idx_x;
                            return (Coordinate{x, y}, Facing::Left)
                        },
                        Facing::Left => panic!("Bad direction"),
                        Facing::Right => {
                            //1r <-> 4r
                            let x = input.face_size * 2-1;
                            let y = input.face_size * 3 - (sub_idx_y+1);
                            return (Coordinate{x, y}, Facing::Left)
                        },
                    }
                },
                (1,1) => {
                    match facing {
                        Facing::Up => panic!("Bad direction"),
                        Facing::Down => panic!("Bad direction"),
                        Facing::Left => {
                            //3l <-> 5u
                            let x = sub_idx_y;
                            let y = input.face_size * 2;
                            return (Coordinate{x, y}, Facing::Down)
                        },
                        Facing::Right => {
                            //3r <-> 1d
                            let x = input.face_size * 2 + sub_idx_y;
                            let y = input.face_size -1;
                            return (Coordinate{x, y}, Facing::Up)
                        },
                    }
                },
                (0,2) => {
                    match facing {
                        Facing::Up => {
                            //5u <-> 3l
                            let x = input.face_size;
                            let y = input.face_size + sub_idx_x;
                            return (Coordinate{x, y}, Facing::Right)
                        },
                        Facing::Down => panic!("Bad direction"),
                        Facing::Left => {
                            //5l <-> 2l
                            let x = input.face_size;
                            let y = input.face_size - (sub_idx_y+1);
                            return (Coordinate{x, y}, Facing::Right)
                        },
                        Facing::Right => panic!("Bad direction"),
                    }
                },
                (1,2) => {
                    match facing {
                        Facing::Up => panic!("Bad direction"),
                        Facing::Down => {
                            //4d <-> 6r
                            let x = input.face_size-1;
                            let y = input.face_size * 3 + sub_idx_x;
                            return (Coordinate{x, y}, Facing::Left)
                        },
                        Facing::Left => panic!("Bad direction"),
                        Facing::Right => {
                            //4r <-> 1r
                            let x = input.face_size * 3-1;
                            let y = input.face_size - (sub_idx_y+1);
                            return (Coordinate{x, y}, Facing::Left)
                        },
                    }
                },
                (0,3) => {
                    match facing {
                        Facing::Up => panic!("Bad direction"),
                        Facing::Down => {
                            //6d <-> 1u
                            let x = input.face_size * 2 + sub_idx_x;
                            let y = 0;
                            return (Coordinate{x, y}, Facing::Down)
                        },
                        Facing::Left => {
                            //6l <-> 2u
                            let x = input.face_size + sub_idx_y;
                            let y = 0;
                            return (Coordinate{x, y}, Facing::Down)
                        },
                        Facing::Right => {
                            //6r <-> 4d
                            let x = input.face_size + sub_idx_y;
                            let y = input.face_size * 3-1;
                            return (Coordinate{x, y}, Facing::Up)
                        },
                    }
                },
                
                _ => panic!("Invalid input")
            }
        }
    }

    pub fn step(&self, facing: &Facing, input: &Input, part2: bool) -> (Coordinate, Facing) {
        match facing {
            Facing::Up => {
                let bound = input.col_bounds.get(&self.x).unwrap();
                if bound.low == self.y {
                    if part2 {
                        self.wrap_part2(facing, input)
                    } else {
                        (Coordinate{x: self.x, y: bound.high}, *facing)
                    }
                } else {
                    (Coordinate{x: self.x, y: self.y-1}, *facing)
                }
            },
            Facing::Down => {
                let bound = input.col_bounds.get(&self.x).unwrap();
                if bound.high == self.y {
                    if part2 {
                        self.wrap_part2(facing, input)
                    } else {
                        (Coordinate{x: self.x, y: bound.low}, *facing)
                    }
                } else {
                    (Coordinate{x: self.x, y: self.y+1}, *facing)
                }
            },
            Facing::Left => {
                let bound = input.row_bounds[self.y];
                if bound.low == self.x {
                    if part2 {
                        self.wrap_part2(facing, input)
                    } else {
                        (Coordinate{x: bound.high, y: self.y}, *facing)
                    }
                } else {
                    (Coordinate{x: self.x-1, y: self.y}, *facing)
                }
            },
            Facing::Right => {
                let bound = input.row_bounds[self.y];
                if bound.high == self.x {
                    if part2 {
                        self.wrap_part2(facing, input)
                    } else {
                        (Coordinate{x: bound.low, y: self.y}, *facing)
                    }
                } else {
                    (Coordinate{x: self.x+1, y: self.y}, *facing)
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

impl Bound {
    pub fn _contains(&self, p: usize) -> bool {
        self.low <= p && self.high >= p
    }
}

#[derive(Debug)]
struct Input {
    row_bounds: Vec<Bound>,
    col_bounds: HashMap<usize, Bound>,
    _max_x: usize,
    face_size: usize,
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
            _max_x: max_x,
            face_size: if max_x == 150 { 50 } else { 4 },
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
                    let (next, facing) = self.position.step(&self.facing, input, part2);
                    if input.walls.contains(&next) {
                        //println!("Wall @ {:?}", next);
                        return;
                    } else {
                        self.position = next;
                        self.facing = facing;
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
        position.step(&i, &input, false);
    }
    let result = position.score();
    assert_eq!(result, 6032)
    
}

pub fn part1() -> usize {
    let input = parse_input(include_str!("input.txt"));
    let mut position = Position::new(&input);
    for i in &input.instructions {
        //println!("{:?}, {:?}", i, position);
        position.step(&i, &input, false);
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
    let mut position = Position::new(&input);
    for i in &input.instructions {
        //println!("{:?}, {:?}", i, position);
        position.step(&i, &input, true);
    }
    //println!("{:?}", position);
    let result = position.score();
    assert_eq!(result, 5031);
    
}

pub fn part2() -> usize {
    let input = parse_input(include_str!("input.txt"));
    let mut position = Position::new(&input);
    for i in &input.instructions {
        //println!("{:?}, {:?}", i, position);
        position.step(&i, &input, true);
    }
    position.score()
}

#[test]
fn part2_works() {
    assert_eq!(part2(), 34426)
}
