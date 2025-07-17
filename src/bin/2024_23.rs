use std::collections::{HashMap, HashSet};

use all_aoc::helper::{graph::Graph as _, permutations::IteratorCombinator as _};

all_aoc::solution!(23, 2024);

pub fn part_one(input: &str) -> Option<usize> {
    let vec = parse(input);
    let mut set = HashSet::new();
    for (x, y) in &vec {
        set.insert(x);
        set.insert(y);
    }
    let mut map = HashMap::new();
    let mut rev_map = HashMap::new();
    set.into_iter().enumerate().for_each(|(i, x)| {
        map.insert(x, i);
        rev_map.insert(i, x);
    });
    let mut g = vec![vec![]; map.len()];
    for (x, y) in &vec {
        let x = map[x];
        let y = map[y];
        g[x].push(y);
        g[y].push(x);
    }
    Some(
        (0..map.len())
            .combinations(3)
            .filter(|v| v.iter().combinations(2).all(|w| g[*w[0]].contains(w[1])))
            .filter(|v| v.iter().any(|i| rev_map[i].starts_with('t')))
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<String> {
    let vec = parse(input);
    let mut set = HashSet::new();
    for (x, y) in &vec {
        set.insert(x);
        set.insert(y);
    }
    let mut map = HashMap::new();
    let mut rev_map = HashMap::new();
    set.into_iter().enumerate().for_each(|(i, x)| {
        map.insert(x, i);
        rev_map.insert(i, x);
    });
    let mut g = vec![vec![]; map.len()];
    for (x, y) in &vec {
        let x = map[x];
        let y = map[y];
        g[x].push(y);
        g[y].push(x);
    }

    let mut c = g.bron_kerbosch1();
    c.sort_by_key(std::collections::HashSet::len);

    let c = c.last().unwrap();
    let mut c = c
        .iter()
        .map(|i| (*rev_map[i]).to_owned())
        .collect::<Vec<_>>();
    c.sort();
    Some(c.join(","))
}
fn parse(input: &str) -> Vec<(&str, &str)> {
    input.lines().map(|l| l.split_once('-').unwrap()).collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_098));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_owned()));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(
            result,
            Some("ar,ep,ih,ju,jx,le,ol,pk,pm,pp,xf,yu,zg".to_owned())
        );
    }
}
