use std::collections::HashSet;

use all_aoc::helper::{misc::count_occurrences, permutations::IteratorCombinator as _};

all_aoc::solution!(4, 2017);

pub fn part_one(input: &str) -> Option<usize> {
    Some(parse(input).filter(|l| is_valid_part_one(l)).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(parse(input).filter(|l| is_valid_part_two(l)).count())
}
fn is_valid_part_one(input: &str) -> bool {
    let n = input.split_ascii_whitespace().count();
    input.split_ascii_whitespace().collect::<HashSet<_>>().len() == n
}
fn is_valid_part_two(input: &str) -> bool {
    input
        .split_ascii_whitespace()
        .combinations(2)
        .all(|v| !are_anagrams(v[0], v[1]))
}
fn are_anagrams(first: &str, second: &str) -> bool {
    count_occurrences(first.chars()) == count_occurrences(second.chars())
}
fn parse(input: &str) -> impl Iterator<Item = &str> {
    input.lines()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn anagrams() {
        assert_eq!(part_two("abcde fghij"), Some(1));
        assert_eq!(part_two("abcde xyz ecdab"), Some(0));
        assert_eq!(part_two("a ab abc abd abf abj"), Some(1));
        assert_eq!(part_two("iiii oiii ooii oooi oooo"), Some(1));
        assert_eq!(part_two("oiii ioii iioi iiio"), Some(0));
    }
    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(325));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(119));
    }
}
