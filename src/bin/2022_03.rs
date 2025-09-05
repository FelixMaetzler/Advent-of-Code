use std::collections::HashSet;

all_aoc::solution!(3, 2022);
fn calc_rucksack(input: &str) -> u32 {
    let n = input.len() / 2;
    let set_1 = input.chars().take(n).collect::<HashSet<_>>();
    let set_2 = input.chars().skip(n).collect::<HashSet<_>>();
    let intersect = *set_1.intersection(&set_2).next().unwrap();
    if intersect.is_ascii_lowercase() {
        (u8::try_from(intersect).unwrap() - b'a' + 1).into()
    } else {
        (u8::try_from(intersect).unwrap() - b'A' + 27).into()
    }
}
fn calc_elves(input: &[&str]) -> u32 {
    let set_1 = input[0].chars().collect::<HashSet<_>>();
    let set_2 = input[1].chars().collect::<HashSet<_>>();
    let set_3 = input[2].chars().collect::<HashSet<_>>();
    let set = set_1.intersection(&set_2).copied().collect();
    let intersect = *set_3.intersection(&set).next().unwrap();

    if intersect.is_ascii_lowercase() {
        (u8::try_from(intersect).unwrap() - b'a' + 1).into()
    } else {
        (u8::try_from(intersect).unwrap() - b'A' + 27).into()
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    Some(parse(input).map(calc_rucksack).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        parse(input)
            .collect::<Vec<_>>()
            .chunks_exact(3)
            .map(calc_elves)
            .sum(),
    )
}
fn parse(input: &str) -> impl DoubleEndedIterator<Item = &str> {
    input.lines()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(157));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(7_691));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(70));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(2_508));
    }
}
