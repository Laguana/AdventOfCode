use std::{str::FromStr, collections::HashSet, mem};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Coordinate {
    x: i64,
    y: i64,
}

impl Coordinate {
    pub fn new(x: i64, y: i64) -> Coordinate {
        Coordinate {x, y}
    }

    pub fn manhattan_distance(&self, other: &Coordinate) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug, Clone, Copy)]
struct Beacon {
    sensor: Coordinate,
    beacon: Coordinate,
    range: i64
}

#[derive(Debug)]
struct Input {
    sensors: Vec<Beacon>
}

#[derive(Debug)]
struct InputParseError;

impl FromStr for Input {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Input {sensors: s.trim().lines().map(|l| {
            let words: Vec<&str> = l.split(" ").collect();
            let sxs = &words[2][2..];
            let sys = &words[3][2..];
            let bxs = &words[8][2..];
            let bys = &words[9][2..];
            let sx = sxs[..sxs.len()-1].parse().unwrap();
            let sy = sys[..sys.len()-1].parse().unwrap();
            let bx = bxs[..bxs.len()-1].parse().unwrap();
            let by = bys.parse().unwrap();
            let sensor = Coordinate::new(sx,sy);
            let beacon = Coordinate::new(bx,by);
            Beacon {
                sensor,
                beacon,
                range: sensor.manhattan_distance(&beacon) 
            }
        }).collect()})
    }
}

fn parse_input(s: &str) -> Input {
    s.parse().expect("Unable to parse input")
}

fn count_covered_at_row(input: &Input, row: i64) -> usize {
    let mut covered = HashSet::new();

    for sensor in &input.sensors {
        let sensor_range = sensor.range;
        let row_distance = (sensor.sensor.y - row).abs();
        if sensor_range < row_distance {
            continue;
        }

        // sensor excludes at least one location, unless it is a beacon
        if sensor.beacon.y == row && sensor.beacon.x == sensor.sensor.x {
            continue;
        }
        covered.insert(sensor.sensor.x);
        let remaining = sensor_range - row_distance;
        //println!("Sensor {:?} reaches with {} remaining", sensor.sensor, remaining);

        for dx in 1..=remaining {
            let cx = sensor.sensor.x + dx;
            if sensor.beacon.y != row || sensor.beacon.x != cx {
                //println!(" {}", cx);
                covered.insert(cx);
            }
            let cx = sensor.sensor.x - dx;
            if sensor.beacon.y != row || sensor.beacon.x != cx {
                //println!(" {}", cx);
                covered.insert(cx);
            }
        }
    }

    //println!("{:?}", covered);

    covered.len()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Interval {
    low: i64,
    high: i64
}

impl Interval {
    fn new(low: i64, high: i64) -> Interval {
        assert!(low <= high);
        Interval {low, high}
    }

    pub fn intersects(&self, other: &Interval) -> bool {
        !(self.low > other.high || self.high < other.low)
    }

    pub fn add(&mut self, other: &Interval) -> bool {
        if !self.intersects(other) {
            return false;
        }

        self.low = self.low.min(other.low);
        self.high = self.high.max(other.high);

        return true;
    }

    pub fn span(&self) -> i64 {
        self.high-self.low + 1
    }
}

#[derive(Debug)]
struct Intervals {
    collection: Vec<Interval>
}

impl Intervals {
    pub fn new() -> Intervals {
        Intervals { collection: vec![]}
    }

    pub fn add(&mut self, i: Interval) {
        let mut temp: Vec<Interval> = vec![];
        mem::swap(&mut temp, &mut self.collection);
        let (combine, keep): (Vec<_>, Vec<_>) = temp.into_iter()
            .partition(|e| e.intersects(&i));
        self.collection = keep;
        self.collection.push(combine.into_iter()
            .fold(i, |mut acc, e| {
                assert!(acc.add(&e));
                acc
            }));
    }

    pub fn span(&self) -> i64 {
        let mut result = 0i64;
        for span in &self.collection {
            result += span.span();
        }
        result
    }
}

fn count_covered_at_row_interval(input: &Input, row: i64) -> i64 {
    let mut coverage = Intervals::new();

    for sensor in &input.sensors {
        //println!("{:?} {:?}", sensor, coverage);
        let sensor_range = sensor.range;
        let row_distance = (sensor.sensor.y - row).abs();
        if sensor_range < row_distance {
            continue;
        }

        let remaining = sensor_range - row_distance;

        // sensor excludes at least one location, unless it is a beacon
        if sensor.beacon.y == row {
            // something on the row needs to be omitted from the span here
            if remaining == 0 {
                // actually we only see the beacon, ditch it;
                continue;
            }
            let low = sensor.sensor.x - remaining;
            let high = sensor.sensor.x + remaining;
            //println!("- {}-{} {}?", low, high, sensor.beacon.x);
            if sensor.beacon.x == low {
                coverage.add(Interval::new(low+1, high));
            } else if sensor.beacon.x == high {
                coverage.add(Interval::new(low, high-1));
            } else {
                coverage.add(Interval::new(low, sensor.beacon.x-1));
                coverage.add(Interval::new(sensor.beacon.x+1, high));
            }
        } else {
            // just the one span necessary
            let new_span = Interval::new(sensor.sensor.x - remaining, sensor.sensor.x + remaining);
            // combine it with what we have already
            coverage.add(new_span);
        }
    }

    //println!("{:?}", coverage);
    coverage.span()
}

#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("sample.txt"));
    let result = count_covered_at_row_interval(&input, 10);
    assert_eq!(result, 26)
}

