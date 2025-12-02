use core::ops::RangeInclusive;

use all_aoc::helper::misc::number_to_digit_count;

all_aoc::solution!(2, 2025);

/// Notice that 343434 is 34 * 10101
/// the `length` is 6 and the `pattern_count` is 3
/// (there are 3 patterns with each length 2)
/// 10101 is the resulting magic number because if my number (e.g. 343434)
/// is divisible by the magic number, it has a repeating pattern.
fn magic_number(length: usize, pattern_count: usize) -> u64 {
    if pattern_count == 0 {
        return 0;
    }

    let zeros_between = (length / pattern_count) - 1;
    let mut result: u64 = 0;
    let mut multiplier: u64 = 1;
    #[expect(clippy::cast_possible_truncation, reason = "zeros_between cant be big")]
    let c = 10_u64.pow(zeros_between as u32 + 1);
    for _ in 0..pattern_count - 1 {
        result += multiplier;
        multiplier *= c;
    }
    result += multiplier;

    result
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        parse(input)
            .flatten()
            .filter(|x| !is_valid_part_1(*x))
            .sum(),
    )
}

fn is_valid_part_1(x: u64) -> bool {
    let len = number_to_digit_count(x) as usize;
    if len.is_multiple_of(2) {
        !x.is_multiple_of(magic_number(len, 2))
    } else {
        true
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        parse(input)
            .flatten()
            .filter(|x| !is_valid_part_2(*x))
            .sum(),
    )
}
fn is_valid_part_2(x: u64) -> bool {
    let len = number_to_digit_count(x) as usize;
    debug_assert!(len > 0);
    let mut i = len;
    while i != 1 {
        if len.is_multiple_of(i) && x.is_multiple_of(magic_number(len, i)) {
            return false;
        }
        i -= 1;
    }
    for i in 2..=len {
        if len.is_multiple_of(i) && x.is_multiple_of(magic_number(len, i)) {
            return false;
        }
    }
    true
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
