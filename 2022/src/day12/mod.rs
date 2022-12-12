use std::{str::FromStr, collections::{HashMap, VecDeque}};

type Coordinate = (i16, i16);

#[derive(Debug)]
struct Input {
    map: HashMap<Coordinate, u8>,
    start: Coordinate,
    end: Coordinate,
    width: i16,
    height: i16,
}

#[derive(Debug)]
struct InputParseError;

impl FromStr for Input {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut x = 0;
        let mut y = 0;
        let mut start = (0,0);
        let mut end = (0,0);
        let mut map = HashMap::new();
        for char in s.replace("\r","").trim().as_bytes() {
            if *char == b'\n' {
                x = 0;
                y += 1;
                continue;
            }
            let height = match char {
                b'S' => {
                    start = (x, y);
                    0
                },
                b'E' => { 
                    end = (x, y);
                    25
                }
                _ => {
                    char-b'a'
                }
            };
            map.insert((x, y), height);
            x += 1;
        };
        Ok(Input{ map, start, end, width: x, height: y+1})
    }
}

fn parse_input(s: &str) -> Input {
    s.parse().expect("Unable to parse input")
}

const NEIGHBOR_DELTA: [Coordinate; 4] = [(-1,0), (1,0), (0,-1), (0,1)];

fn neighbors((x, y): Coordinate, width: i16, height: i16) -> Vec<Coordinate> {
    NEIGHBOR_DELTA.iter().map(|(dx, dy)| (x + dx, y + dy))
        .filter(|(nx, ny)| *nx >= 0 && *nx < width && *ny >= 0 && *ny < height)
        .collect()
}

fn count_steps(input: &Input) -> usize {
    let mut distance = HashMap::new();
    distance.insert(input.start, 0);

    let mut fringe: VecDeque<Coordinate> = VecDeque::new();
    fringe.push_back(input.start);

    let width = input.width;
    let height = input.height;

    while fringe.len() > 0 {
        let p = fringe.pop_front().unwrap();
        if p == input.end {
            // We've found the end, bail
            break;
        }
        let &ph = input.map.get(&p).unwrap();
        let &pd = distance.get(&p).unwrap();
        for np in neighbors(p, width, height) {
            if distance.contains_key(&np) {
                // already reached, this can't be faster
                continue;
            }
            let &nh = input.map.get(&np).unwrap();
            if nh > ph+1 {
                // can't go here
                continue;
            }
            distance.insert(np, pd+1);
            if np == input.end {
                fringe.clear();
                fringe.push_back(np);
                break;
            }

            fringe.push_back(np);
        }
    }


    *distance.get(&input.end).expect("Should have found a path")
}

#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("sample.txt"));

    let result = count_steps(&input);
    assert_eq!(result, 31);
}

pub fn part1() -> usize {
    let input = parse_input(include_str!("input.txt"));
    count_steps(&input)
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 391)
}


fn count_steps_back(input: &Input) -> usize {
    let mut distance = HashMap::new();
    distance.insert(input.end, 0);

    let mut fringe: VecDeque<Coordinate> = VecDeque::new();
    fringe.push_back(input.end);

    let width = input.width;
    let height = input.height;

    let mut better_start = input.start;

    while fringe.len() > 0 {
        let p = fringe.pop_front().unwrap();
        let &ph = input.map.get(&p).unwrap();

        //println!("Considering {:?}@{}", p, ph);

        if ph == 0 {
            // We've found the candidate start
            better_start = p;
            break;
        }
        let &ph = input.map.get(&p).unwrap();
        let &pd = distance.get(&p).unwrap();
        for np in neighbors(p, width, height) {
            if distance.contains_key(&np) {
                // already reached, this can't be faster
                continue;
            }
            let &nh = input.map.get(&np).unwrap();
            //println!(" Neighbor {:?}@{}", np, nh);
            
            if nh < ph-1 {
                // can't go up more than 1
                continue;
            }

            distance.insert(np, pd+1);
            if nh == 0 {
                //println!("Found 0 height: {:?}", np);
                fringe.clear();
                fringe.push_back(np);
                break;
            }

            fringe.push_back(np);
        }
    }

    //_print_map(&distance, width, height);

    *distance.get(&better_start).expect("Should have found a path")
}

fn _print_map(m: &HashMap<Coordinate, usize>, w: i16, h: i16) {
    println!("{}", (0..h).map(|y| {
        String::from_iter((0..w).map(|x| {
            match m.get(&(x,y)) {
                None => " ".to_string(),
                Some(v) => String::from_utf8(vec![(b'A' + (*v as u8))]).unwrap()
            }
        }))
    }).fold("".to_string(), |acc, s| format!("{}\n{}", acc, s)));
}

#[test]
fn part2_sample_works() {
    let input = parse_input(include_str!("sample.txt"));
    let result = count_steps_back(&input);
    assert_eq!(result, 29)
}

pub fn part2() -> usize {
    let input = parse_input(include_str!("input.txt"));
    count_steps_back(&input)
}

#[test]
fn part2_works() {
    assert_eq!(part2(), 386)
}
