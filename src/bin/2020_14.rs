use std::{collections::HashMap, str::FromStr};

use all_aoc::helper::{bitmask::Bitmask, permutations::IteratorCombinator};

all_aoc::solution!(14, 2020);
#[derive(Debug)]
enum Instruction {
    Mask(Vec<(usize, bool)>),
    Write { index: usize, val: u64 },
}
impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("mask") {
            let res = s.split_once(" = ").unwrap().1;
            let m = res
                .chars()
                .rev()
                .enumerate()
                .flat_map(|(i, c)| match c {
                    'X' => None,
                    '1' => Some((i, true)),
                    '0' => Some((i, false)),
                    _ => unreachable!(),
                })
                .collect();
            Ok(Self::Mask(m))
        } else if s.starts_with("mem") {
            let (i, n) = s.split_once(" = ").unwrap();
            let index = i
                .strip_prefix("mem[")
                .unwrap()
                .strip_suffix(']')
                .unwrap()
                .parse()
                .unwrap();
            let val = n.parse().unwrap();
            Ok(Self::Write { index, val })
        } else {
            Err(s.to_string())
        }
    }
}
pub fn part_one(input: &str) -> Option<u64> {
    let vec = parse(input);
    let mut mem = HashMap::new();
    let mut mask = None;
    for ins in vec {
        match ins {
            Instruction::Mask(items) => mask = Some(items),
            Instruction::Write { index, val } => {
                let mut val = val;
                for (i, b) in mask.as_ref().unwrap() {
                    val.set_bit(*i, *b);
                }
                mem.insert(index, val);
            }
        }
    }
    Some(mem.values().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let vec = parse(input);
    let mut mem = HashMap::new();
    let mut mask = None;
    for ins in vec {
        match ins {
            Instruction::Mask(items) => {
                mask = Some(items);
            }
            Instruction::Write { index, val } => {
                let x = use_mask(index, mask.as_ref().unwrap());
                let indices = generate_all_combs(x.0, &x.1);
                for i in indices {
                    mem.insert(i, val);
                }
            }
        }
    }
    Some(mem.values().sum())
}
fn use_mask(mem: usize, mask: &[(usize, bool)]) -> (usize, Vec<usize>) {
    let mask: HashMap<usize, bool> = HashMap::from_iter(mask.iter().copied());
    let mut v = vec![];
    let mut mem = mem;
    for i in 0..36 {
        if let Some(x) = mask.get(&i) {
            if *x {
                mem.set_bit(i, true);
            }
        } else {
            v.push(i);
        }
    }
    (mem, v)
}
fn generate_all_combs(mem: usize, floating_index: &[usize]) -> Vec<usize> {
    floating_index
        .iter()
        .powerset()
        .map(|s| {
            let mut mem = mem;
            for i in floating_index {
                mem.set_bit(*i, s.contains(&i));
            }
            mem
        })
        .collect()
}
fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| Instruction::from_str(l).unwrap())
        .collect()
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
        assert_eq!(result, Some(165));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(12_512_013_221_615));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            all_aoc::cli::read_examples_file(DAY)
                .split_once("\n\n")
                .unwrap()
                .1,
        );
        assert_eq!(result, Some(208));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(3_905_642_473_893));
    }
}
