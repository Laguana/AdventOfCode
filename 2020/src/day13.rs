#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("inputs/day13.sample"));
    let (wait, id) = next_departure(&input);
    assert_eq!((wait, id), (5, 59))
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Departure {
    X,
    Bus(i128),
}

struct Input {
    start_time: i128,
    departures: Vec<Departure>,
}

fn parse_input(input: &str) -> Input {
    let lines = input
        .trim()
        .split("\n")
        .map(|l| l.trim())
        .collect::<Vec<_>>();
    let start_time = lines[0].parse::<i128>().expect("Should be parsable");
    let departures = lines[1]
        .split(",")
        .map(|v| {
            if v == "x" {
                Departure::X
            } else {
                Departure::Bus(v.parse::<i128>().expect("Should be parsable"))
            }
        })
        .collect();
    Input {
        start_time: start_time,
        departures: departures,
    }
}

fn next_departure(input: &Input) -> (i128, i128) {
    let offset = input.start_time;
    input
        .departures
        .iter()
        .fold((i128::MAX, 0), |(wait, id), e| match e {
            Departure::X => (wait, id),
            Departure::Bus(v) => {
                let new_wait = v - (offset % v);
                if new_wait < wait {
                    (new_wait, *v)
                } else {
                    (wait, id)
                }
            }
        })
}

pub fn part1() -> i128 {
    let input = parse_input(include_str!("inputs/day13.txt"));
    let (wait, id) = next_departure(&input);
    return wait * id;
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 5257)
}

fn find_scheduling_sequence(input: &Input) -> i128 {
    // Need to find a time t such that for each (i,e) in the input, t+i % e == 0
    // All bus inputs appear to be prime; which means that we we can operate modulo prod(e) = P
    // for (i,e) in the input, t+i %e == 0 --> t%e == (e-i) --> (t % P) % e == (e-i)
    // which does what for us?
    // Oh right, the chinese remainder theorem applies
    //
    let data = input
        .departures
        .iter()
        .enumerate()
        .filter(|(_, &e)| e != Departure::X)
        .map(|(i, e)| match e {
            Departure::Bus(v) => ((i as i128) % v, v),
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();
    //let base = data.iter().fold(1, |acc, (_, &p)| acc*p);
    //println!("%{}: {:?}", base, data);

    let (result, _) = data.iter().fold((0, 1), |(t, base), (r, &p)| {
        // We want to solve t%base == t and t%p == -r
        let (x,_) = euclid(base, p);
        //println!("({},{}) ({},{}) -> {},{}", t,base,r,p, x,y);
        // base*x + y*p == 1

        let new_base = base*p;
        let mut new_t = (t + (p-r-t)*x*base) % new_base;

        let mut iterations = 0;
        
        while new_t < 0  {
            new_t += new_base;
            iterations += 1;
            if iterations >= 100 {
                panic!("{} {} {}", iterations, new_t, new_base);
            }
            
        }
        return (new_t, new_base)
    });

    result
}

fn euclid(a: i128, b: i128) -> (i128, i128) {
    // Compute x, y such that ax + by = gcd(a,b)
    let mut r = std::cmp::max(a,b);
    let mut r2 = std::cmp::min(a,b);
    let mut s = 1;
    let mut s2 = 0;
    let mut t = 0;
    let mut t2 = 1;

    loop {
        // ri+1 = ri-1 -qi*ri
        let q = r / r2;
        let rn = r - q*r2;
        if rn == 0 {
            if a > b {
                return (s2, t2)
            } else {
                return (t2,s2)
            }
        }

        let sn = s - q*s2;
        let tn = t - q*t2;

        r = r2;
        r2 = rn;
        s = s2;
        s2 = sn;
        t = t2;
        t2 = tn;
    }
}

#[test]
fn part2_sample_works() {
    let input = parse_input(include_str!("inputs/day13.sample"));
    let time = find_scheduling_sequence(&input);
    assert_eq!(time, 1068781)
}

pub fn part2() -> i128 {
    let input = parse_input(include_str!("inputs/day13.txt"));
    find_scheduling_sequence(&input)
}

#[test]
fn part2_works() {
    assert_eq!(part2(), 538703333547789)
}
