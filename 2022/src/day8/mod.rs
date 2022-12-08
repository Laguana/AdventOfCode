use std::str::FromStr;

#[derive(Debug)]
struct Input {
    map: Vec<Vec<u8>>,
    dimensions: (i8, i8)
}

#[derive(Debug)]
struct InputParseError;

impl FromStr for Input {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map: Vec<Vec<u8>> = s.replace("\r","").trim().split("\n")
            .map(|l| l.as_bytes().iter().map(|c| c- b'0').collect()).collect();
        let height = map.len() as i8;
        let width = map[0].len() as i8;
        Ok(Input{ map, dimensions: (width, height) })
    }
}

fn parse_input(s: &str) -> Input {
    s.parse().expect("Unable to parse input")
}

#[derive(Debug, Clone, Copy)]
enum ScanDirection {
    Up,
    Down,
    Left,
    Right,
}

fn scan(input: &Input, x: i8, y: i8, dir: ScanDirection) -> bool {
    let (dx, dy) = match dir {
        ScanDirection::Up => (0i8,-1i8),
        ScanDirection::Down => (0,1),
        ScanDirection::Left => (-1,0),
        ScanDirection::Right => (1,0),
    };
    let candidate_height = input.map[y as usize][x as usize];
    let mut x = x;
    let mut y = y;

    let (w, h) = input.dimensions;

    while x > 0 && x < w-1 && y > 0 && y < h-1 {
        x = x + dx;
        y = y + dy;
        let blocker_height = input.map[y as usize][x as usize];
        if blocker_height >= candidate_height {
            return false;
        }
    }
    return true
}

fn is_visible(input: &Input, x: i8, y: i8) -> bool {
    return scan(input, x, y, ScanDirection::Up)
        || scan(input, x, y, ScanDirection::Down)
        || scan(input, x, y, ScanDirection::Left)
        || scan(input, x, y, ScanDirection::Right)

}

fn count_visible(input: &Input) -> u32 {
    let (w, h) = input.dimensions;
    let mut result: u32 = 0;
    for x in 0..w {
        for y in 0..h {
            if is_visible(input, x, y) {
                result = result+1;
            }
        }
    }

    result
}

#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("sample.txt"));
    let result = count_visible(&input);

    assert_eq!(result, 21)
    
}

pub fn part1() -> u32 {
    let input = parse_input(include_str!("input.txt"));
    count_visible(&input)
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 1669)
}

fn scan_trees(input: &Input, x: i8, y: i8, dir: ScanDirection) -> usize {
    let (dx, dy) = match dir {
        ScanDirection::Up => (0i8,-1i8),
        ScanDirection::Down => (0,1),
        ScanDirection::Left => (-1,0),
        ScanDirection::Right => (1,0),
    };
    let candidate_height = input.map[y as usize][x as usize];
    let mut x = x;
    let mut y = y;
    let mut visible = 0;

    let (w, h) = input.dimensions;

    while x > 0 && x < w-1 && y > 0 && y < h-1 {
        x = x + dx;
        y = y + dy;
        visible = visible + 1;
        let blocker_height = input.map[y as usize][x as usize];
        if blocker_height >= candidate_height {
            return visible;
        }
    }
    return visible
}

fn scenic_score(input: &Input, x: i8, y: i8) -> usize {
    return scan_trees(input, x, y, ScanDirection::Up)
        * scan_trees(input, x, y, ScanDirection::Down)
        * scan_trees(input, x, y, ScanDirection::Left)
        * scan_trees(input, x, y, ScanDirection::Right)
}

fn max_scenic_score(input: &Input) -> usize {
    let (w, h) = input.dimensions;
    let mut result: usize = 0;
    for x in 0..w {
        for y in 0..h {
            let score = scenic_score(input, x, y);
            result = score.max(result);
        }
    }

    result
}

#[test]
fn part2_sample_works() {
    let input = parse_input(include_str!("sample.txt"));

    let result = max_scenic_score(&input);
    assert_eq!(result, 8)
}

pub fn part2() -> usize {
    let input = parse_input(include_str!("input.txt"));
    max_scenic_score(&input)
}

#[test]
fn part2_works() {
    assert_eq!(part2(), 331344)
}
