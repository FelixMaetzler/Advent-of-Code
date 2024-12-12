use std::{collections::HashMap, iter::successors};

use all_aoc::helper::misc::number_to_digit_count;

all_aoc::solution!(11, 2024);

pub fn part_one(input: &str) -> Option<u64> {
    execute(input, 25)
}

pub fn part_two(input: &str) -> Option<u64> {
    execute(input, 75)
}
fn blink(map: &HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut ret = HashMap::with_capacity(map.len());
    for (stone, count) in map {
        if *stone == 0 {
            *ret.entry(1).or_default() += count;
        } else if number_to_digit_count(*stone) % 2 == 0 {
            let (n1, n2) = split_number(*stone);
            *ret.entry(n1).or_default() += count;
            *ret.entry(n2).or_default() += count;
        } else {
            *ret.entry(2024 * stone).or_default() += count;
        }
    }
    ret
}
fn execute(input: &str, n: usize) -> Option<u64> {
    let map = parse(input).iter().map(|n| (*n, 1)).collect();
    Some(
        successors(Some(map), |nums| Some(blink(nums)))
            .nth(n)
            .unwrap()
            .values()
            .sum(),
    )
}
fn split_number(n: u64) -> (u64, u64) {
    let m = 10_u64.pow(number_to_digit_count(n) as u32 / 2);
    let n2 = n % m;
    let n1 = (n - n2) / m;
    (n1, n2)
}
fn parse(input: &str) -> Vec<u64> {
    input
        .split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(55_312));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(175_006));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(65_601_038_650_482));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(207_961_583_799_296));
    }
}
