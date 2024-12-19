use std::collections::HashSet;

use all_aoc::helper::permutations::{Combinator, IteratorCombinator};

all_aoc::solution!(24, 2015);

fn execute(input: &str, count: usize) -> Option<u64> {
    let input = parse(input);
    let group_weight = input.iter().sum::<u64>() / count as u64;
    let mut combs = input
        .into_iter()
        .powerset()
        .filter(|v| v.iter().sum::<u64>() == group_weight)
        .collect::<Vec<_>>();

    while let Some(x) = combs
        .iter()
        .enumerate()
        .min_by_key(|(_, d)| (d.len(), d.iter().map(|a| *a as u128).sum::<u128>()))
    {
        let first = combs.swap_remove(x.0);
        let clone = combs
            .iter()
            .filter(|v| first.iter().all(|i| !v.contains(i)))
            .collect::<Vec<_>>();
        let c = Combinator::new(clone, count - 1);
        let s = c.into_iter().any(|v| {
            let count = v.iter().map(|f| f.len()).sum::<usize>();
            let set = v.into_iter().flatten().collect::<HashSet<_>>();
            count == set.len()
        });
        if s {
            return Some(first.into_iter().product());
        }
    }
    None
}
pub fn part_one(input: &str) -> Option<u64> {
    execute(input, 3)
}
pub fn part_two(input: &str) -> Option<u64> {
    execute(input, 4)
}

fn parse(input: &str) -> Vec<u64> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(99));
    }
    #[cfg(feature = "expensive")]
    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(10_439_961_859));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(44));
    }
    #[cfg(feature = "expensive")]
    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(72_050_269));
    }
}
