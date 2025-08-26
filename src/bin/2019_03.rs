use core::str::FromStr;
use std::collections::{HashMap, HashSet};

use all_aoc::helper::position::{Direction4, Position};

all_aoc::solution!(3, 2019);
struct Instruction {
    dir: Direction4,
    len: u32,
}
impl FromStr for Instruction {
    type Err = char;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = s.chars().next().unwrap();
        let len = s[1..].trim_end().parse().unwrap();
        let dir = match c {
            'L' => Direction4::West,
            'R' => Direction4::East,
            'U' => Direction4::North,
            'D' => Direction4::South,
            x => return Err(x),
        };
        Ok(Self { dir, len })
    }
}
pub fn part_one(input: &str) -> Option<i32> {
    let (one, two) = parse(input);
    let set_one = to_set(&one);
    let set_two = to_set(&two);
    let common = set_one.intersection(&set_two);
    Some(
        common
            .map(|p| p.manhattan_distance(&Position { x: 0, y: 0 }))
            .min()
            .unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (one, two) = parse(input);
    let map_one = to_map(&one);
    let map_two = to_map(&two);
    Some(
        map_one
            .into_iter()
            .filter_map(|(k, v)| map_two.get(&k).map(|s| s + v))
            .min()
            .unwrap(),
    )
}
fn to_set(vec: &[Instruction]) -> HashSet<Position<i32>> {
    let mut set = HashSet::with_capacity(vec.len() * 10);
    let mut curr = Position { x: 0, y: 0 };
    for ins in vec {
        for _ in 0..ins.len {
            curr = curr.direction(ins.dir);
            set.insert(curr);
        }
    }
    set
}
fn to_map(vec: &[Instruction]) -> HashMap<Position<i32>, u32> {
    let mut map = HashMap::with_capacity(vec.len() * 10);
    let mut curr = Position { x: 0, y: 0 };
    let mut dist = 1;
    for ins in vec {
        for _ in 0..ins.len {
            curr = curr.direction(ins.dir);
            map.entry(curr).or_insert(dist);
            dist += 1;
        }
    }
    map
}
fn parse(input: &str) -> (Vec<Instruction>, Vec<Instruction>) {
    let one = input.lines().next().unwrap();
    let two = input.lines().nth(1).unwrap();
    (
        one.split(',')
            .map(|s| Instruction::from_str(s).unwrap())
            .collect(),
        two.split(',')
            .map(|s| Instruction::from_str(s).unwrap())
            .collect(),
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(
            all_aoc::cli::read_examples_file(DAY)
                .split_once("\n\n")
                .unwrap()
                .0,
        );
        assert_eq!(result, Some(159));
        let result = part_one(
            all_aoc::cli::read_examples_file(DAY)
                .split_once("\n\n")
                .unwrap()
                .1,
        );
        assert_eq!(result, Some(135));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(232));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            all_aoc::cli::read_examples_file(DAY)
                .split_once("\n\n")
                .unwrap()
                .0,
        );
        assert_eq!(result, Some(610));
        let result = part_two(
            all_aoc::cli::read_examples_file(DAY)
                .split_once("\n\n")
                .unwrap()
                .1,
        );
        assert_eq!(result, Some(410));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(6_084));
    }
}
