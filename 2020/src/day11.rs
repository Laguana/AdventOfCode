#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("inputs/day11.sample"));
    let seats = find_fixpoint(input);
    assert_eq!(seats, 37)
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum SeatState {
    Floor,
    Empty,
    Full,
}

type Point = (usize, usize);

fn parse_input(input: &str) -> Vec<Vec<SeatState>> {
    input
        .trim()
        .split("\n")
        .map(|l| {
            l.trim()
                .chars()
                .map(|c| {
                    if c == '.' {
                        SeatState::Floor
                    } else {
                        SeatState::Empty
                    }
                })
                .collect()
        })
        .collect()
}

fn count_adjacent(input: &Vec<Vec<SeatState>>, (x, y): Point) -> u8 {
    let mut count = 0;
    let height = input.len();
    let width = input[0].len();
    for dy in -1..=1_i8 {
        let y = y.wrapping_add(dy as usize);
        if y >= height {
            continue;
        }
        for dx in -1..=1_i8 {
            if dx == dy && dx == 0 {
                continue;
            }
            let x = x.wrapping_add(dx as usize);
            if x >= width {
                continue;
            }
            let e = input[y][x];
            //println!(" ({},{}): {:?}", x, y, e);
            if e == SeatState::Full {
                count += 1;
            }
        }
    }
    return count;
}

fn step_cell(input: &Vec<Vec<SeatState>>, in_state: SeatState, (x, y): Point) -> SeatState {
    if in_state == SeatState::Floor {
        return SeatState::Floor;
    }
    let adjacent = count_adjacent(input, (x, y));
    //println!("({},{}): {}", x, y, adjacent);
    match in_state {
        SeatState::Empty => {
            if adjacent == 0 {
                SeatState::Full
            } else {
                SeatState::Empty
            }
        }
        SeatState::Full => {
            if adjacent >= 4 {
                SeatState::Empty
            } else {
                SeatState::Full
            }
        }
        _ => panic!("This case was handled earlier"),
    }
}

fn find_fixpoint(input: Vec<Vec<SeatState>>) -> usize {
    let mut prev = input;
    loop {
        //print_state(&prev);
        let next = prev
            .iter()
            .enumerate()
            .map(|(y, r)| {
                r.iter()
                    .enumerate()
                    .map(|(x, &e)| step_cell(&prev, e, (x, y)))
                    .collect::<Vec<_>>()
            })
            .collect();

        if next == prev {
            break;
        } else {
            prev = next;
        }
    }
    prev.iter()
        .map(|l| l.iter().map(|&e| e == SeatState::Full))
        .flatten()
        .filter(|&b| b)
        .count()
}

#[allow(dead_code)]
fn print_state(grid: &Vec<Vec<SeatState>>) {
    println!(
        "{}\n---",
        grid.iter()
            .map(|l| l
                .iter()
                .map(|e| match e {
                    SeatState::Full => '#',
                    SeatState::Empty => 'L',
                    SeatState::Floor => '.',
                })
                .collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    );
}

pub fn part1() -> usize {
    let input = parse_input(include_str!("inputs/day11.txt"));
    find_fixpoint(input)
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 2448)
}
