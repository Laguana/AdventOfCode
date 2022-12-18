use std::{str::FromStr, collections::HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    x: i64,
    y: i64,
    z: i64
}

impl Coordinate {
    fn add (&self, other: &Coordinate) -> Coordinate {
        Coordinate{ x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }

    pub fn new(x: i64, y: i64, z: i64) -> Coordinate {
        Coordinate{x, y, z}
    }

    pub fn adjacent(&self) -> Vec<Coordinate> {
        [Self::new(-1,0,0), Self::new(1,0,0),
         Self::new(0,-1,0), Self::new(0,1,0),
         Self::new(0,0,-1), Self::new(0,0,1)]
         .map(|d| self.add(&d)).into()
    }
}

#[derive(Debug)]
struct Input {
    cubes: HashSet<Coordinate>
}

#[derive(Debug)]
struct InputParseError;

impl FromStr for Input {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Input {cubes: s.trim().lines().map(|l| {
            let parts: Vec<_> = l.split(",").map(|e| e.parse::<i64>().unwrap()).collect();
            let x = parts[0];
            let y = parts[1];
            let z = parts[2];
            Coordinate{x, y, z}
        }).collect()})
    }
}

fn parse_input(s: &str) -> Input {
    s.parse().expect("Unable to parse input")
}

fn surface_area(input: &Input) -> usize {
    let mut result = 0;

    for cube in &input.cubes {
        result += cube.adjacent().iter().filter(|c| !input.cubes.contains(c)).count();
    }

    result
}

#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("sample.txt"));
    let result = surface_area(&input);
    assert_eq!(result, 64);
    
}

pub fn part1() -> usize {
    let input = parse_input(include_str!("input.txt"));
    surface_area(&input)
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 4322)
}

#[derive(Debug, PartialEq, Eq)]
enum Classification {
    Interior,
    Exterior,
    Unknown
}

fn _print_diagram(cubes: &HashSet<Coordinate>) {
    let (x_min, x_max, y_min, y_max, z_min, z_max) = cubes.iter()
        .fold((i64::MAX,0, i64::MAX, 0, i64::MAX, 0), |(xl, xh, yl, yh, zl, zh), c| 
            (xl.min(c.x), xh.max(c.x),
             yl.min(c.y), yh.max(c.y),
             zl.min(c.z), zh.max(c.z)));
    
    for z in z_min..=z_max {
        for y in y_min..=y_max {
            println!("|{}|", String::from_iter((x_min..=x_max).map(|x| {
                if cubes.contains(&Coordinate::new(x,y,z)) {
                    '#'
                } else {
                    ' '
                }
            })));
        }
        println!{"---"}
    }
}

fn fill_inner(input: &mut Input) {
    let (x_min, x_max, y_min, y_max, z_min, z_max) = input.cubes.iter()
        .fold((i64::MAX,0, i64::MAX, 0, i64::MAX, 0), |(xl, xh, yl, yh, zl, zh), c| 
            (xl.min(c.x), xh.max(c.x),
             yl.min(c.y), yh.max(c.y),
             zl.min(c.z), zh.max(c.z)));
    
    let mut interior: HashSet<Coordinate> = HashSet::new();
    let mut exterior: HashSet<Coordinate> = HashSet::new();

    for x in x_min..=x_max {
        //println!("x: {}", x);
        for y in y_min..=y_max {
            //println!(" y: {}", y);
            for z in z_min..=z_max {
                //println!("  z: {}", z);
                let mut current_region: HashSet<Coordinate> = HashSet::new();
                let c = Coordinate::new(x,y,z);
                if input.cubes.contains(&c) {
                    continue;
                }
                let mut fringe = vec![c];
                current_region.extend(fringe.iter());

                let mut classification = Classification::Unknown;

                loop {
                    let c = match fringe.pop() {
                        None => break,
                        Some(c) => c
                    };

                    //println!("     {:?}", c);

                    let new_coords: Vec<_> = c.adjacent().into_iter()
                        .filter(|cn| !input.cubes.contains(&cn) && !current_region.contains(&cn)).collect();
                    //println!("     {:?}", new_coords);
                    
                    for cn in &new_coords {
                        if interior.contains(&cn) {
                            classification = Classification::Interior;
                            break;
                        } else if exterior.contains(&cn) {
                            classification = Classification::Exterior;
                            break;
                        } else {
                            if cn.x < x_min || cn.x > x_max || cn.y < y_min || cn.y > y_max || cn.z < y_min || cn.z > y_max {
                                classification = Classification::Exterior;
                                break;
                            }
                        }
                    };
                    
                    if classification == Classification::Interior {
                        interior.extend(current_region.iter());
                        current_region.clear();
                        interior.extend(new_coords.into_iter());
                        break;
                    } else if classification == Classification::Exterior {
                        exterior.extend(current_region.iter());
                        current_region.clear();
                        exterior.extend(new_coords.into_iter());
                        break;
                    } else {
                        current_region.extend(new_coords.iter());
                        fringe.extend(new_coords.into_iter());
                    }
                }

                //println!("   {:?}: {:?}", classification, current_region);
                if classification == Classification::Unknown {
                    interior.extend(current_region);
                }
            }
        }
    }
    //println!("{:?}", interior);
    //_print_diagram(&input.cubes);
    input.cubes.extend(interior);
    //_print_diagram(&input.cubes);
}

#[test]
fn part2_sample_works() {
    let mut input = parse_input(include_str!("sample.txt"));
    fill_inner(&mut input);
    let result = surface_area(&input);
    assert_eq!(result, 58);
    
}

pub fn part2() -> usize {
    let mut input = parse_input(include_str!("input.txt"));
    //_print_diagram(&input.cubes);
    fill_inner(&mut input);
    surface_area(&input)
}

#[test]
fn part2_works() {
    assert_eq!(part2(), 2516)
}
