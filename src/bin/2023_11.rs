use core::fmt::Debug;
use std::collections::HashMap;

use all_aoc::helper::grid::{Grid as _, dense::DenseGrid, index::GridIndex as _};

all_aoc::solution!(11, 2023);

#[derive(Clone, PartialEq, Copy)]
enum Tile {
    Space,
    Galaxy,
}
impl TryFrom<char> for Tile {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Space),
            '#' => Ok(Self::Galaxy),
            ch => Err(ch),
        }
    }
}
impl Debug for Tile {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Space => write!(f, "."),
            Self::Galaxy => write!(f, "#"),
        }
    }
}
pub fn part_one(input: &str) -> Option<usize> {
    solve(input, 2)
}
const fn distance(n: ((usize, usize), (usize, usize))) -> usize {
    let (x, y) = n;
    x.0.abs_diff(y.0) + x.1.abs_diff(y.1)
}
fn build_combinations(vec: &[(usize, usize)]) -> Vec<((usize, usize), (usize, usize))> {
    (0..vec.len() - 1)
        .flat_map(|i| (i + 1..vec.len()).map(move |j| (vec[i], vec[j])))
        .collect()
}
pub fn part_two(input: &str) -> Option<usize> {
    solve(input, 1_000_000)
}
fn solve(input: &str, times: usize) -> Option<usize> {
    let grid = parse(input);

    let empty_rows = (0..grid.height())
        .filter(|&y| grid.get_row(y).all(|t| *t == Tile::Space))
        .collect::<Vec<_>>();
    let empty_cols = (0..grid.width())
        .filter(|&x| grid.get_col(x).all(|t| t == &Tile::Space))
        .collect::<Vec<_>>();

    let mut map_rows: HashMap<usize, usize> = (0..grid.height())
        .map(|n| (n, n))
        .collect::<HashMap<_, _>>();
    let mut map_cols: HashMap<usize, usize> =
        (0..grid.width()).map(|n| (n, n)).collect::<HashMap<_, _>>();
    for n in &empty_rows {
        map_rows
            .iter_mut()
            .filter(|(k, _)| k > &n)
            .for_each(|(_, v)| *v += times - 1);
    }
    for n in &empty_cols {
        map_cols
            .iter_mut()
            .filter(|(k, _)| k > &n)
            .for_each(|(_, v)| *v += times - 1);
    }
    let new_galaxies = grid
        .iter()
        .enumerate()
        .filter(|(_, t)| t == &&Tile::Galaxy)
        .map(|(n, _)| n.to_coordinates(&grid))
        .map(|(y, x)| (map_rows[&y], map_cols[&x]))
        .collect::<Vec<_>>();
    let combinations = build_combinations(&new_galaxies);
    Some(combinations.into_iter().map(distance).sum())
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
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(9_769_724));
    }

    #[test]
    fn test_part_two() {
        let result = solve(&all_aoc::cli::read_examples_file(DAY), 10);
        assert_eq!(result, Some(1_030));
        let result = solve(&all_aoc::cli::read_examples_file(DAY), 100);
        assert_eq!(result, Some(8_410));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(603_020_563_700));
    }
}
