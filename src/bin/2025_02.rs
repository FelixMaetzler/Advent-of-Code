use core::iter;
use core::ops::RangeInclusive;
use std::collections::HashSet;

use all_aoc::helper::misc::{Joinable as _, number_to_digit_count};

all_aoc::solution!(2, 2025);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        parse(input)
            .flat_map(|r| {
                if number_to_digit_count(*r.start()) == number_to_digit_count(*r.end()) {
                    [Some(r), None]
                } else {
                    [
                        Some(
                            *r.start()
                                ..=10_u64.pow(u32::from(number_to_digit_count(*r.start()))) - 1,
                        ),
                        Some(
                            (10_u64.pow(u32::from(number_to_digit_count(*r.start()))) + 1)
                                ..=*r.end(),
                        ),
                    ]
                }
            })
            .flatten()
            .map(execute_range_part_1)
            .sum(),
    )
}
fn execute_range_part_1(range: RangeInclusive<u64>) -> u64 {
    let start = range.start().to_string();
    let end = range.end().to_string();
    debug_assert_eq!(start.len(), end.len());
    if start.len() % 2 == 1 {
        return 0;
    }
    let first_half_start: u64 = start[..start.len() / 2].parse().unwrap();
    let first_half_end: u64 = end[..end.len() / 2].parse().unwrap();

    (first_half_start..=first_half_end)
        .map(|i| format!("{i}{i}").parse().unwrap())
        .filter(|i| range.contains(i))
        .sum()
}
pub fn part_two(input: &str) -> Option<u64> {
    Some(
        parse(input)
            .flat_map(|r| {
                if number_to_digit_count(*r.start()) == number_to_digit_count(*r.end()) {
                    [Some(r), None]
                } else {
                    [
                        Some(
                            *r.start()
                                ..=10_u64.pow(u32::from(number_to_digit_count(*r.start()))) - 1,
                        ),
                        Some(
                            (10_u64.pow(u32::from(number_to_digit_count(*r.start()))) + 1)
                                ..=*r.end(),
                        ),
                    ]
                }
            })
            .flatten()
            .map(execute_range_part_2)
            .sum(),
    )
}
fn execute_range_part_2(range: RangeInclusive<u64>) -> u64 {
    let length = number_to_digit_count(*range.end());
    let mut set = HashSet::new();
    for i in 1..length {
        set.extend(execute_range_part_2_with_n(range.clone(), i.into()));
    }
    set.into_iter().filter(|v| range.contains(v)).sum()
}
fn execute_range_part_2_with_n(range: RangeInclusive<u64>, pattern_length: usize) -> HashSet<u64> {
    let start = range.start().to_string();
    let end = range.end().to_string();
    debug_assert_eq!(start.len(), end.len());
    if !start.len().is_multiple_of(pattern_length) {
        return HashSet::new();
    }
    let pattern_count = start.len() / pattern_length;

    let first_half_start: u64 = start[..pattern_length].parse().unwrap();
    let first_half_end: u64 = end[..pattern_length].parse().unwrap();

    (first_half_start..=first_half_end)
        .map(|i| {
            iter::repeat_n(i, pattern_count)
                .join("")
                .parse::<u64>()
                .unwrap()
        })
        .collect()
}
fn parse(input: &str) -> impl Iterator<Item = RangeInclusive<u64>> {
    input.split(',').map(|range| {
        let (x, y) = range.split_once('-').unwrap();
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();
        x..=y
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(1_227_775_554));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(12_850_231_731));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(4_174_379_265));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(24_774_350_322));
    }
}
