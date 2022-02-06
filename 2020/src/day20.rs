use std::collections::HashMap;

#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("inputs/day20.sample"));
    let result = corner_product(&input);
    assert_eq!(result, 20899048083289);
}

/*
Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...
*/

struct Tile {
    data: Vec<Vec<bool>>,
}

type Input = HashMap<u64, Tile>;

fn parse_input(input: &str) -> Input {
    input
        .replace("\r", "")
        .split("\n\n")
        .map(|tile| {
            let mut lines = tile.lines();
            let first = lines.next().unwrap();
            let id = first[5..first.len() - 1].parse::<u64>().unwrap();
            (
                id,
                Tile {
                    data: lines
                        .map(|l| l.chars().map(|c| c == '#').collect())
                        .collect(),
                },
            )
        })
        .collect()
}

fn compute_sides(tile: &Tile) -> Vec<(u16, u16)> {
    let mut ret = vec![];
    let height = tile.data.len();
    let width = tile.data[0].len();
    // top
    let mut a = 0;
    let mut b = 0;
    for i in 0..width {
        a = a << 1 | if tile.data[0][i] { 1 } else { 0 };
        b = b << 1 | if tile.data[0][width - 1 - i] { 1 } else { 0 }
    }
    ret.push((a, b));
    // right
    let mut a = 0;
    let mut b = 0;
    for i in 0..height {
        a = a << 1 | if tile.data[i][width - 1] { 1 } else { 0 };
        b = b << 1
            | if tile.data[height - 1 - i][width - 1] {
                1
            } else {
                0
            }
    }
    ret.push((a, b));
    // bottom
    let mut a = 0;
    let mut b = 0;
    for i in 0..width {
        a = a << 1 | if tile.data[height - 1][i] { 1 } else { 0 };
        b = b << 1
            | if tile.data[height - 1][width - 1 - i] {
                1
            } else {
                0
            }
    }
    ret.push((a, b));
    // left
    let mut a = 0;
    let mut b = 0;
    for i in 0..height {
        a = a << 1 | if tile.data[i][0] { 1 } else { 0 };
        b = b << 1 | if tile.data[height - 1 - i][0] { 1 } else { 0 }
    }
    ret.push((a, b));
    ret
}

fn corner_product(input: &Input) -> u64 {
    let mut product = 1;
    for (id, t) in input.iter() {
        let sides = compute_sides(t);
        let matched_sides = sides
            .iter()
            .filter(|(a, b)| 
                input.iter().any(|(id2, t2)| {
                    if id == id2 {
                        return false;
                    }
                    compute_sides(t2).iter().any(|(c,_)| a == c || b == c)
                })
            )
            .count();
        if matched_sides == 2 {
            product *= id;
        }
    }
    product
}


pub fn part1() -> u64 {
    let input = parse_input(include_str!("inputs/day20.txt"));
    corner_product(&input)
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 15670959891893)
}