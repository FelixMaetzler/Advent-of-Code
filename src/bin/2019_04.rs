use std::iter;

all_aoc::solution!(4, 2019);

pub fn part_one(input: &str) -> Option<usize> {
    let (start, end) = parse(input);
    let start = 100_000.max(start);
    let end = 999_999.min(end);
    let range = start..=end;
    Some(range.into_iter().filter(|x| filter_part_one(*x)).count())
}
pub fn part_two(input: &str) -> Option<usize> {
    let (start, end) = parse(input);
    let start = 100_000.max(start);
    let end = 999_999.min(end);
    let range = start..=end;
    Some(range.into_iter().filter(|x| filter_part_two(*x)).count())
}
fn filter_part_one(x: u32) -> bool {
    adjacent_are_same(x) && not_decrease(x)
}
fn filter_part_two(x: u32) -> bool {
    adjacent_are_same_but_not_more(x) && not_decrease(x)
}
fn adjacent_are_same(x: u32) -> bool {
    let vec = x.to_string().chars().collect::<Vec<_>>();
    vec.windows(2).any(|w| w[0] == w[1])
}
fn adjacent_are_same_but_not_more(x: u32) -> bool {
    let s = x.to_string();
    let vec = iter::once('a')
        .chain(s.chars())
        .chain(iter::once('a'))
        .collect::<Vec<_>>();
    vec.windows(4)
        .any(|w| w[1] == w[2] && w[0] != w[1] && w[3] != w[2])
}
fn not_decrease(x: u32) -> bool {
    let vec = x.to_string().chars().collect::<Vec<_>>();
    vec.windows(2).all(|w| w[0] <= w[1])
}

fn parse(input: &str) -> (u32, u32) {
    let (p1, p2) = input.split_once('-').unwrap();
    (p1.parse().unwrap(), p2.parse().unwrap())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(921));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(603));
    }
    #[test]
    fn test() {
        assert!(adjacent_are_same_but_not_more(112_233));
        assert!(!adjacent_are_same_but_not_more(123_444));
        assert!(adjacent_are_same_but_not_more(111_122));
    }
}
