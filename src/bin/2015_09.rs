use std::collections::{HashMap, HashSet};

use all_aoc::helper::permutations::IteratorPermutator;

all_aoc::solution!(9, 2015);

pub fn part_one(input: &str) -> Option<u32> {
    let SetupResult(map, perm) = setup(input);
    perm.into_iter().map(|v| distance(&v, &map)).min()
}

pub fn part_two(input: &str) -> Option<u32> {
    let SetupResult(map, perm) = setup(input);
    perm.into_iter().map(|v| distance(&v, &map)).max()
}
struct SetupResult(HashMap<(String, String), u32>, Vec<Vec<String>>);

fn setup(input: &str) -> SetupResult {
    let map = parse(input);
    let vec = map
        .keys()
        .map(|(k, _)| k)
        .collect::<HashSet<_>>()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    let perm = vec.into_iter().permutation().collect();
    SetupResult(map, perm)
}
fn parse(input: &str) -> HashMap<(String, String), u32> {
    let mut map = HashMap::new();
    for line in input.lines() {
        let vec = line.split(' ').collect::<Vec<_>>();
        let s1 = vec[0].to_string();
        let s2 = vec[2].to_string();
        let x = vec[4].parse().unwrap();
        map.insert((s1.clone(), s2.clone()), x);
        map.insert((s2, s1), x);
    }

    map
}
fn distance(vec: &[String], map: &HashMap<(String, String), u32>) -> u32 {
    vec.windows(2)
        .map(|x| (&x[0], &x[1]))
        .map(|(s1, s2)| map.get(&(s1.clone(), s2.clone())).unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(605));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(141));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(982));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(736));
    }
}
