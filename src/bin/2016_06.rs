use std::collections::HashMap;

all_aoc::solution!(6, 2016);
enum Part {
    One,
    Two,
}
pub fn part_one(input: &str) -> Option<String> {
    solve(input, Part::One)
}

pub fn part_two(input: &str) -> Option<String> {
    solve(input, Part::Two)
}
fn solve(input: &str, part: Part) -> Option<String> {
    let len = input.lines().next().unwrap().len();

    let mut maps = vec![HashMap::new(); len];
    for l in input.lines() {
        for (i, c) in l.char_indices() {
            maps[i].entry(c).and_modify(|v| *v += 1).or_insert(1);
        }
    }
    let s = maps
        .into_iter()
        .map(|m| {
            *m.iter().max_by_key(|s| match part {
            Part::One => 1,
            Part::Two => -1,
        } * s.1).unwrap().0
        })
        .collect();
    Some(s)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some("easter".to_string()));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some("xdkzukcf".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some("advent".to_string()));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some("cevsgyvd".to_string()));
    }
}
