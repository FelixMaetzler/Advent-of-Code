use std::{collections::HashMap, fmt::Debug};

use all_aoc::helper::{
    graph::{WithWeights as _, build_graph4_special},
    grid::{Grid as _, dense::DenseGrid, index::GridIndex},
    permutations::IteratorCombinator as _,
};

all_aoc::solution!(20, 2024);
#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Start,
    End,
    Track,
    Wall,
}
impl TryFrom<char> for Tile {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Track),
            '#' => Ok(Self::Wall),
            'S' => Ok(Self::Start),
            'E' => Ok(Self::End),
            x => Err(x),
        }
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    execute(input, 100, 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    execute(input, 100, 20)
}
fn execute(input: &str, saving: u32, cheats_activated: u32) -> Option<u32> {
    let grid = parse(input);
    let (start, end) = start_end(&grid);
    let graph = build_graph4_special(&grid, |curr, n| {
        (*curr != Tile::Wall && *n != Tile::Wall).then_some(1)
    });
    let (dist_map, path) = graph.dijkstra_shortest_path(start, end);
    let length_without_cheats = dist_map[&end];
    let mut ret = HashMap::new();
    for v in path.into_iter().combinations(2) {
        let from = v[0];
        let to = v[1];
        if let Some(x) = is_cheatable(from, to, &grid, cheats_activated) {
            let length_to_from = dist_map[&from];
            let length_from_to_to_end = length_without_cheats - dist_map[&to];
            let new_len = length_from_to_to_end + x + length_to_from;
            let save = length_without_cheats.saturating_sub(new_len);
            if save > 0 {
                ret.entry(save).and_modify(|v| *v += 1).or_insert(1);
            }
        }
    }
    Some(
        ret.into_iter()
            .filter(|(k, _)| *k >= saving)
            .map(|(_, v)| v)
            .sum(),
    )
}
fn is_cheatable<T>(
    a: impl GridIndex<T>,
    b: impl GridIndex<T>,
    grid: &DenseGrid<T>,
    max_dist: u32,
) -> Option<u32>
where
    T: Clone + Debug,
{
    let a = a.to_coordinates(grid);
    let b = b.to_coordinates(grid);
    let dist = (a.0.abs_diff(b.0) + a.1.abs_diff(b.1)).try_into().unwrap();
    (2..=max_dist).contains(&dist).then_some(dist)
}
fn start_end(grid: &DenseGrid<Tile>) -> (usize, usize) {
    let start = grid
        .iter()
        .enumerate()
        .find(|(_, t)| **t == Tile::Start)
        .unwrap()
        .0;
    let end = grid
        .iter()
        .enumerate()
        .find(|(_, t)| **t == Tile::End)
        .unwrap()
        .0;
    (start, end)
}
fn parse(input: &str) -> DenseGrid<Tile> {
    DenseGrid::from_string(input)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = execute(&all_aoc::cli::read_examples_file(DAY), 8, 2);
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_417));
    }

    #[test]
    fn test_part_two() {
        let result = execute(&all_aoc::cli::read_examples_file(DAY), 50, 20);
        assert_eq!(result, Some(285));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_014_683));
    }
}
