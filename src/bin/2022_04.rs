use core::ops::RangeInclusive;

use all_aoc::helper::range::ExtRangeOps as _;

all_aoc::solution!(4, 2022);

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        parse(input)
            .filter(|(a, b)| a.contains_range(b) || b.contains_range(a))
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        parse(input)
            .filter(|(a, b)| a.intersection(b).is_some())
            .count(),
    )
}
fn parse(input: &str) -> impl Iterator<Item = (RangeInclusive<u32>, RangeInclusive<u32>)> {
    input
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(a, b)| (parse_range(a), parse_range(b)))
}
fn parse_range(input: &str) -> RangeInclusive<u32> {
    let (a, b) = input.split_once('-').unwrap();
    a.parse().unwrap()..=b.parse().unwrap()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(560));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(839));
    }
}
