use std::collections::HashSet;

#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("inputs/day17.sample"));
    let active = run_input(&input, 6);
    assert_eq!(active, 112)
}

type Coord = (isize, isize, isize);

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
                .map(|(x, _)| (x as isize, y as isize, 0))
                .collect::<HashSet<_>>()
        })
        .flatten()
        .collect()
}

fn determine_state(before: &HashSet<Coord>, (px,py,pz): &Coord) -> bool {
    let mut on = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                if dx == 0 && dy == 0 && dz == 0 {
                    continue;
                }
                if before.contains(&(px+dx,py+dy,pz+dz)) {
                    on += 1;
                }
            }
        }
    }
    match before.contains(&(*px,*py,*pz)) {
        false => on == 3,
        true => on == 2 || on == 3,
    }
}

fn run_input(input: &HashSet<Coord>, steps: u32) -> usize {
    let mut state = input.clone();

    for step in 0..steps {
        //println!("{}: {}", step, state.len());
        let to_consider = state
            .iter()
            .map(|(x, y, z)| {
                (-1..=1)
                    .map(|dx| {
                        (-1..=1)
                            .map(|dy| {
                                (-1..=1)
                                    .map(|dz| (x + dx, y + dy, z + dz))
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
        
        state = to_consider.into_iter().filter(|p| determine_state(&state, p)).collect::<HashSet<Coord>>();
    }
    state.len()
}

pub fn part1() -> usize {
    let input = parse_input(include_str!("inputs/day17.txt"));
    run_input(&input, 6)
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 237)
}