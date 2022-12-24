use std::{str::FromStr, collections::HashSet};
use pathfinding::prelude::astar;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None
}

#[derive(Debug)]
struct Input {
    blizzards: Vec<(usize, usize, Direction)>,
    width: usize,
    height: usize,
}

#[derive(Debug)]
struct InputParseError;

impl FromStr for Input {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blizzards = vec![];
        let mut height = 0;
        let mut width = 0;
        s.trim().lines().enumerate().for_each(|(y, l)| {
            height = height.max(y+1);
            width = l.len();
            l.chars().enumerate().for_each(|(x, c)| {
                let dir = match c {
                    '^' => {
                        Direction::Up
                    },
                    'v' => {
                        Direction::Down
                    },
                    '<' => {
                        Direction::Left
                    },
                    '>' => {
                        Direction::Right
                    },
                    _ => Direction::None
                };
                if dir != Direction::None {
                    blizzards.push((x,y,dir));
                }
            })
        });

        Ok(Input{ blizzards, height, width})
    }
}

fn parse_input(s: &str) -> Input {
    s.parse().expect("Unable to parse input")
}

type BlizzardLookup = HashSet<(usize, usize)>;

fn make_blizzard_lookup(input: &Input) -> (BlizzardLookup, BlizzardLookup, BlizzardLookup, BlizzardLookup) {
    let mut up_blizzards = HashSet::new();
    let mut down_blizzards = HashSet::new();
    let mut left_blizzards = HashSet::new();
    let mut right_blizzards = HashSet::new();
    for (x,y,d) in &input.blizzards {
        match d {
            Direction::Up => {
                // To check if a blizzard is in position (x, y) at time t,
                // check to see if up_blizzards((x, (y-t)%(height-2)) is a hit
                up_blizzards.insert((x-1, y-1));
            },
            Direction::Down => {
                // check to see if down_blizzards((x, (y+t)%(height-2)) is a hit
                down_blizzards.insert((x-1, y-1));
            },
            Direction::Left => {
                left_blizzards.insert((x-1, y-1));
            },
            Direction::Right => {
                right_blizzards.insert((x-1, y-1));
            },
            _ => todo!()
        }
    }
    (up_blizzards, down_blizzards, left_blizzards, right_blizzards)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    x: usize,
    y: usize,
    t: usize,
}

impl State {
    fn distance(&self, other: &State) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    fn adjacent(&self) -> Vec<State> {
        let mut result = vec![];
        let t = self.t+1;
        if self.y > 1 || self.y == 1 && self.x == 1{
            result.push(State{x: self.x, y: self.y-1, t});
        }
        if self.x > 1 {
            result.push(State{x: self.x-1, y: self.y, t});
        }
        if self.x > 0 {
            result.push(State{x: self.x, y: self.y+1, t});
        }
        if self.y > 0 {
            result.push(State{x: self.x+1, y: self.y, t});
        }
        result.push(State{x: self.x, y: self.y, t});

        result
    }
}

fn search(input: &Input, start: &State, goal: &State) -> usize {
    let (up, down, left, right) = make_blizzard_lookup(input);
    //println!("up:{:?}\ndown:{:?}\nleft:{:?}\nright:{:?}", up, down, left, right);
    let h_mod = input.height-2;
    let w_mod = input.width-2;
    //println!("w,h = {},{}", w_mod, h_mod);
    let successors = |s0: &State| {
        let result = s0.adjacent().into_iter().filter(|s| {
            if s.y >= input.height-1 {
                if s.x == input.width-2 {
                    return true;
                }
                return false;
            }
            if s.x >= input.width-1 {
                return false;
            }
            if s.x == 0 || s.y == 0 {
                return true;
            }
            let t_mod_h = s.t%h_mod;
            let t_mod_w = s.t%w_mod;
            let x = s.x-1;
            let y = s.y-1;
            if up.contains(&(x, (y+ t_mod_h) % h_mod))
                || down.contains(&(x, (y+ h_mod - t_mod_h)% h_mod)) 
                || left.contains(&((x + t_mod_w) % w_mod, y)) 
                || right.contains(&((x + w_mod - t_mod_w) % w_mod, y)) {
                    return false
            }
            //println!("{},{}@({},{}) passed {},{}", x, y, t_mod_w, t_mod_h, (x + w_mod - t_mod_w) % w_mod, y);
            return true;
        }).map(|s| (s, 1));
        return result
    };
    let result = astar(start ,successors, |p| p.distance(&goal), |p| p.x == goal.x && p.y == goal.y);
    //println!("{:?}", result);
    result.unwrap().1
}

#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("sample.txt"));
    let result = search(&input, &State {x:1,y:0,t:0}, &State {x: input.width-2, y: input.height-1, t: 0});
    assert_eq!(result, 18)
    
}

pub fn part1() -> usize {
    let input = parse_input(include_str!("input.txt"));
    search(&input, &State {x:1,y:0,t:0}, &State {x: input.width-2, y: input.height-1, t: 0})
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 269)
}

#[test]
fn part2_sample_works() {
    let input = parse_input(include_str!("sample.txt"));
    let step1 = search(&input, &State {x:1,y:0,t:0}, &State {x: input.width-2, y: input.height-1, t: 0});
    let step2 = search(&input, &State{x: input.width-2, y: input.height-1, t: step1}, &State{ x: 1, y: 0, t: 0});
    let step3 = search(&input, &State {x:1,y:0,t:step1+step2}, &State {x: input.width-2, y: input.height-1, t: 0});
    assert_eq!(step1 + step2 + step3, 54);
}

pub fn part2() -> usize {
    let input = parse_input(include_str!("input.txt"));
    let step1 = search(&input, &State {x:1,y:0,t:0}, &State {x: input.width-2, y: input.height-1, t: 0});
    let step2 = search(&input, &State{x: input.width-2, y: input.height-1, t: step1}, &State{ x: 1, y: 0, t: 0});
    let step3 = search(&input, &State {x:1,y:0,t:step1+step2}, &State {x: input.width-2, y: input.height-1, t: 0});
    step1 + step2 + step3
}

#[test]
fn part2_works() {
    assert_eq!(part2(), 825)
}
