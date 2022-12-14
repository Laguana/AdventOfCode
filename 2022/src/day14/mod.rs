use std::{str::FromStr, collections::HashMap};

type Coordinate = (u16, u16);

#[derive(Debug)]
struct Input {
    rocks: Vec<Vec<Coordinate>>
}

#[derive(Debug)]
struct InputParseError;

fn parse_coordinate(s: &str) -> Coordinate {
    let parts: Vec<u16> = s.split(",").map(|e| e.parse().unwrap()).collect();
    (parts[0], parts[1])
}

impl FromStr for Input {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Input { rocks: s.trim().lines().map(
                |l| l.split(" -> ").map(|e| 
                    parse_coordinate(e)
                ).collect()
            ).collect()
        })
    }
}

fn parse_input(s: &str) -> Input {
    s.parse().expect("Unable to parse input")
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Rock,
    Sand
}

#[derive(Debug)]
struct Grid {
    grid: HashMap<Coordinate, Cell>,
    top_left: Coordinate,
    bottom_right: Coordinate,
}

impl Grid {
    fn adjust_bounds(&mut self, p: &Coordinate) {
        let (x,y) = p;
        let (l, u) = self.top_left;
        let (r, b) = self.bottom_right;
        self.top_left = (*x.min(&l), *y.min(&u));
        self.bottom_right = (*x.max(&r), *y.max(&b));
    }

    pub fn new(input: &Input) -> Grid {
        let mut result = Grid {grid: HashMap::new(), top_left: (500,0), bottom_right: (500,0)};
        for rock_line in &input.rocks {
            let mut it = rock_line.iter();
            let mut current = it.next().unwrap();
            result.adjust_bounds(current);
            let mut next = it.next();
            while next != None {
                let to = next.unwrap();
                result.adjust_bounds(to);

                let (fx, fy) = current;
                let (tx, ty) = to;
                
                //println!("From {:?} to {:?}", current, to);

                if fx == tx {
                    // vertical
                    let sy = fy.min(ty);
                    let ey = fy.max(ty);
                    for y in *sy..=*ey {
                        //println!("Inserting v {:?}", (fx, y));
                        result.grid.insert((*fx, y), Cell::Rock);
                    };
                } else {
                    // horizontal
                    let sx = fx.min(tx);
                    let ex = fx.max(tx);
                    for x in *sx..=*ex {
                        //println!("Inserting h {:?}", (x, fy));
                        result.grid.insert((x, *fy), Cell::Rock);
                    }
                }

                current = to;
                next = it.next();
            }
        }

        result
    }

    fn drop_grain(&mut self, part2: bool) -> bool {
        let mut p = (500,0);
        let (_, bottom) = self.bottom_right;
        if self.grid.contains_key(&p) {
            return false;
        }
        //println!("Dropping {:?}", p);
        loop {
            //println!(" {:?}", p);
            let (px, py) = p;
            if part2 {
                if py == bottom + 1 {
                    // Blocked: add to grid
                self.grid.insert(p, Cell::Sand);
                return true;
                }
            } else {
                if py > bottom {
                    return false
                }
            }

            if !self.grid.contains_key(&(px, py+1)) {
                p = (px, py+1);
                continue
            }
            if !self.grid.contains_key(&(px-1, py+1)) {
                p = (px-1, py+1);
                continue
            }
            if !self.grid.contains_key(&(px+1, py+1)) {
                p = (px+1, py+1);
                continue
            }
            // Blocked: add to grid
            self.grid.insert(p, Cell::Sand);
            return true;
        }
    }

    pub fn count_sand(&mut self, part2: bool) -> usize {
        let mut grains = 0;
        while self.drop_grain(part2) {
            grains += 1;
        }

        grains
    }

    pub fn count_sand2(&mut self) -> usize {
        let mut result = 0;
        let mut fringe = vec![(500,0)];
        let (_, bottom) = self.bottom_right;
        loop {
            let p = match fringe.pop() {
                Some(p) => p,
                None => return result
            };
            if self.grid.contains_key(&p) {
                continue;
            }
            result += 1;
            self.grid.insert(p, Cell::Sand);
            let (px, py) = p;
            if py >= bottom+1 {
                continue;
            }

            fringe.push((px-1, py+1));
            fringe.push((px, py+1));
            fringe.push((px+1, py+1));
        }
    }
}



#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("sample.txt"));
    let mut grid = Grid::new(&input);
    
    let result = grid.count_sand(false);
    //println!("{:?}", grid);
    assert_eq!(result, 24);
    
}

pub fn part1() -> usize {
    let input = parse_input(include_str!("input.txt"));
    let mut grid = Grid::new(&input);

    grid.count_sand(false)
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 698)
}

#[test]
fn part2_sample_works() {
    let input = parse_input(include_str!("sample.txt"));
    let mut grid = Grid::new(&input);
    
    let result = grid.count_sand2();
    assert_eq!(result, 93);
    
}

pub fn part2() -> usize {
    let input = parse_input(include_str!("input.txt"));
    let mut grid = Grid::new(&input);

    grid.count_sand2()
}

#[test]
fn part2_works() {
    assert_eq!(part2(), 28594)
}
