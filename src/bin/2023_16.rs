use std::collections::{HashSet, VecDeque};

use all_aoc::helper::{
    grid::{Grid as _, dense::DenseGrid, index::GridIndex as _},
    position::Direction4 as Dir4,
};

all_aoc::solution!(16, 2023);
#[derive(Debug, Clone, Copy)]
enum Tile {
    Space,
    Vertical,
    Horizontal,
    SouthWest,
    SouthEast,
}
impl TryFrom<char> for Tile {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Space),
            '-' => Ok(Self::Horizontal),
            '|' => Ok(Self::Vertical),
            '/' => Ok(Self::SouthEast),
            '\\' => Ok(Self::SouthWest),
            val => Err(val),
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse(input);
    let start = (0, Dir4::East);
    solve(&grid, start)
}
fn solve(grid: &DenseGrid<Tile>, start: (usize, Dir4)) -> Option<usize> {
    let mut queue = VecDeque::from(vec![start]);
    let mut visited = HashSet::new();
    while let Some(curr) = queue.pop_front() {
        if visited.contains(&curr) {
            continue;
        }
        visited.insert(curr);
        let tile = grid[curr.0];
        let next = match (tile, curr.1) {
            (Tile::Space, dir) => vec![dir],
            (Tile::Vertical, Dir4::East | Dir4::West) => {
                vec![Dir4::North, Dir4::South]
            }
            (Tile::Horizontal, Dir4::North | Dir4::South) => {
                vec![Dir4::East, Dir4::West]
            }
            (Tile::Vertical | Tile::Horizontal, other) => vec![other],
            (Tile::SouthWest, Dir4::North) | (Tile::SouthEast, Dir4::South) => {
                vec![Dir4::West]
            }
            (Tile::SouthWest, Dir4::South) | (Tile::SouthEast, Dir4::North) => {
                vec![Dir4::East]
            }
            (Tile::SouthWest, Dir4::East) | (Tile::SouthEast, Dir4::West) => {
                vec![Dir4::South]
            }
            (Tile::SouthWest, Dir4::West) | (Tile::SouthEast, Dir4::East) => {
                vec![Dir4::North]
            }
        };
        for dir in next {
            let opt = grid.get_dir8(curr.0, dir.into());
            if let Some((x, _)) = opt {
                queue.push_back((x.to_flat_index(grid), dir));
            }
        }
    }
    let visited = visited.into_iter().map(|(x, _)| x).collect::<HashSet<_>>();
    Some(visited.len())
}
pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse(input);
    (0..grid.height())
        .flat_map(|y| {
            vec![
                ((y, 0).to_flat_index(&grid), Dir4::East),
                ((y, grid.width() - 1).to_flat_index(&grid), Dir4::West),
            ]
        })
        .chain((0..grid.width()).flat_map(|x| {
            vec![
                ((0, x).to_flat_index(&grid), Dir4::South),
                ((grid.height() - 1, x).to_flat_index(&grid), Dir4::North),
            ]
        }))
        .map(|s| solve(&grid, s))
        .max()
        .unwrap()
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
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(7_067));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(51));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(7_324));
    }
}
