use core::str::FromStr;

all_aoc::solution!(4, 2023);

#[derive(Debug)]
struct ScratchCard {
    _id: usize,
    winning_numbers: Vec<usize>,
    numbers_i_have: Vec<usize>,
}
impl FromStr for ScratchCard {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(": ").unwrap();
        let (_, n) = left.split_once(' ').unwrap();
        let id = n.trim().parse().unwrap();
        let (left, right) = right.split_once(" | ").unwrap();
        let winning_numbers = left
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let mut numbers_i_have: Vec<_> = right
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        numbers_i_have.sort_unstable();
        Ok(Self {
            _id: id,
            winning_numbers,
            numbers_i_have,
        })
    }
}
impl ScratchCard {
    fn points(&self) -> usize {
        let n = self.count_winning_numbers();
        match n {
            0 => 0,
            _ => 1 << (n - 1),
        }
    }
    fn count_winning_numbers(&self) -> usize {
        self.winning_numbers
            .iter()
            .filter_map(|w| self.numbers_i_have.binary_search(w).ok())
            .count()
    }
}
pub fn part_one(input: &str) -> Option<usize> {
    let vec = parse(input);
    Some(vec.into_iter().map(|s| s.points()).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let vec = parse(input);
    let matching_numbers: Vec<_> = vec.iter().map(ScratchCard::count_winning_numbers).collect();
    let mut winning = vec![1; vec.len()];
    for (i, x) in matching_numbers.into_iter().enumerate() {
        for j in 1..=x {
            winning[i + j] += winning[i];
        }
    }
    Some(winning.iter().sum())
}
fn parse(input: &str) -> Vec<ScratchCard> {
    input
        .trim()
        .lines()
        .map(|l| ScratchCard::from_str(l).unwrap())
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(20_107));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(30));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(8_172_507));
    }
}
