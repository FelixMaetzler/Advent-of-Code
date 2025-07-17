use std::collections::HashSet;

all_aoc::solution!(5, 2024);

pub fn part_one(input: &str) -> Option<u32> {
    let (page_ordering, updates) = parse(input);
    Some(
        updates
            .into_iter()
            .filter_map(|update| is_right_order(&page_ordering, &update))
            .sum(),
    )
}
fn is_right_order(page_ordering: &HashSet<(u32, u32)>, update: &[u32]) -> Option<u32> {
    for i in 0..update.len() - 1 {
        for j in i + 1..update.len() {
            let wrong_pair = (update[j], update[i]);
            if page_ordering.contains(&wrong_pair) {
                return None;
            }
        }
    }
    Some(update[update.len() / 2])
}
fn ordered(page_ordering: &HashSet<(u32, u32)>, update: &mut [u32]) -> Option<u32> {
    let mut changed = true;
    let mut first = true;
    while changed {
        changed = false;
        for i in 0..update.len() - 1 {
            for j in i + 1..update.len() {
                let wrong_pair = (update[j], update[i]);
                if page_ordering.contains(&wrong_pair) {
                    update.swap(i, j);
                    changed = true;
                    first = false;
                }
            }
        }
    }
    if first {
        None
    } else {
        Some(update[update.len() / 2])
    }
}
pub fn part_two(input: &str) -> Option<u32> {
    let (page_ordering, updates) = parse(input);
    Some(
        updates
            .into_iter()
            .filter_map(|mut update| ordered(&page_ordering, &mut update))
            .sum(),
    )
}
fn parse(input: &str) -> (HashSet<(u32, u32)>, Vec<Vec<u32>>) {
    let (page_ordering, updates) = input.split_once("\n\n").unwrap();
    let page_ordering = page_ordering
        .lines()
        .map(|l| l.split_once('|').unwrap())
        .map(|(s1, s2)| (s1.parse().unwrap(), s2.parse().unwrap()))
        .collect();

    let updates = updates
        .lines()
        .map(|l| l.split(',').map(|i| i.parse().unwrap()).collect())
        .collect();
    (page_ordering, updates)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(6_949));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(123));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(4_145));
    }
}
