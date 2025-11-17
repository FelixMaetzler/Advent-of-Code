all_aoc::solution!(25, 2019);

use core::iter;
use std::collections::{HashMap, HashSet};

use all_aoc::helper::{
    graph::{Graph as _, Special, WithWeights as _},
    permutations::IteratorCombinator as _,
};
use all_aoc::helper::{
    intcode::{InputMode, IntInteger, Intcode, Return},
    misc::Joinable as _,
    position::Direction4,
};
#[expect(dead_code, reason = "is a valid command")]
enum Command {
    Direction(Direction4),
    Inv,
    Take(String),
    Drop(String),
}

#[derive(Debug)]
enum CommandResult {
    Finished(u32),
    Room(Room),
    Taken(String),
    Dropped(String),
    BackToCheckpoint,
}

#[derive(Debug)]
struct Room {
    name: String,
    doors: Vec<Direction4>,
    items: Vec<String>,
}
impl TryFrom<Vec<String>> for Room {
    type Error = Vec<String>;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let mut vec = value.as_slice();
        let name = if vec
            .first()
            .is_some_and(|s| s.starts_with("== ") && s.ends_with(" =="))
        {
            value[0]
                .trim_start_matches("== ")
                .trim_end_matches(" ==")
                .to_owned()
        } else {
            return Err(value);
        };
        vec = &vec[2..];
        if vec[0] != "Doors here lead:" {
            return Err(value);
        }
        vec = &vec[1..];
        let mut doors = vec![];
        while vec.first().is_some_and(|s| s.starts_with('-')) {
            let dir = match vec[0].as_ref() {
                "- north" => Direction4::North,
                "- south" => Direction4::South,
                "- west" => Direction4::West,
                "- east" => Direction4::East,
                _ => return Err(value),
            };
            vec = &vec[1..];
            doors.push(dir);
        }
        let mut items = vec![];
        if !vec.is_empty() {
            if vec[0] != "Items here:" {
                return Err(value);
            }
            vec = &vec[1..];
            while vec.first().is_some_and(|s| s.starts_with('-')) {
                let item = vec[0].trim_start_matches("- ").to_owned();
                items.push(item);
                vec = &vec[1..];
            }
        }
        if !vec.is_empty() {
            return Err(value);
        }
        Ok(Self { name, doors, items })
    }
}
impl Command {
    fn as_input(&self) -> Vec<IntInteger> {
        let s = match self {
            Self::Direction(direction4) => match direction4 {
                Direction4::North => "north".to_owned(),
                Direction4::East => "east".to_owned(),
                Direction4::West => "west".to_owned(),
                Direction4::South => "south".to_owned(),
            },
            Self::Inv => "inv".to_owned(),
            Self::Take(x) => format!("take {x}"),
            Self::Drop(x) => format!("drop {x}"),
        };
        s.chars()
            .chain(iter::once('\n'))
            .map(|c| c as u8 as isize)
            .collect()
    }
}
fn run_until_command(computer: &mut Intcode) -> CommandResult {
    let mut vec: Vec<String> = vec![];
    while Return::NewOutput == computer.execute() {
        let output = computer.get_outputs();
        let test = output
            .iter()
            .map(|i| u8::try_from(*i).unwrap() as char)
            .join("");
        if !test.ends_with('\n') {
            continue;
        }
        let test = test.trim();
        computer.clear_output();
        if test.is_empty() {
            continue;
        }
        if test == "Command?" {
            return if vec.len() == 1 && vec[0].starts_with("You take the ") {
                let item = vec[0]
                    .trim_end_matches('.')
                    .trim_start_matches("You take the ");
                CommandResult::Taken(item.to_owned())
            } else if vec.len() == 1 && vec[0].starts_with("You drop the ") {
                let item = vec[0]
                    .trim_end_matches('.')
                    .trim_start_matches("You drop the ");
                CommandResult::Dropped(item.to_owned())
            } else if vec
                .iter()
                .any(|s| s.contains("you are ejected back to the checkpoint."))
            {
                return CommandResult::BackToCheckpoint;
            } else {
                let r = Room::try_from(vec.clone()).unwrap();
                CommandResult::Room(r)
            };
        }

        if test.starts_with("\"Oh, hello!") {
            let password = test
                .split_ascii_whitespace()
                .find_map(|s| s.parse().ok())
                .unwrap();
            return CommandResult::Finished(password);
        }
        vec.push(test.to_owned());
    }

    unreachable!()
}
const ITEMS: [&str; 5] = [
    "escape pod",
    "photons",
    "molten lava",
    "giant electromagnet",
    "infinite loop",
];
#[expect(clippy::too_many_lines, reason = "only slightly over")]
pub fn part_one(input: &str) -> Option<u32> {
    let mut computer = Intcode::new(input.split(',').map(|l| l.parse().unwrap()).collect());
    computer.halt_at_output(true);

    let CommandResult::Room(mut r) = run_until_command(&mut computer) else {
        unreachable!()
    };
    let mut visited: HashSet<(usize, Direction4)> = HashSet::new();
    let mut queue = r.doors.iter().map(|d| (0, *d)).collect::<Vec<_>>();
    let mut name_to_index = HashMap::new();
    let mut graph: Special<Direction4> = Special::new();
    name_to_index.insert(r.name.clone(), 0);
    let mut current = 0;
    let mut inventory = HashSet::new();
    while let Some(x) = queue.pop() {
        if visited.contains(&x) {
            continue;
        }
        if name_to_index
            .get("Security Checkpoint")
            .is_some_and(|i| *i == x.0)
        {
            continue;
        }
        visited.insert(x);
        r = go_to((current, r), x.0, &graph, &mut computer);
        for item in &r.items {
            if ITEMS.contains(&item.as_str()) {
                continue;
            }
            computer.set_inputs(
                Command::Take(item.clone()).as_input().into_iter(),
                InputMode::Replace,
            );
            if let CommandResult::Taken(i) = run_until_command(&mut computer) {
                assert_eq!(item, &i);
                let b = inventory.insert(i);
                assert!(b);
            } else {
                unreachable!()
            }
        }
        current = name_to_index[&r.name];
        computer.set_inputs(
            Command::Direction(x.1).as_input().into_iter(),
            InputMode::Replace,
        );
        if let CommandResult::Room(room) = run_until_command(&mut computer) {
            r = room;
        } else {
            unreachable!()
        }
        let len = name_to_index.len();
        let new_index = *name_to_index.entry(r.name.clone()).or_insert(len);
        graph.add_edge(current, new_index, x.1);
        graph.add_edge(new_index, current, x.1.opposite());
        current = new_index;
        for dir in &r.doors {
            queue.push((current, *dir));
        }
    }
    let all_items = inventory.clone();
    r = go_to(
        (current, r),
        name_to_index["Security Checkpoint"],
        &graph,
        &mut computer,
    );
    current = name_to_index["Security Checkpoint"];
    let grpah_dirs = graph
        .outgoing(current)
        .map(|i| graph.weight(current, i).unwrap())
        .collect::<HashSet<_>>();
    let mut d = r.doors;
    d.retain(|e| !grpah_dirs.contains(e));
    assert_eq!(d.len(), 1);
    for should_be in inventory.clone().into_iter().powerset() {
        let should_be: HashSet<String> = should_be.into_iter().collect();
        for s in &all_items {
            if should_be.contains(s) && !inventory.contains(s) {
                computer.set_inputs(
                    Command::Take(s.clone()).as_input().into_iter(),
                    InputMode::Replace,
                );
                let r = inventory.insert(s.clone());
                assert!(r);
                run_until_command(&mut computer);
            } else if !should_be.contains(s) && inventory.contains(s) {
                computer.set_inputs(
                    Command::Drop(s.clone()).as_input().into_iter(),
                    InputMode::Replace,
                );
                let r = inventory.remove(s);
                assert!(r);
                let CommandResult::Dropped(x) = run_until_command(&mut computer) else {
                    unreachable!()
                };
                assert_eq!(&x, s);
            }
        }
        assert_eq!(should_be, inventory);
        // Test if we can go through the door
        computer.set_inputs(
            Command::Direction(d[0]).as_input().into_iter(),
            InputMode::Replace,
        );

        if let CommandResult::Finished(x) = run_until_command(&mut computer) {
            return Some(x);
        }
    }
    unreachable!()
}
fn go_to(
    (start, mut room): (usize, Room),
    end: usize,
    graph: &Special<Direction4>,
    computer: &mut Intcode,
) -> Room {
    if start == end {
        return room;
    }

    let path = graph.all_paths(start, end);
    assert!(!path.is_empty());
    for dir in path[0]
        .windows(2)
        .map(|w| graph.weight(w[0], w[1]).unwrap())
    {
        computer.set_inputs(
            Command::Direction(dir).as_input().into_iter(),
            InputMode::Replace,
        );
        if let CommandResult::Room(r) = run_until_command(computer) {
            room = r;
        } else {
            unreachable!()
        }
    }
    room
}
pub const fn part_two(_: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(2_147_502_592));
    }
}
