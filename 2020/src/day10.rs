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
        .collect::<Result<_,_>>()
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
        let diff = e-prev;
        prev = e;
        match diff {
            1 => d1 += 1,
            3 => d3 += 1,
            2 => (),
            _ => panic!("Invalid input")
        }
    }
    return (d1, d3)
}

pub fn part1() -> usize {
    let input = parse_input(include_str!("inputs/day10.txt"));
    let (d1,d3) = compute_distribution(&input);
    return d1*d3
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 1856)
}