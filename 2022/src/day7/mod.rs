use std::{str::FromStr, slice::Iter, collections::HashMap};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Command {
    Ls,
    Cd(String)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Data {
    Command(Command),
    Dir(String),
    File(usize, String)
}

#[derive(Debug)]
struct Input {
    session: Vec<Data>
}

#[derive(Debug)]
struct InputParseError;

impl FromStr for Input {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let session = s.replace("\r","").trim().split("\n")
            .map(|line| {
                let parts: Vec<&str> = line.split(" ").collect();
                match parts[0] {
                    "$" => {
                        match parts[1] {
                            "ls" => Data::Command(Command::Ls),
                            "cd" => Data::Command(Command::Cd(parts[2].into())),
                            _ => panic!("Bad input: {}", line)
                        }
                    },
                    "dir" => Data::Dir(parts[1].into()),
                    _ => {
                        let size: usize = parts[0].parse().expect("Didn't get a valid number");
                        Data::File(size, parts[1].into())
                    }
                }
            }).collect::<Vec<Data>>();
        Ok(Input{session})
    }
}

fn parse_input(s: &str) -> Input {
    s.parse().expect("Unable to parse input")
}

#[derive(Debug)]
struct FileSystem {
    files: Vec<(usize, String)>,
    folders: HashMap<String, Box<FileSystem>>
}

impl FileSystem {
    fn extend(&mut self, iter: &mut Iter<Data>) {
        loop {
            let data = iter.next();
            //println!("Looping: {:?}", data);
            match data {
                Some(Data::Command(Command::Ls)) => (),
                Some(Data::Command(Command::Cd(subdir))) => {
                    match subdir.as_str() {
                        ".." => {
                            return;
                        },
                        "/" => {
                            panic!("Didn't expect to jump back to the root")
                        }
                        _ => {
                            self.folders.get_mut(subdir).expect("Folder should exist already!")
                                .extend(iter);
                        }
                    }
                }
                Some(Data::Dir(d)) => {
                    let existed = self.folders.insert(d.to_string(), Box::new(FileSystem {files: vec![], folders: HashMap::new()}));
                    assert!(existed.is_none(), "Folder dir'd twice: {}", d);                
                },
                Some(Data::File(size, name)) => self.files.push((*size, name.clone())),
                None => return
            };
        }
    }

    pub fn from_input(input: &Input) -> FileSystem {
        let mut result = FileSystem{files: vec![], folders: HashMap::new()};

        let mut iter = input.session.iter();

        match iter.next() {
            Some(Data::Command(Command::Cd(d))) => assert_eq!(d, "/"),
            _ => panic!("Invalid starting command {:?}", input.session[0])
        }
        result.extend(&mut iter);
        assert!(iter.next().is_none(), "Iter has data left");

        result
    }

    const SCORE_THRESHOLD: usize = 100000;

    fn exclusive_size(&self) -> usize {
        self.files.iter().fold(0, |acc, (size,_)| acc + *size)
    }

    fn score_internal(&self) -> (usize, usize) {
        let exclusive_size = self.exclusive_size();
        let sub_scores = self.folders.iter()
            .map(|(_, v)| v.score_internal());
        let (size, score) = sub_scores.fold((exclusive_size, 0), |(total_size, acc_score), (size, sub_score)| {
            (total_size + size, acc_score + sub_score)
        });
        if size < FileSystem::SCORE_THRESHOLD {
            (size, score + size)
        } else {
            (size, score)
        }
    }

    pub fn score(&self) -> usize {
        let (_, score) = self.score_internal();
        score
    }

    const TOTAL_AVAILABLE: usize = 70000000;
    const NEED_AVAILABLE: usize = 30000000;

    fn get_candidate(&self, needed: usize) -> (usize, usize) {
        let sub_scores = self.folders.iter()
            .map(|(_, f)| f.get_candidate(needed));
        let (sub_size, candidate) = sub_scores.fold((0, usize::MAX),
            |(acc_size, acc_candidate), (size, candidate)| (acc_size + size, candidate.min(acc_candidate)));
        let exclusive_size = self.exclusive_size();
        let size = sub_size + exclusive_size;
        if size >= needed {
            (size, size.min(candidate))
        } else {
            (size, candidate)
        }
    }

    pub fn deletion_candidate_size(&self) -> usize {
        let (size, _) = self.score_internal();
        let remaining = FileSystem::TOTAL_AVAILABLE - size;
        assert!(remaining < FileSystem::NEED_AVAILABLE);
        let needed = FileSystem::NEED_AVAILABLE - remaining;

        let (_, result) = self.get_candidate(needed);
        result
    }

}

#[test]
fn part1_sample_works() {
    let input = parse_input(include_str!("sample.txt"));

    let filesystem = FileSystem::from_input(&input);

    let result = filesystem.score();
    assert_eq!(result, 95437);
    
}

pub fn part1() -> usize {
    let input = parse_input(include_str!("input.txt"));
    let filesystem = FileSystem::from_input(&input);
    //println!("Produced input: {:?}", filesystem);
    filesystem.score()
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 2061777)
}

#[test]
fn part2_sample_works() {
    let input = parse_input(include_str!("sample.txt"));
    let filesystem = FileSystem::from_input(&input);

    let result = filesystem.deletion_candidate_size();
    assert_eq!(result, 24933642);
}

pub fn part2() -> usize {
    let input = parse_input(include_str!("input.txt"));
    let filesystem = FileSystem::from_input(&input);
    //println!("Produced input: {:?}", filesystem);
    filesystem.deletion_candidate_size()
}

#[test]
fn part2_works() {
    assert_eq!(part2(), 4473403)
}
