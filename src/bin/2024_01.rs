use std::collections::HashMap;

all_aoc::solution!(1, 2024);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut v1, mut v2) = parse(input);
    v1.sort();
    v2.sort();
    let (v1, v2) = (v1, v2);
    Some(v1.into_iter().zip(v2).map(|(x1, x2)| x1.abs_diff(x2)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (v1, v2) = parse(input);
    let mut map = HashMap::new();
    v2.iter().for_each(|v| {
        map.entry(v).and_modify(|x| *x += 1).or_insert(1);
    });
    let map = map;
    Some(
        v1.into_iter()
            .map(|x| {
                x * match map.get(&x) {
                    Some(y) => *y,
                    None => 0,
                }
            })
            .sum(),
    )
}
fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    let vec1 = input
        .lines()
        .map(|l| l.split_ascii_whitespace().next().unwrap().parse().unwrap())
        .collect();
    let vec2 = input
        .lines()
        .map(|l| l.split_ascii_whitespace().nth(1).unwrap().parse().unwrap())
        .collect();
    (vec1, vec2)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(2_970_687));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(31));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(23_963_899));
    }
}
