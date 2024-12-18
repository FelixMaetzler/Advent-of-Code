use all_aoc::helper::{
    graph::{build_graph4, GraphWithWeights, SpecialGraph},
    grid::{dense_grid::DenseGrid, grid_index::GridIndex, Grid},
    position::Position,
};

all_aoc::solution!(18, 2024);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Corrupted,
}
pub fn part_one(input: &str) -> Option<u32> {
    solve_part_1(input, 71, 1_024)
}
fn solve_part_1(input: &str, size: usize, bytes: usize) -> Option<u32> {
    let vec = parse(input);
    let mut grid = DenseGrid::new(size, size, Tile::Empty);
    vec.into_iter().take(bytes).for_each(|p| {
        grid.set(p.as_yx_tuple(), Tile::Corrupted);
    });
    let grid = grid;
    let graph = build_graph4(&grid, |curr, nei| {
        *curr == Tile::Empty && *nei == Tile::Empty
    });
    let graph = SpecialGraph::from_edges(
        graph
            .into_iter()
            .enumerate()
            .flat_map(|(from, tos)| tos.into_iter().map(move |to| (from, to, 1_u32))),
    );
    let end = grid.len() - 1;
    let erg = graph.dijkstra(0);
    Some(*erg.get(&end).unwrap())
}
fn solve_part_2(input: &str, size: usize) -> Option<String> {
    let vec = parse(input);
    let grid = DenseGrid::new(size, size, Tile::Empty);
    let graph = build_graph4(&grid, |curr, nei| {
        *curr == Tile::Empty && *nei == Tile::Empty
    });
    let mut graph = SpecialGraph::from_edges(
        graph
            .into_iter()
            .enumerate()
            .flat_map(|(from, tos)| tos.into_iter().map(move |to| (from, to, 1_u32))),
    );
    let end = grid.len() - 1;
    for p in vec {
        let i = p.as_yx_tuple().to_flat_index(&grid);
        let x = graph.remove_node(i);
        debug_assert!(x);
        let erg = graph.dijkstra(0);
        if !erg.contains_key(&end) {
            return format!("{},{}", p.x, p.y).into();
        }
    }
    unreachable!()
}
pub fn part_two(input: &str) -> Option<String> {
    solve_part_2(input, 71)
}
fn parse(input: &str) -> Vec<Position<usize>> {
    input
        .lines()
        .map(|l| Position::from_xy(l).unwrap())
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_1(&all_aoc::cli::read_examples_file(DAY), 7, 12);
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(318));
    }

    #[test]
    fn test_part_two() {
        let result = solve_part_2(&all_aoc::cli::read_examples_file(DAY), 7);
        assert_eq!(result, Some("6,1".to_string()));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some("56,29".to_string()));
    }
}
