use core::str::FromStr;
use std::collections::{HashMap, HashSet};

all_aoc::solution!(7, 2022);
enum Line {
    Command(Command),
    List(List),
}
impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('$') {
            Ok(Self::Command(Command::from_str(s)?))
        } else {
            Ok(Self::List(List::from_str(s)?))
        }
    }
}
enum Command {
    ChangeToRoot,
    Change(String),
    ChangeUp,
    List,
}
impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "$ cd /" => Ok(Self::ChangeToRoot),
            "$ cd .." => Ok(Self::ChangeUp),
            "$ ls" => Ok(Self::List),
            x if x.starts_with("$ cd ") => {
                Ok(Self::Change(x.split_once("$ cd ").unwrap().1.to_owned()))
            }
            x => Err(x.to_owned()),
        }
    }
}
enum List {
    Dir(String),
    File(u32, String),
}

impl FromStr for List {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("dir ") {
            Ok(Self::Dir(s.trim_start_matches("dir ").to_owned()))
        } else {
            let (size, name) = s.trim().split_once(' ').unwrap();
            Ok(Self::File(size.parse().unwrap(), name.to_owned()))
        }
    }
}
#[derive(Debug)]
enum FsEntry {
    File { size: u32 },
    Dir { name: String, entries: Vec<Self> },
}
impl FsEntry {
    fn insert(&mut self, path: &[String], file: (String, u32)) {
        match self {
            Self::Dir { entries, .. } => {
                if path.is_empty() {
                    entries.push(Self::File { size: file.1 });
                } else {
                    let dir_name = &path[0];
                    let mut dir = entries
                        .iter_mut()
                        .find(|e| matches!(e, Self::Dir { name, .. } if name == dir_name));

                    if dir.is_none() {
                        entries.push(Self::Dir {
                            name: dir_name.clone(),
                            entries: Vec::new(),
                        });
                        dir = entries.last_mut();
                    }
                    if let Some(d) = dir {
                        d.insert(&path[1..], file);
                    }
                }
            }
            Self::File { .. } => unreachable!("cannot insert into a file"),
        }
    }
}
fn to_fs(map: HashMap<Vec<String>, HashSet<(String, u32)>>) -> FsEntry {
    let mut root = FsEntry::Dir {
        name: "<root>".to_owned(),
        entries: Vec::new(),
    };

    for (path, files) in map {
        let path = if path.first().is_some_and(|s| s == "<root>") {
            path[1..].to_vec()
        } else {
            path
        };

        for (name, size) in files {
            root.insert(&path, (name, size));
        }
    }

    root
}
pub fn part_one(input: &str) -> Option<u32> {
    let root = parse(input);
    let mut sum = 0;
    let mut queue = vec![root];
    while let Some(e) = queue.pop() {
        let c = calc_dir_size(&e);
        match e {
            FsEntry::File { .. } => {}
            FsEntry::Dir { mut entries, .. } => {
                if c < 100_000 {
                    sum += c;
                }
                queue.append(&mut entries);
            }
        }
    }
    Some(sum)
}
fn calc_dir_size(entry: &FsEntry) -> u32 {
    match entry {
        FsEntry::File { size, .. } => *size,
        FsEntry::Dir { entries, .. } => entries.iter().map(calc_dir_size).sum(),
    }
}
pub fn part_two(input: &str) -> Option<u32> {
    let root = parse(input);
    let space_uesd = calc_dir_size(&root);
    let to_free = 30_000_000 - (70_000_000 - space_uesd);
    let mut vec = vec![];
    let mut queue = vec![root];
    while let Some(e) = queue.pop() {
        let c = calc_dir_size(&e);
        match e {
            FsEntry::File { .. } => {}
            FsEntry::Dir { mut entries, .. } => {
                if c >= to_free {
                    vec.push(c);
                }
                queue.append(&mut entries);
            }
        }
    }
    Some(vec.into_iter().min().unwrap())
}
fn parse(input: &str) -> FsEntry {
    let i = input.lines().map(|l| Line::from_str(l).unwrap());
    let mut map: HashMap<Vec<String>, HashSet<(String, u32)>> = HashMap::new();
    let mut curr_path = vec![];
    for c in i {
        match c {
            Line::Command(command) => match command {
                Command::ChangeToRoot => {
                    while !curr_path.is_empty() {
                        curr_path.pop();
                    }
                    curr_path.push("<root>".to_owned());
                }
                Command::Change(s) => curr_path.push(s),
                Command::ChangeUp => {
                    curr_path.pop().unwrap();
                }
                Command::List => {}
            },
            Line::List(list) => match list {
                List::Dir(s) => {
                    curr_path.push(s);
                    map.entry(curr_path.clone()).or_default();
                    curr_path.pop();
                }
                List::File(size, name) => {
                    map.entry(curr_path.clone())
                        .and_modify(|e| {
                            e.insert((name.clone(), size));
                        })
                        .or_insert_with(|| HashSet::from([(name, size)]));
                }
            },
        }
    }
    to_fs(map)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(95_437));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_543_140));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(24_933_642));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_117_448));
    }
}
