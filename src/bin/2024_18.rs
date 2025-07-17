use all_aoc::helper::{
    graph::{WithWeights, build_graph4_special},
    grid::{Grid, dense::DenseGrid, index::GridIndex},
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
        grid[p.as_yx_tuple()] = Tile::Corrupted;
    });
    let graph = build_graph4_special(&grid, |curr, neig| {
        (*neig == Tile::Empty && *curr == Tile::Empty).then_some(1)
    });
    let end = grid.len() - 1;
    let erg = graph.dijkstra_distances(0, Some(end));
    Some(erg[&end])
}
fn solve_part_2(input: &str, size: usize) -> Option<String> {
    let vec = parse(input);
    let grid = DenseGrid::new(size, size, Tile::Empty);
    let mut graph = build_graph4_special(&grid, |_, _| Some(1));

    let end = grid.len() - 1;
    let (_, mut path) = graph.dijkstra_shortest_path(0, end);
    for p in vec {
        let i = p.as_yx_tuple().to_flat_index(&grid);
        let x = graph.remove_node(i);
        debug_assert!(x);
        if !path.contains(&i) {
            continue;
        }
        let (erg, pa) = graph.dijkstra_shortest_path(0, end);
        path = pa;
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
        assert_eq!(result, Some("6,1".to_owned()));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some("56,29".to_owned()));
    }
}
