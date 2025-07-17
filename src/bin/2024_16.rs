use std::collections::{HashMap, HashSet};

use all_aoc::helper::{
    graph::{Graph, Special, WithWeights},
    grid::{Grid, dense::DenseGrid, index::GridIndex},
    position::Direction4,
};

all_aoc::solution!(16, 2024);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Start,
    End,
    Wall,
    Empty,
}
impl TryFrom<char> for Tile {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            '#' => Ok(Self::Wall),
            'S' => Ok(Self::Start),
            'E' => Ok(Self::End),
            x => Err(x),
        }
    }
}
const fn map(node: &(usize, Direction4)) -> usize {
    4 * node.0
        + match node.1 {
            Direction4::North => 0,
            Direction4::East => 1,
            Direction4::West => 2,
            Direction4::South => 3,
        }
}
fn setup(grid: &DenseGrid<Tile>) -> Special<u32> {
    let mut graph = Special::new();
    for i in 0..grid.len() {
        if *grid.get(i).unwrap() == Tile::Wall {
            continue;
        }
        for dir in Direction4::all_dirs() {
            let x = map(&(i, dir));
            if let Some((new_index, new_tile)) = grid.get_dir8(i, dir.into()) {
                if *new_tile != Tile::Wall {
                    let new_index = new_index.to_flat_index(grid);
                    let y = map(&(new_index, dir));
                    graph.add_edge(x, y, 1);
                }
            }
            let l = map(&(i, dir.turn_left()));
            let r = map(&(i, dir.turn_right()));
            graph.add_edge(x, l, 1000);
            graph.add_edge(x, r, 1000);
        }
    }
    graph
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
fn solve(graph: &Special<u32>, grid: &DenseGrid<Tile>) -> u32 {
    let (start_grid, end_grid) = start_end(grid);
    let start_graph = map(&(start_grid, Direction4::East));
    let from_start = graph.dijkstra_distances(start_graph, None);
    *Direction4::all_dirs()
        .into_iter()
        .map(|d| map(&(end_grid, d)))
        .map(|i| &from_start[&i])
        .min()
        .unwrap()
}
pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse(input);
    let graph = setup(&grid);
    Some(solve(&graph, &grid))
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse(input);
    let graph = setup(&grid);
    let ret = solve(&graph, &grid);
    let (start_grid, end_grid) = start_end(&grid);
    let start_graph = map(&(start_grid, Direction4::East));
    let from_start = graph.dijkstra_distances(start_graph, None);
    let from_ends =
        Direction4::all_dirs().map(|dir| graph.dijkstra_distances(map(&(end_grid, dir)), None));
    let mut from_end = HashMap::new();
    from_ends.into_iter().flatten().for_each(|(k, v)| {
        from_end
            .entry(k)
            .and_modify(|val| *val = v.min(*val))
            .or_insert(v);
    });
    let mut set = HashSet::new();
    for i in 0..grid.len() {
        for d in Direction4::all_dirs() {
            let index_forward = map(&(i, d));
            let index_backward = map(&(i, d.opposite()));
            if let Some(starting) = from_start.get(&index_forward) {
                if let Some(ending) = from_end.get(&index_backward) {
                    if ret == starting + ending {
                        set.insert(i);
                    }
                }
            }
        }
    }
    Some(set.len())
}
fn parse(input: &str) -> DenseGrid<Tile> {
    DenseGrid::from_string(input)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(11_048));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(111_480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(64));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(529));
    }
}
