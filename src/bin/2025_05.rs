use core::ops::RangeInclusive;

use all_aoc::helper::range::ExtRangeOps as _;

all_aoc::solution!(5, 2025);

pub fn part_one(input: &str) -> Option<usize> {
    let (ranges, ids) = parse(input);
    let ranges = merge_ranges(ranges.collect());
    Some(
        ids.filter(|id| ranges.iter().any(|r| r.contains(id)))
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        merge_ranges(parse(input).0.collect())
            .into_iter()
            .map(core::iter::Iterator::count)
            .sum(),
    )
}
fn merge_ranges(mut ranges: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    ranges.sort_by_key(|r| *r.start());
    let ranges = ranges;
    let mut merged = Vec::with_capacity(ranges.len());
    let mut current = ranges[0].clone();

    for r in ranges.into_iter().skip(1) {
        if let Some(u) = current.union(&r) {
            current = u;
        } else {
            merged.push(current);
            current = r;
        }
    }
    merged.push(current);
    merged
}
fn parse(
    input: &str,
) -> (
    impl Iterator<Item = RangeInclusive<u64>>,
    impl Iterator<Item = u64>,
) {
    let (ranges, numbers) = input.split_once("\n\n").unwrap();
    let ranges = ranges
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .map(|(x, y)| x.parse().unwrap()..=y.parse().unwrap());
    let numbers = numbers.lines().map(|line| line.parse().unwrap());
    (ranges, numbers)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(862));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(357_907_198_933_892));
    }
}
