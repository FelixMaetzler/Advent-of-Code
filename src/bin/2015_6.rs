use std::{collections::HashMap, ops::RangeInclusive, str::FromStr};

use all_aoc::helper::position::Position;

all_aoc::solution!(6, 2015);
enum Mode {
    On,
    Off,
    Toggle,
}

struct Instruction {
    m: Mode,
    range: RangeInclusive<Position>,
}
impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec: Vec<_> = s.split(' ').collect();
        let m = match vec[0] {
            "toggle" => Mode::Toggle,
            "turn" => match vec[1] {
                "on" => Mode::On,
                "off" => Mode::Off,
                _ => return Err(()),
            },
            _ => return Err(()),
        };
        let start = vec.iter().rev().nth(2).unwrap().parse().unwrap();
        let end = vec.iter().rev().nth(0).unwrap().parse().unwrap();
        Ok(Self {
            m,
            range: start..=end,
        })
    }
}
pub fn part_one(input: &str) -> Option<usize> {
    let mut grid = (0..1000)
        .flat_map(|x| (0..1000).map(move |y| (Position { x, y }, false)))
        .collect::<HashMap<_, _>>();
    let vec = parse(input);
    for instruction in vec {
        match instruction.m {
            Mode::On => {
                for pos in generate_all_combs(instruction.range) {
                    grid.entry(pos).and_modify(|e| *e = true);
                }
            }
            Mode::Off => {
                for pos in generate_all_combs(instruction.range) {
                    grid.entry(pos).and_modify(|e| *e = false);
                }
            }
            Mode::Toggle => {
                for pos in generate_all_combs(instruction.range) {
                    grid.entry(pos).and_modify(|e| *e = !*e);
                }
            }
        }
    }
    Some(grid.values().filter(|x| **x).count())
}
pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = (0..1000)
        .flat_map(|x| (0..1000).map(move |y| (Position { x, y }, 0_u32)))
        .collect::<HashMap<_, _>>();
    let vec = parse(input);
    for instruction in vec {
        match instruction.m {
            Mode::On => {
                for pos in generate_all_combs(instruction.range) {
                    grid.entry(pos).and_modify(|e| *e += 1);
                }
            }
            Mode::Off => {
                for pos in generate_all_combs(instruction.range) {
                    grid.entry(pos).and_modify(|e| *e = e.saturating_sub(1));
                }
            }
            Mode::Toggle => {
                for pos in generate_all_combs(instruction.range) {
                    grid.entry(pos).and_modify(|e| *e += 2);
                }
            }
        }
    }
    Some(grid.values().cloned().sum())
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| Instruction::from_str(l).unwrap())
        .collect()
}
fn generate_all_combs(range: RangeInclusive<Position>) -> impl Iterator<Item = Position> {
    let start = *range.start();
    let end = *range.end();

    (start.y..=end.y).flat_map(move |y| (start.x..=end.x).map(move |x| Position { x, y }))
}
#[cfg(feature = "expensive")]
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(543_903));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(14_687_245));
    }
}
