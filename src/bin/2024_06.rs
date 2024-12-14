use std::collections::HashSet;

use all_aoc::helper::{dense_grid::DenseGrid, position::Direction4};

all_aoc::solution!(6, 2024);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Obstruction,
    Air,
    Guard(Direction4),
}
impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Air,
            '#' => Self::Obstruction,
            'v' => Self::Guard(Direction4::South),
            '^' => Self::Guard(Direction4::North),
            '<' => Self::Guard(Direction4::West),
            '>' => Self::Guard(Direction4::East),
            x => unreachable!("invalid char: {x}"),
        }
    }
}
pub fn part_one(input: &str) -> Option<usize> {
    let mut grid = parse(input);
    let (pos, dir) = extract_guard_pos(&mut grid);
    Some(get_all_locations_visited(&grid, pos, dir).len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid = parse(input);
    let (pos, dir) = extract_guard_pos(&mut grid);
    let possible_poses = get_all_locations_visited(&grid, pos, dir)
        .into_iter()
        .filter(|p| *p != pos && *grid.get(*p).unwrap() == Tile::Air)
        .collect::<Vec<_>>();
    Some(
        possible_poses
            .into_iter()
            .filter(|p| {
                grid[*p] = Tile::Obstruction;
                let ret = check_if_loop(&grid, pos, dir);
                grid[*p] = Tile::Air;
                ret
            })
            .count(),
    )
}
fn extract_guard_pos(grid: &mut DenseGrid<Tile>) -> (usize, Direction4) {
    let (pos, tile) = grid
        .iter()
        .enumerate()
        .find(|(_, t)| matches!(t, Tile::Guard(_)))
        .expect("There has to be a guard");
    let dir = match tile {
        Tile::Guard(direction4) => *direction4,
        _ => unreachable!(),
    };
    grid[pos] = Tile::Air;
    (pos, dir)
}
fn get_all_locations_visited(
    grid: &DenseGrid<Tile>,
    pos: usize,
    dir: Direction4,
) -> HashSet<usize> {
    let mut set = HashSet::new();
    set.insert(pos);
    let mut pos = pos;
    let mut dir = dir;
    while let Some((next_pos, tile)) = grid.get_dir8(pos, dir.into()) {
        let (next_dir, next_pos) = match tile {
            Tile::Obstruction => (dir.turn_right(), pos),
            Tile::Air => (dir, next_pos),
            Tile::Guard(_) => unreachable!(),
        };
        set.insert(next_pos);
        pos = next_pos;
        dir = next_dir;
    }
    set
}
fn check_if_loop(grid: &DenseGrid<Tile>, pos: usize, dir: Direction4) -> bool {
    let mut set = HashSet::new();
    set.insert((pos, dir));
    let mut pos = pos;
    let mut dir = dir;
    while let Some((next_pos, tile)) = grid.get_dir8(pos, dir.into()) {
        let (next_dir, next_pos) = match tile {
            Tile::Obstruction => (dir.turn_right(), pos),
            Tile::Air => (dir, next_pos),
            Tile::Guard(_) => unreachable!(),
        };
        if !set.insert((next_pos, next_dir)) {
            return true;
        };
        pos = next_pos;
        dir = next_dir;
    }
    false
}
fn parse(input: &str) -> DenseGrid<Tile> {
    DenseGrid::from_iter_iter(input.lines().map(|line| line.chars().map(Tile::from)))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(5_534));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(2_262));
    }
}
