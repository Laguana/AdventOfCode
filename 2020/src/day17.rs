use std::collections::HashSet;

#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("inputs/day17.sample"));
    let active = run_input(&input, 6, false);
    assert_eq!(active, 112)
}

type Coord = (isize, isize, isize, isize);

fn parse_input(input: &str) -> HashSet<Coord> {
    input
        .trim()
        .split("\n")
        .enumerate()
        .map(|(y, l)| {
            l.trim()
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(x, _)| (x as isize, y as isize, 0, 0))
                .collect::<HashSet<_>>()
        })
        .flatten()
        .collect()
}

fn determine_state(before: &HashSet<Coord>, (px, py, pz, pw): &Coord, part2: bool) -> bool {
    let mut on = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                let w_range = if part2 { -1..=1 } else { 0..=0 };
                for dw in w_range {
                    if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
                        continue;
                    }
                    if before.contains(&(px + dx, py + dy, pz + dz, pw + dw)) {
                        on += 1;
                    }
                }
            }
        }
    }
    match before.contains(&(*px, *py, *pz, *pw)) {
        false => on == 3,
        true => on == 2 || on == 3,
    }
}

fn run_input(input: &HashSet<Coord>, steps: u32, part2: bool) -> usize {
    let mut state = input.clone();

    for _ in 0..steps {
        //println!("{}: {}", step, state.len());
        let to_consider = state
            .iter()
            .map(|(x, y, z, w)| {
                (-1..=1)
                    .map(|dx| {
                        (-1..=1)
                            .map(|dy| {
                                (-1..=1)
                                    .map(|dz| {
                                        let w_range = if part2 { -1..=1 } else { 0..=0 };
                                        w_range
                                            .map(|dw| (x + dx, y + dy, z + dz, w + dw))
                                            .collect::<HashSet<_>>()
                                    })
                                    .flatten()
                                    .collect::<HashSet<_>>()
                            })
                            .flatten()
                            .collect::<HashSet<_>>()
                    })
                    .flatten()
                    .collect::<HashSet<_>>()
            })
            .flatten()
            .collect::<HashSet<_>>();

        state = to_consider
            .into_iter()
            .filter(|p| determine_state(&state, p, part2))
            .collect::<HashSet<Coord>>();
    }
    state.len()
}

pub fn part1() -> usize {
    let input = parse_input(include_str!("inputs/day17.txt"));
    run_input(&input, 6, false)
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 237)
}

#[test]
fn part2_sample_works() {
    let input = parse_input(include_str!("inputs/day17.sample"));
    let active = run_input(&input, 6, true);
    assert_eq!(active, 848)
}

pub fn part2() -> usize {
    let input = parse_input(include_str!("inputs/day17.txt"));
    run_input(&input, 6, true)
}

#[test]
fn part2_works() {
    assert_eq!(part2(), 2448)
}