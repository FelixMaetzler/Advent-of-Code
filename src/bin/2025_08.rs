use core::cmp::Reverse;
use std::collections::BinaryHeap;

use all_aoc::helper::{
    dsu::DisjointUnionSets, permutations::IteratorCombinator as _, position3d::Position3d,
};

all_aoc::solution!(8, 2025);

pub fn part_one(input: &str) -> Option<usize> {
    solve_part_1(input, 1_000)
}
fn solve_part_1(input: &str, iterations: usize) -> Option<usize> {
    let vec = parse(input).collect::<Vec<_>>();
    let mut heap = (0..vec.len())
        .combinations(2)
        .map(|c| {
            (
                Reverse(
                    (vec[c[0]].abs_diff(&vec[c[1]]))
                        .cast::<u128>()
                        .euclidean_norm_squared(),
                ),
                c,
            )
        })
        .collect::<BinaryHeap<_>>();
    let mut circuits = DisjointUnionSets::new(vec.len());
    for _ in 0..iterations {
        let min = heap.pop().unwrap().1;
        let v1 = min[0];
        let v2 = min[1];
        circuits.union_sets(v1, v2);
    }
    let mut vec = circuits
        .groups()
        .into_iter()
        .map(|s| s.len())
        .collect::<Vec<_>>();
    vec.sort_unstable();
    Some(vec.into_iter().rev().take(3).product())
}

pub fn part_two(input: &str) -> Option<u32> {
    let vec = parse(input).collect::<Vec<_>>();
    let mut heap = (0..vec.len())
        .combinations(2)
        .map(|c| {
            (
                Reverse(
                    (vec[c[0]].abs_diff(&vec[c[1]]))
                        .cast::<u128>()
                        .euclidean_norm_squared(),
                ),
                c,
            )
        })
        .collect::<BinaryHeap<_>>();
    let mut circuits = DisjointUnionSets::new(vec.len());
    while let Some((_, min)) = heap.pop() {
        let v1 = min[0];
        let v2 = min[1];
        circuits.union_sets(v1, v2);
        if circuits.is_all_unified() {
            return Some(vec[v1].x * vec[v2].x);
        }
    }
    unreachable!()
}
fn parse(input: &str) -> impl Iterator<Item = Position3d<u32>> {
    input
        .lines()
        .map(|line| Position3d::from_it(line.split(',').map(|n| n.parse().unwrap())).unwrap())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_1(&all_aoc::cli::read_examples_file(DAY), 10);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(112_230));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(25_272));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(2_573_952_864));
    }
}
