use std::collections::HashSet;

use all_aoc::helper::{
    graph::{Graph, build_graph4},
    grid::{Grid, dense_grid::DenseGrid},
};

all_aoc::solution!(10, 2024);

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse(input);
    let graph = build_graph4(&grid, |curr, neigh| *neigh == curr + 1);
    let zeros = grid
        .iter()
        .enumerate()
        .filter(|(_, val)| **val == 0)
        .map(|(i, _)| i);
    Some(
        zeros
            .map(|i| all_9s_reachable(i, &graph, &grid).len())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse(input);
    let graph = build_graph4(&grid, |curr, neigh| *neigh == curr + 1);
    let zeros = grid
        .iter()
        .enumerate()
        .filter(|(_, val)| **val == 0)
        .map(|(i, _)| i);
    Some(zeros.map(|i| traversal_part_2(i, &graph, &grid)).sum())
}

fn all_9s_reachable(start: usize, graph: &[Vec<usize>], grid: &DenseGrid<u32>) -> HashSet<usize> {
    let mut queue = vec![start];
    let mut set = HashSet::new();
    while let Some(i) = queue.pop() {
        if grid[i] == 9 {
            set.insert(i);
        }
        for n in &graph[i] {
            queue.push(*n);
        }
    }
    set
}
fn traversal_part_2(start: usize, graph: &Vec<Vec<usize>>, grid: &DenseGrid<u32>) -> usize {
    let reachable = all_9s_reachable(start, graph, grid);
    reachable
        .into_iter()
        .map(|end| graph.all_paths(start, end).len())
        .sum()
}

fn parse(input: &str) -> DenseGrid<u32> {
    DenseGrid::from_iter_iter(
        input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap())),
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(587));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(81));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_340));
    }
}
