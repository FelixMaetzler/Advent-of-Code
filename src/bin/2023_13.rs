use core::fmt::Debug;
use std::collections::HashSet;

use all_aoc::helper::grid::{Grid as _, dense::DenseGrid};

all_aoc::solution!(13, 2023);

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Rock,
    Ash,
}
impl core::ops::Not for Tile {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Rock => Self::Ash,
            Self::Ash => Self::Rock,
        }
    }
}
impl TryFrom<char> for Tile {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Ash),
            '#' => Ok(Self::Rock),
            val => Err(val),
        }
    }
}
impl Debug for Tile {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Rock => write!(f, "#"),
            Self::Ash => write!(f, "."),
        }
    }
}
pub fn part_one(input: &str) -> Option<usize> {
    let grids = parse(input);
    let mut sum_horizontal = 0;
    let mut sum_vertical = 0;
    for grid in grids {
        let erg = check_horizontal_symmetry(&grid);
        let erg = erg.first().unwrap_or(&0);
        sum_horizontal += erg;
        let erg = check_vertical_symmetry(&grid);
        let erg = erg.first().unwrap_or(&0);
        sum_vertical += erg;
    }
    Some(100 * sum_horizontal + sum_vertical)
}
fn check_horizontal_symmetry(grid: &DenseGrid<Tile>) -> Vec<usize> {
    let mut set = (1..grid.height()).collect();
    for i in 0..grid.width() {
        let row = grid.get_col(i);
        let erg = check_symmetry(&row.copied().collect::<Vec<_>>(), &set);
        let erg = erg.intersection(&set).copied().collect();
        set = erg;
        if set.is_empty() {
            return vec![];
        }
    }
    set.into_iter().collect()
}
fn check_vertical_symmetry(grid: &DenseGrid<Tile>) -> Vec<usize> {
    let mut set = (1..grid.width()).collect();
    for i in 0..grid.height() {
        let row = grid.get_row(i);
        let erg = check_symmetry(&row.copied().collect::<Vec<_>>(), &set);
        let erg = erg.intersection(&set).copied().collect();
        set = erg;
        if set.is_empty() {
            return vec![];
        }
    }
    set.into_iter().collect()
}
fn check_symmetry(line: &[Tile], options: &HashSet<usize>) -> HashSet<usize> {
    options
        .iter()
        .filter(|i| is_symmetic(line, **i))
        .copied()
        .collect()
}
fn is_symmetic(line: &[Tile], i: usize) -> bool {
    let left = line.iter().take(i).rev();
    let right = line.iter().skip(i);
    let mut zip = left.zip(right);
    zip.all(|(a, b)| a == b)
}
pub fn part_two(input: &str) -> Option<usize> {
    let grids = parse(input);
    let mut sum_horizontal = 0;
    let mut sum_vertical = 0;
    for grid in grids {
        let old = check_horizontal_symmetry(&grid);

        let old = *old.first().unwrap_or(&0);
        let mut clone = grid.clone();
        for i in 0..clone.len() {
            clone[i] = !clone[i];
            let erg = check_horizontal_symmetry(&clone);
            if !erg.is_empty()
                && let Some(x) = erg.into_iter().find(|i| i != &old)
            {
                sum_horizontal += x;
                clone[i] = !clone[i];
                break;
            }
            clone[i] = !clone[i];
        }
        let old = check_vertical_symmetry(&grid);
        let old = *old.first().unwrap_or(&0);
        let mut clone = grid.clone();
        for i in 0..clone.len() {
            clone[i] = !clone[i];
            let erg = check_vertical_symmetry(&clone);
            if !erg.is_empty()
                && let Some(x) = erg.into_iter().find(|i| i != &old)
            {
                sum_vertical += x;
                clone[i] = !clone[i];
                break;
            }
            clone[i] = !clone[i];
        }
    }
    Some(100 * sum_horizontal + sum_vertical)
}
fn parse(input: &str) -> Vec<DenseGrid<Tile>> {
    input
        .trim()
        .split("\n\n")
        .map(DenseGrid::from_string)
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(33_122));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(400));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(32_312));
    }
    #[test]
    fn test_part_one_1() {
        let result = part_one(
            "
.###....#.###..
...#.##...#.#..
.#.#.#.......##
#######..#..#..
#####.###...#..
##.##......#.##
###.#.##.#...##
#####.#.###....
###..##.####...
....#...#....##
...#.######.#..
.#..#...#.###..
.###.####..#...
..#####........
####...#.#..###
####...#.#..###
..###.#........",
        );
        assert_eq!(result, Some(14));
    }
    #[test]
    fn test_part_two_1() {
        let result = part_two(
            "
.###....#.###..
...#.##...#.#..
.#.#.#.......##
#######..#..#..
#####.###...#..
##.##......#.##
###.#.##.#...##
#####.#.###....
###..##.####...
....#...#....##
...#.######.#..
.#..#...#.###..
.###.####..#...
..#####........
####...#.#..###
####...#.#..###
..###.#........",
        );
        assert_eq!(result, Some(1500));
    }
    #[test]
    fn test_part_two_2() {
        // (3, 2)
        let result = part_two(
            "
..#.##.#..#.###
#.######.###.##
...........##..
...#..##....###
#........##..##
..#....#...#...
##.####.##...##
.#.####.#.#....
.########....##
###....###...##
...........#...
###########.#..
##########.##..",
        );
        assert_eq!(result, Some(5));
    }
}
