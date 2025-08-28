use core::fmt::Debug;

use all_aoc::helper::{
    graph::{WithWeights as _, build_graph4_special},
    grid::{dense::DenseGrid, index::GridIndex as _},
    position::Position,
};

all_aoc::solution!(13, 2016);
#[derive(PartialEq, Clone, Copy, Debug)]
enum Tile {
    Space,
    Wall,
}

const fn calc_tile(x: usize, y: usize, offset: usize) -> Tile {
    let num = x * x + 3 * x + 2 * x * y + y + y * y + offset;
    if num.count_ones().is_multiple_of(2) {
        Tile::Space
    } else {
        Tile::Wall
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let offset = input.parse().unwrap();
    shortest_path(Position { x: 31, y: 39 }, offset)
}

pub fn part_two(input: &str) -> Option<usize> {
    let offset = input.parse().unwrap();
    let grid =
        DenseGrid::from_iter_iter((0..=50).map(|y| (0..=50).map(move |x| calc_tile(x, y, offset))));
    let graph = build_graph4_special(&grid, |curr, nei| {
        (curr == nei && matches!(curr, Tile::Space)).then_some(1)
    });

    let start = Position { x: 1, y: 1 }.to_flat_index(&grid);
    let map = graph.dijkstra_distances(start, None);
    Some(map.into_values().filter(|d| *d <= 50).count())
}
fn shortest_path(goal: Position<usize>, offset: usize) -> Option<u32> {
    let mut grid_size = goal;
    loop {
        let grid = DenseGrid::from_iter_iter(
            (0..=grid_size.y).map(|y| (0..=grid_size.x).map(move |x| calc_tile(x, y, offset))),
        );
        let graph = build_graph4_special(&grid, |curr, nei| {
            (curr == nei && matches!(curr, Tile::Space)).then_some(1)
        });

        let start = Position { x: 1, y: 1 }.to_flat_index(&grid);
        let end = goal.to_flat_index(&grid);
        let (map, _) = graph.dijkstra_shortest_path(start, end);
        if let Some(ret) = map.get(&end) {
            return Some(*ret);
        }
        grid_size *= 2;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = shortest_path(Position { x: 7, y: 4 }, 10);
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(96));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(141));
    }
}
