use std::{str::FromStr, collections::HashMap};

#[derive(Debug)]
struct Input {
    numbers: Vec<i64>
}

#[derive(Debug)]
struct InputParseError;

impl FromStr for Input {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Input{ numbers: s.trim().lines().map(|l| l.parse().unwrap()).collect()})
    }
}

fn parse_input(s: &str) -> Input {
    s.parse().expect("Unable to parse input")
}

const DECRYPTION_KEY: i64 = 811589153;

struct State {
    indexes: HashMap<usize, usize>,
    rev_indexes: HashMap<usize, usize>,
    zero_idx: usize,
    list: Vec<i64>
}

impl State {
    pub fn new(input: &Input) -> State {
        // Store what index each initial element mapped to
        let mut indexes: HashMap<usize, usize> = HashMap::new();
        let mut rev_indexes: HashMap<usize, usize> = HashMap::new();
        let mut zero_idx = 0;
        input.numbers.iter().enumerate().for_each(|(idx, v)| {
            indexes.insert(idx, idx);
            rev_indexes.insert(idx, idx);
            if *v == 0 {
                zero_idx = idx;
            }
        });
        State {indexes, rev_indexes, zero_idx, list: input.numbers.clone()}
    }
}

// I'm doing something wrong here
// This what we should do
// I don't understand these negative moves
//
//  [1, -3, 2, 3, -2, 0, 4]
//  -3 moves between -2 and 0:
//  [1, 2, 3, -2, -3, 0, 4]
//
//  [1, 2, -2, -3, 0, 3, 4]
//  -2 moves between 4 and 1:
//  [1, 2, -3, 0, 3, 4, -2]
//
// I guess that the element is ~removed from the list, then re-inserted
// and prefers inserting at the back end rather than the front
// The net result is everything ahead moves backwards, and counting
// is a little weird? I.e. moving from i to i+v goes between i+v and (i+v-1)


fn mix(input: &Input, state: &mut State) {
    
    let mut result = state.list.clone();
    let size = input.numbers.len();
    for (idx, &v) in input.numbers.iter().enumerate() {
        if v == 0 {
            continue;
        }
        let current_index = *state.indexes.get(&idx).unwrap();
        let new_index = {
            let candidate_index = (((2*(size-1) + current_index) as i64 + v) as usize) % (size-1);
            if candidate_index == 0 {
                size-1
            } else {
                candidate_index
            }
        };

        //println!("{}/{},{}->{}: {:?}", idx, v, current_index, new_index, result);
        // Everything always moves downwards
        if new_index > current_index {
            // [... X a b c ...]
            // [... a b c X ...]
            for i in current_index+1..=new_index {
                let pi = (i + size - 1) % size;
                //println!(" up: {}:{}={}:{}", pi, result[pi], i, result[i]);
                result[pi] = result[i];
                let initial_index = *state.rev_indexes.get(&i).unwrap();
                state.indexes.insert(initial_index, pi);
                state.rev_indexes.insert(pi, initial_index);
            }
        } else {
            // [... a b c X ...]
            // [... X a b c ...]
            for i in (new_index..current_index).rev() {
                let pi = (i + 1) % size;
                //println!(" down: {}:{}={}:{}", pi,result[pi], i, result[i]);
                result[pi] = result[i];
                let initial_index = *state.rev_indexes.get(&i).unwrap();
                state.indexes.insert(initial_index, pi);
                state.rev_indexes.insert(pi, initial_index);
            }
        }
        result[new_index] = v;
        state.rev_indexes.insert(new_index, idx);
        state.indexes.insert(idx, new_index);
    }
    
    let zero_end_index = *state.indexes.get(&state.zero_idx).unwrap();
    assert_eq!(result[zero_end_index], 0);

    state.zero_idx = zero_end_index;
    state.list = result;
}

pub fn score(list: Vec<i64>, zero: usize) -> i64 {
    let len = list.len();
    let v1000 = list[(zero + 1000) % len];
    let v2000 = list[(zero + 2000) % len];
    let v3000 = list[(zero + 3000) % len];
    //println!("{:?}@{}", list, zero);
    //println!("{},{},{}", v1000, v2000, v3000);
    v1000 + v2000 + v3000
}

#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("sample.txt"));
    let mut state = State::new(&input);
    mix(&input, &mut state);
    let result = score(state.list, state.zero_idx);
    assert_eq!(result, 3);
}

pub fn part1() -> i64 {
    let input = parse_input(include_str!("input.txt"));
    let mut state = State::new(&input);
    mix(&input, &mut state);
    score(state.list, state.zero_idx)
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 16533)
}

#[test]
fn part2_sample_works() {
    let input = parse_input(include_str!("sample.txt"));

    
}

pub fn part2() -> u32 {
    let input = parse_input(include_str!("input.txt"));
    0
}

#[test]
fn part2_works() {
    assert_eq!(part2(), 0)
}
