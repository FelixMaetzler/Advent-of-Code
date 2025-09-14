use all_aoc::helper::{
    graph::{WithWeights as _, build_graph4_special},
    grid::{Grid as _, dense::DenseGrid},
};

all_aoc::solution!(12, 2022);
#[derive(Debug, Clone, Copy)]
enum Tile {
    Number(u8),
    Start,
    End,
}
impl Tile {
    const fn get_elevation(self) -> u8 {
        match self {
            Self::Number(x) => x,
            Self::Start => b'a',
            Self::End => b'z',
        }
    }
}
impl TryFrom<char> for Tile {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Self::Start),
            'E' => Ok(Self::End),
            x => Ok(Self::Number(x as u8)),
        }
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse(input);
    solve(
        &grid,
        &[grid
            .iter()
            .enumerate()
            .find(|(_, t)| matches!(t, Tile::Start))
            .unwrap()
            .0],
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse(input);
    solve(
        &grid,
        &grid
            .iter()
            .enumerate()
            .filter(|(_, t)| t.get_elevation() == Tile::Start.get_elevation())
            .map(|(i, _)| i)
            .collect::<Vec<_>>(),
    )
}
fn solve(grid: &DenseGrid<Tile>, starts: &[usize]) -> Option<u32> {
    let graph = build_graph4_special(grid, |curr, neigh| {
        (neigh.get_elevation() <= curr.get_elevation() + 1).then_some(1)
    });
    let end = grid
        .iter()
        .enumerate()
        .find(|(_, t)| matches!(t, Tile::End))
        .unwrap()
        .0;
    Some(
        starts
            .iter()
            .map(|start| graph.dijkstra_distances(*start, Some(end)))
            .filter_map(|m| m.get(&end).copied())
            .min()
            .unwrap(),
    )
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
        assert_eq!(result, Some(31));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(468));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(29));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(459));
    }
}
