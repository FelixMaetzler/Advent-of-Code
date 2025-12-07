use std::collections::{HashMap, HashSet};

use all_aoc::helper::{
    grid::{Grid as _, dense::DenseGrid, index::GridIndex as _},
    position::Direction8,
};

all_aoc::solution!(7, 2025);
#[derive(Debug, Clone, Copy)]
enum Tile {
    Air,
    Spliter,
    Start,
}
impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Air,
            '^' => Self::Spliter,
            'S' => Self::Start,
            _ => unreachable!(),
        }
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse(input);
    let start = grid
        .get_row(0)
        .enumerate()
        .find(|(_, x)| matches!(x, Tile::Start))
        .unwrap()
        .0;
    let mut set = HashSet::new();
    set.insert(start);
    let mut cnt = 0;
    for row in 1..grid.height() {
        let mut new_set = HashSet::new();
        for col in set {
            match grid[(row, col)] {
                Tile::Air => {
                    new_set.insert(col);
                }
                Tile::Spliter => {
                    for dir in [Direction8::West, Direction8::East] {
                        let (index, tile) = grid.get_dir8((row, col), dir).unwrap();
                        debug_assert!(matches!(tile, Tile::Air));
                        new_set.insert(index.1);
                    }
                    cnt += 1;
                }
                Tile::Start => unreachable!(),
            }
        }
        set = new_set;
    }
    Some(cnt)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = parse(input);
    let start = grid
        .get_row(0)
        .enumerate()
        .find(|(_, x)| matches!(x, Tile::Start))
        .unwrap()
        .0;
    Some(solve_recursive(&grid, start, &mut HashMap::new()))
}
fn solve_recursive(grid: &DenseGrid<Tile>, pos: usize, cache: &mut HashMap<usize, u64>) -> u64 {
    if let Some(x) = cache.get(&pos) {
        return *x;
    }
    let erg = match grid[pos] {
        Tile::Air | Tile::Start => pos
            .dir(Direction8::South, grid)
            .map_or(1, |x| solve_recursive(grid, x.to_flat_index(grid), cache)),
        Tile::Spliter => {
            solve_recursive(
                grid,
                pos.dir(Direction8::West, grid).unwrap().to_flat_index(grid),
                cache,
            ) + solve_recursive(
                grid,
                pos.dir(Direction8::East, grid).unwrap().to_flat_index(grid),
                cache,
            )
        }
    };
    cache.insert(pos, erg);
    erg
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
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_630));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(47_857_642_990_160));
    }
}