pub fn part1() -> i64 {
    let input = parse_input(include_str!("input.txt"));
    count_covered_at_row_interval(&input, 2000000)
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 5100463)
}

fn is_covered(p: &Coordinate, input: &Input) -> bool {
    for sensor in &input.sensors {
        if p.manhattan_distance(&sensor.sensor) <= sensor.range {
            return true;
        }
    }
    return false;
}

fn walk_fringe(sensor: &Beacon, input: &Input, bound: i64) -> Option<Coordinate> {
    
    let sy = sensor.sensor.y;
    let sx = sensor.sensor.x;
    let range = sensor.range+1;
    
    // top to right
    let ymin = sy-range;
    let ystart = ymin.max(0);
    let yend = sy.min(bound);
    for y in ystart..=yend {
        let dx = y-ymin;
        let x = sx + dx;
        if x < 0 {
            continue;
        }
        if x > bound {
            break;
        }
        let c = Coordinate::new(x,y);
        //println!(" TR Considering {:?}", c);
        if !is_covered(&c, input) {
            return Some(c)
        }
    };

    // top to left
    let ymin = sy-range;
    let ystart = ymin.max(0);
    let yend = sy.min(bound);
    for y in ystart..=yend {
        let dx = y - ymin;
        let x = sx - dx;
        if x < 0 {
            break;
        }
        if x > bound {
            continue;
        }
        let c = Coordinate::new(x,y);
        //println!(" TL Considering {:?}", c);
        if !is_covered(&c, input) {
            return Some(c)
        }
    };

    // right to bottom
    let ystart = sy.max(0);
    let yend = (sy+range).min(bound);
    for y in ystart..=yend {
        let dx = y - sy;
        let x = sx + dx;
        if x < 0 {
            break;
        }
        if x > bound {
            continue;
        }
        let c = Coordinate::new(x,y);
        //println!(" RB Considering {:?}", c);
        if !is_covered(&c, input) {
            return Some(c)
        }
    }

    // left to bottom
    let ystart = sy.max(0);
    let yend = (sy+range).min(bound);
    for y in ystart..=yend {
        let dx = y - sy;
        let x = sx - dx;
        if x < 0 {
            continue;
        }
        if x > bound {
            break;
        }
        let c = Coordinate::new(x,y);
        //println!(" LB Considering {:?}", c);
        if !is_covered(&c, input) {
            return Some(c)
        }
    }
    
    return None;
}

fn locate_distress_beacon(input: &Input, bound: i64) -> Coordinate {
    // Is this a system of simultaneous equations?
    // s@(sx,sy) b@(bx, by) ~> "the answer is not within |s-b| of s"
    //                      ~> |s-p| > |s-b|
    //                      ~> |sx-px| + |sy-py| > |sx-bx| + |sy-by|
    //                      ~> |sx-px| + |sy-py| > d
    //                      ~> |sx-px| + |sy-py| - d > 0
    //                      4 possible cases? px >, px <  X  py>, py <
    //                      - px > sx, py > sy:
    //                          - px - sx + py - sy - d > 0
    //                          ~> px + py - (sx + sy) -d > 0
    //                          ~> px + py > sx + sy + d
    //                      - px < sx, py > sy:
    //                          - sx - px + py - sy - d > 0
    //                          ~> py - px - (sy - sx) - d > 0
    //                          ~> py - px > sy - sx + d
    //                      - px > sx, py < sy:
    //                          - px - sx + sy - py - d > 0
    //                          ~> px - py - (sx - sy) -d > 0
    //                          ~> px - py > sx - sy + d
    //                      - px < sx, py < sy:
    //                          - sx - px + sy - py - d > 0
    //                          ~> -(px + py) + sx + sy - d > 0
    //                          ~> sx + sy -d > px + py
    // also px >= 0, py >= 0, px <= 4000000, py <= 4000000
    // but how do we combine two beacon inequalities together... 4^14 is too many to work with brute force
    //
    // Other thoughts: Somewhere equidistant from all beacons?
    // i.e. |p-b| >= d where d is the detection range of the sensors?
    //  No; it could be adjacent to a beacon
    // Also |p-bi| == |si-bi|+1 for some sensor/beacon pair ?
    //  No; the beacon could be in a different direction from the sensor
    // But, |p-si| == |si-bi|+1 for several sensor/beacon pairs
    //  Could try to walk the fringe of each sensor?
    //   For each sensor, with a detection range of D there is a fringe of 4(D+1) squares just outside the fringe

    for sensor in &input.sensors {
        //println!("Sensor {:?}", sensor.sensor);
        match walk_fringe(sensor, input, bound) {
            Some(p) => return p,
            None => continue,
        }
    }
    panic!("No candidate found");
}

fn pt2_score_coord(c: Coordinate) -> i64 {
    c.x * 4000000 + c.y
}

#[test]
fn part2_sample_works() {
    let input = parse_input(include_str!("sample.txt"));
    let coord = locate_distress_beacon(&input, 20);
    println!("candidate {:?}", coord);
    let result = pt2_score_coord(coord);
    assert_eq!(result, 56000011)
}

pub fn part2() -> i64 {
    let input = parse_input(include_str!("input.txt"));
    let coord = locate_distress_beacon(&input, 4_000_000);
    pt2_score_coord(coord)
}

#[test]
fn part2_works() {
    assert_eq!(part2(), 11557863040754)
}
