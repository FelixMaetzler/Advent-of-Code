use std::collections::HashSet;

use all_aoc::helper::position::{Direction4, Position};

all_aoc::solution!(3, 2015);

pub fn part_one(input: &str) -> Option<usize> {
    let mut curr = Position::default();
    let mut set = HashSet::with_capacity(input.chars().count());
    set.insert(curr);
    for c in input.chars() {
        let d = Direction4::from_hat(c).unwrap();
        curr = curr.direction(d);
        set.insert(curr);
    }
    Some(set.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut santa = Position::default();
    let mut robo = Position::default();
    let mut set = HashSet::with_capacity(input.chars().count());
    set.insert(santa);
    for (i, c) in input.char_indices() {
        let d = Direction4::from_hat(c).unwrap();
        match i % 2 {
            0 => {
                santa = santa.direction(d);
                set.insert(santa);
            }
            1 => {
                robo = robo.direction(d);
                set.insert(robo);
            }
            _ => unreachable!("Mathematically impossible"),
        }
    }
    Some(set.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(Some(2), part_one(">"));
        assert_eq!(Some(4), part_one("^>v<"));
        assert_eq!(Some(2), part_one("^v^v^v^v^v"));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(2_565));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Some(3), part_two("^v"));
        assert_eq!(Some(3), part_two("^>v<"));
        assert_eq!(Some(11), part_two("^v^v^v^v^v"));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(2_639));
    }
}
