use std::collections::{HashMap, hash_map::Entry};

use all_aoc::helper::md5;

all_aoc::solution!(14, 2016);

pub fn part_one(input: &str) -> Option<usize> {
    let test = (0..)
        .filter_map(|index| three_chars_part1(input, index))
        .filter(|(index, ch)| five_chars_part1(input, *index, *ch))
        .nth(63)
        .unwrap();
    Some(test.0)
}
fn three_chars_part1(salt: &str, index: usize) -> Option<(usize, char)> {
    let input = md5::md5(&(salt.to_owned() + &index.to_string()))
        .chars()
        .collect::<Vec<_>>();
    for (c1, c2, c3) in input.windows(3).map(|w| (w[0], w[1], w[2])) {
        if c1 == c2 && c2 == c3 {
            return Some((index, c1));
        }
    }
    None
}
fn five_chars_part1(salt: &str, index: usize, ch: char) -> bool {
    for i in (index + 1)..(index + 1000) {
        let input = md5::md5(&(salt.to_owned() + &i.to_string()))
            .chars()
            .collect::<Vec<_>>();
        for (c1, c2, c3, c4, c5) in input.windows(5).map(|w| (w[0], w[1], w[2], w[3], w[4])) {
            if ch == c1 && c1 == c2 && c2 == c3 && c3 == c4 && c4 == c5 {
                return true;
            }
        }
    }
    false
}
pub fn part_two(input: &str) -> Option<usize> {
    let mut map = HashMap::new();
    let test = (0..usize::MAX)
        .filter_map(|index| three_chars_part2(input, index))
        .filter(|(index, ch)| five_chars_part2(input, *index, *ch, &mut map))
        .nth(63)
        .unwrap();
    Some(test.0)
}
fn three_chars_part2(salt: &str, index: usize) -> Option<(usize, char)> {
    let first = md5::md5(&(salt.to_owned() + &index.to_string()));
    let mut input = first;
    for _ in 0..2016 {
        input = md5::md5(&input);
    }
    let input = input.chars().collect::<Vec<_>>();
    for (c1, c2, c3) in input.windows(3).map(|w| (w[0], w[1], w[2])) {
        if c1 == c2 && c2 == c3 {
            return Some((index, c1));
        }
    }
    None
}
fn five_chars_part2(salt: &str, index: usize, ch: char, map: &mut HashMap<usize, String>) -> bool {
    for i in (index + 1)..(index + 1000) {
        let first = md5::md5(&(salt.to_owned() + &i.to_string()));

        let input = if let Entry::Vacant(e) = map.entry(i) {
            let mut input = first;
            for _ in 0..2016 {
                input = md5::md5(&input);
            }
            e.insert(input.clone());
            input
        } else {
            map.get(&i).unwrap().to_owned()
        };
        let input = input.chars().collect::<Vec<_>>();
        for (c1, c2, c3, c4, c5) in input.windows(5).map(|w| (w[0], w[1], w[2], w[3], w[4])) {
            if ch == c1 && c1 == c2 && c2 == c3 && c3 == c4 && c4 == c5 {
                return true;
            }
        }
    }
    false
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(18626));
    }
    #[cfg(feature = "expensive")]
    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(20092));
    }
}
