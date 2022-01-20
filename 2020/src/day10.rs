use memoize::memoize;

#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("inputs/day10.sample"));
    let (d1, d3) = compute_distribution(&input);
    assert_eq!((d1, d3), (7, 5))
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .trim()
        .replace("\r", "")
        .split("\n")
        .map(|l| l.parse())
        .collect::<Result<_, _>>()
        .expect("Should be parsable")
}

fn compute_distribution(input: &Vec<u32>) -> (usize, usize) {
    let mut sorted: Vec<u32> = Vec::new();
    sorted.extend_from_slice(input);
    sorted.sort_unstable();
    let mut d1 = 0;
    let mut d3 = 1;
    let mut prev = 0;
    for e in sorted {
        let diff = e - prev;
        prev = e;
        match diff {
            1 => d1 += 1,
            3 => d3 += 1,
            2 => (),
            _ => panic!("Invalid input"),
        }
    }
    return (d1, d3);
}

pub fn part1() -> usize {
    let input = parse_input(include_str!("inputs/day10.txt"));
    let (d1, d3) = compute_distribution(&input);
    return d1 * d3;
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 1856)
}

#[test]
fn part2_sample_works() {
    let input = parse_input(include_str!("inputs/day10.sample"));
    let orderings = count_orderings(&input);
    assert_eq!(orderings, 8)
}

#[test]
fn part2_sample2_works() {
    let input = parse_input(include_str!("inputs/day10.sample2"));
    let orderings = count_orderings(&input);
    assert_eq!(orderings, 19208)
}

#[memoize]
fn streak_combinations(streak: usize) -> usize {
    // we had a streak of `streak` 1s in a row, up to this
    // 3 jump that we need to reach exactly
    // How many ways can we go through `streak` 1s in steps of 1, 2 or 3?
    match streak {
        0 => 1,
        1 => 1,         // we went x -> x+1 -> x+4, that's the only way
        2 => 1 + 1,     // go direct, or go via intermediate
        3 => 1 + 1 + 2, // go direct, go via 1 step, go via 2 step
        // for general case, you can either do a 3 step followed by whatever happens next, or
        // do a 2 step followed by whatever happens next, or a 1 step followed by whatever happens next. Don't need any +1s
        _ => {
            streak_combinations(streak - 3)
                + streak_combinations(streak - 2)
                + streak_combinations(streak - 1)
        }
    }
}

fn count_orderings(input: &Vec<u32>) -> usize {
    let mut sorted: Vec<u32> = Vec::new();
    sorted.extend_from_slice(input);
    sorted.sort_unstable();

    //println!("{:?}", sorted);

    let mut orderings = 1;
    let mut streak = 0;
    let mut prev = 0;
    for e in sorted {
        if e - prev == 1 {
            streak += 1;
        } else {
            /*println!(
                "Streak {} at element {}: {}",
                streak,
                e,
                streak_combinations(streak)
            );*/
            orderings *= streak_combinations(streak);
            streak = 0;
        }
        prev = e;
    }

    orderings * streak_combinations(streak)
}

pub fn part2() -> usize {
    let input = parse_input(include_str!("inputs/day10.txt"));
    count_orderings(&input)
}

#[test]
fn part2_works() {
    assert_eq!(part2(), 2314037239808)
}
