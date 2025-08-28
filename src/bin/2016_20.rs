use core::ops::RangeInclusive;

use all_aoc::helper::range::Ext as _;

all_aoc::solution!(20, 2016);
#[derive(Debug)]
struct Allowed(Vec<RangeInclusive<u32>>);
impl Allowed {
    fn new(range: RangeInclusive<u32>) -> Self {
        Self(vec![range])
    }
    fn substract(&mut self, range: RangeInclusive<u32>) {
        let mut new = vec![];
        for allowed in &mut self.0 {
            let mut x = allowed.subtract(&range);
            new.append(&mut x);
        }
        self.0 = new;
    }
    fn give_all_numbers(self) -> impl Iterator<Item = u32> {
        self.0
            .into_iter()
            .flat_map(core::iter::IntoIterator::into_iter)
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let blocked = parse(input);
    let mut v = Allowed::new(0..=u32::MAX);
    for b in blocked {
        v.substract(b);
    }
    Some(v.give_all_numbers().next().unwrap())
}

pub fn part_two(input: &str) -> Option<usize> {
    let blocked = parse(input);
    let mut v = Allowed::new(0..=u32::MAX);
    for b in blocked {
        v.substract(b);
    }
    Some(v.give_all_numbers().count())
}
fn parse(input: &str) -> Vec<RangeInclusive<u32>> {
    input
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .map(|(first, second)| first.parse().unwrap()..=second.parse().unwrap())
        .collect()
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
        assert_eq!(result, Some(17_348_574));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(104));
    }
}
