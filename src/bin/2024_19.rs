use std::collections::HashMap;

all_aoc::solution!(19, 2024);

pub fn part_one(input: &str) -> Option<usize> {
    let (towels, designs) = parse(input);
    let mut cache = HashMap::new();
    Some(
        designs
            .into_iter()
            .filter(|design| check(design, &towels, &mut cache))
            .count(),
    )
}
pub fn part_two(input: &str) -> Option<u64> {
    let (towels, designs) = parse(input);
    let mut cache = HashMap::new();
    Some(
        designs
            .into_iter()
            .map(|design| arrangements(&design, &towels, &mut cache))
            .sum(),
    )
}
fn check(design: &str, towels: &[String], cache: &mut HashMap<String, bool>) -> bool {
    if design.is_empty() {
        return true;
    }
    if let Some(&x) = cache.get(design) {
        return x;
    }

    for t in towels {
        if let Some(x) = design.strip_prefix(t)
            && check(x, towels, cache)
        {
            cache.insert(design.to_owned(), true);
            return true;
        }
    }
    cache.insert(design.to_owned(), false);
    false
}
fn arrangements(design: &str, towels: &[String], cache: &mut HashMap<String, u64>) -> u64 {
    if design.is_empty() {
        return 1;
    }
    if let Some(&x) = cache.get(design) {
        return x;
    }

    let sum = towels
        .iter()
        .filter_map(|t| design.strip_prefix(t))
        .map(|rest| arrangements(rest, towels, cache))
        .sum();
    cache.insert(design.to_owned(), sum);
    sum
}

fn parse(input: &str) -> (Vec<String>, Vec<String>) {
    let (towels, designs) = input.split_once("\n\n").unwrap();
    let towels = towels.split(", ").map(|l| l.parse().unwrap()).collect();
    let designs = designs.lines().map(|l| l.parse().unwrap()).collect();
    (towels, designs)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(287));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(571_894_474_468_161));
    }
}
