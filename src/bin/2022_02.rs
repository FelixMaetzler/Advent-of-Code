all_aoc::solution!(2, 2022);
fn score_part_1((opponent, me): (char, char)) -> u32 {
    match (opponent, me) {
        ('A', 'X') => 4,
        ('A', 'Y') => 8,
        ('A', 'Z') => 3,
        ('B', 'X') => 1,
        ('B', 'Y') => 5,
        ('B', 'Z') => 9,
        ('C', 'X') => 7,
        ('C', 'Y') => 2,
        ('C', 'Z') => 6,
        _ => unreachable!(),
    }
}
fn score_part_2((opponent, me): (char, char)) -> u32 {
    match (opponent, me) {
        ('A', 'X') => 3,
        ('A', 'Y') => 4,
        ('A', 'Z') => 8,
        ('B', 'X') => 1,
        ('B', 'Y') => 5,
        ('B', 'Z') => 9,
        ('C', 'X') => 2,
        ('C', 'Y') => 6,
        ('C', 'Z') => 7,
        _ => unreachable!(),
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    Some(parse(input).map(score_part_1).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(parse(input).map(score_part_2).sum())
}
fn parse(input: &str) -> impl Iterator<Item = (char, char)> {
    input
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(15));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(12_855));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(13_726));
    }
}
