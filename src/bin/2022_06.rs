use std::collections::HashSet;

all_aoc::solution!(6, 2022);

pub fn part_one(input: &str) -> Option<usize> {
    solve(input, 4)
}

pub fn part_two(input: &str) -> Option<usize> {
    solve(input, 14)
}
fn solve(input: &str, count: usize) -> Option<usize> {
    let v = input.chars().collect::<Vec<_>>();
    Some(
        v.windows(count)
            .enumerate()
            .find(|(_, w)| w.iter().collect::<HashSet<_>>().len() == count)
            .unwrap()
            .0
            + count,
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = &all_aoc::cli::read_examples_file(DAY);
        let v = input
            .lines()
            .map(|l| part_one(l).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(v, vec![7, 5, 6, 10, 11]);
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_912));
    }

    #[test]
    fn test_part_two() {
        let input = &all_aoc::cli::read_examples_file(DAY);
        let v = input
            .lines()
            .map(|l| part_two(l).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(v, vec![19, 23, 23, 29, 26]);
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(2_122));
    }
}
