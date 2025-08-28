use std::collections::HashMap;

use all_aoc::helper::{
    graph::{WithWeights as _, build_graph4_special},
    grid::{Grid as _, dense::DenseGrid},
    permutations::IteratorPermutator as _,
};

all_aoc::solution!(24, 2016);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Air,
    Number(u8),
}

impl TryFrom<char> for Tile {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::Wall),
            '.' => Ok(Self::Air),
            x @ '0'..='9' => Ok(Self::Number(x.to_string().parse().unwrap())),
            x => Err(x),
        }
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    solve(input, calc_distance_part_1)
}
pub fn part_two(input: &str) -> Option<u32> {
    solve(input, calc_distance_part_2)
}
fn solve(
    input: &str,
    func: impl Fn(&HashMap<(usize, usize), u32>, Vec<usize>) -> u32,
) -> Option<u32> {
    let grid = parse(input);
    let highest_number = grid.iter().filter(|t| matches!(t, Tile::Number(_))).count() - 1;
    let pos = (0..=highest_number)
        .map(|n| {
            grid.iter()
                .enumerate()
                .find(|(_, t)| **t == Tile::Number(n.try_into().unwrap()))
                .unwrap()
                .0
        })
        .collect::<Vec<_>>();
    let graph = build_graph4_special(&grid, |curr, neig| {
        (*curr != Tile::Wall && *neig != Tile::Wall).then_some(1)
    });
    let distances = pos
        .iter()
        .map(|n| graph.dijkstra_distances(*n, None))
        .collect::<Vec<_>>();
    let distances = distances
        .iter()
        .enumerate()
        .flat_map(|(from, map)| {
            pos.iter()
                .enumerate()
                .map(move |(i, to)| ((from, i), map[to]))
        })
        .collect::<HashMap<_, _>>();
    let erg = (1..=highest_number)
        .permutation()
        .map(|perm| (func(&distances, perm.clone()), perm))
        .min()
        .unwrap();
    Some(erg.0)
}
fn calc_distance_part_1(distances: &HashMap<(usize, usize), u32>, mut perm: Vec<usize>) -> u32 {
    perm.insert(0, 0);
    perm.windows(2).map(|w| &distances[&(w[0], w[1])]).sum()
}
fn calc_distance_part_2(distances: &HashMap<(usize, usize), u32>, mut perm: Vec<usize>) -> u32 {
    perm.insert(0, 0);
    perm.push(0);
    perm.windows(2).map(|w| &distances[&(w[0], w[1])]).sum()
}

fn parse(input: &str) -> DenseGrid<Tile> {
    DenseGrid::from_iter_iter(
        input
            .lines()
            .map(|l| l.chars().map(|c| c.try_into().unwrap())),
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(470));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(20));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(720));
    }
}
