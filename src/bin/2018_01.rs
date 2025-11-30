use std::collections::HashSet;

all_aoc::solution!(1, 2018);

pub fn part_one(input: &str) -> Option<i32> {
    Some(parse(input).sum())
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut set = HashSet::new();
    let mut curr = 0;
    for i in parse(input).cycle() {
        curr += i;
        if !set.insert(curr) {
            return Some(curr);
        }
    }
    unreachable!()
}
fn parse(input: &str) -> impl Iterator<Item = i32> + Clone {
    input.lines().map(|n| n.parse().unwrap())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(520));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(394));
    }
}
