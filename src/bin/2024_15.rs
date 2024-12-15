use std::{
    collections::HashSet,
    fmt::{Debug, Write},
};

use all_aoc::helper::{
    grid::{dense_grid::DenseGrid, grid_index::GridIndex, Grid},
    position::{Direction4, Direction8},
};

all_aoc::solution!(15, 2024);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TilePart1 {
    Robot,
    Wall,
    Box,
    Empty,
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum TilePart2 {
    Robot,
    Wall,
    BoxLeft,
    BoxRight,
    Empty,
}
impl Debug for TilePart2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            TilePart2::Robot => '@',
            TilePart2::Wall => '#',
            TilePart2::BoxLeft => '[',
            TilePart2::BoxRight => ']',
            TilePart2::Empty => '.',
        })
    }
}
pub fn part_one(input: &str) -> Option<usize> {
    let (mut grid, dirs) = parse_part_1(input);
    let mut start = grid
        .iter()
        .enumerate()
        .find(|(_, t)| **t == TilePart1::Robot)
        .unwrap()
        .0;
    for dir in dirs {
        start = make_move_part_1(&mut grid, start, dir).to_flat_index(&grid);
    }
    Some(
        grid.iter()
            .enumerate()
            .filter(|(_, t)| matches!(t, TilePart1::Box))
            .map(|(i, _)| {
                let (y, x) = i.to_coordinates(&grid);
                100 * y + x
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (mut grid, dirs) = parse_part_2(input);
    let mut start = grid
        .iter()
        .enumerate()
        .find(|(_, t)| **t == TilePart2::Robot)
        .unwrap()
        .0;
    for dir in dirs {
        start = make_move_part_2(&mut grid, start, dir).to_flat_index(&grid);
    }

    Some(
        grid.iter()
            .enumerate()
            .filter(|(_, t)| matches!(t, TilePart2::BoxLeft))
            .map(|(i, _)| {
                let (y, x) = i.to_coordinates(&grid);
                100 * y + x
            })
            .sum(),
    )
}
fn make_move_part_1(
    grid: &mut DenseGrid<TilePart1>,
    pos: usize,
    dir: Direction4,
) -> impl GridIndex<TilePart1> {
    debug_assert_eq!(grid.get(pos), Some(&TilePart1::Robot));
    if let Some((mut next_pos, next_tile)) = grid.get_dir8(pos, dir.into()) {
        let first = next_pos.to_flat_index(grid);
        match next_tile {
            TilePart1::Robot => unreachable!(),
            TilePart1::Wall => pos,
            TilePart1::Empty => {
                grid.set(pos, TilePart1::Empty);
                grid.set(first, TilePart1::Robot);
                first
            }
            TilePart1::Box => loop {
                match grid.get_dir8(next_pos, dir.into()) {
                    Some((np, next_tile)) => match next_tile {
                        TilePart1::Robot => unreachable!(),
                        TilePart1::Wall => return pos,
                        TilePart1::Box => next_pos = np,
                        TilePart1::Empty => {
                            grid.set(pos, TilePart1::Empty);
                            grid.set(first, TilePart1::Robot);
                            grid.set(np, TilePart1::Box);
                            return first;
                        }
                    },
                    None => return pos,
                }
            },
        }
    } else {
        pos
    }
}
fn make_move_part_2(
    grid: &mut DenseGrid<TilePart2>,
    pos: usize,
    dir: Direction4,
) -> impl GridIndex<TilePart2> {
    debug_assert_eq!(grid.get(pos), Some(&TilePart2::Robot));
    if let Some((next_pos, next_tile)) = grid.get_dir8(pos, dir.into()) {
        let first = next_pos.to_flat_index(grid);
        match next_tile {
            TilePart2::Robot => unreachable!(),
            TilePart2::Wall => pos,
            TilePart2::Empty => {
                grid.set(pos, TilePart2::Empty);
                grid.set(first, TilePart2::Robot);
                first
            }
            TilePart2::BoxLeft | TilePart2::BoxRight => {
                if let Some((from, to)) = is_free(grid, next_pos, dir) {
                    grid.set(pos, TilePart2::Empty);
                    for i in from {
                        grid.set(i, TilePart2::Empty);
                    }
                    for (i, t) in to {
                        grid.set(i, t);
                    }
                    grid.set(first, TilePart2::Robot);
                    first
                } else {
                    pos
                }
            }
        }
    } else {
        pos
    }
}
type From = Vec<(usize, usize)>;
type To = Vec<((usize, usize), TilePart2)>;
fn is_free(
    grid: &DenseGrid<TilePart2>,
    pos: (usize, usize),
    dir: Direction4,
) -> Option<(From, To)> {
    debug_assert!(matches!(
        grid.get(pos).unwrap(),
        TilePart2::BoxLeft | TilePart2::BoxRight
    ));
    let mut stack = vec![pos];
    let mut from = vec![];
    let mut to = vec![];
    let mut visited = HashSet::new();
    while let Some(pos) = stack.pop() {
        if !visited.insert(pos) {
            continue;
        }
        match grid.get(pos).expect("Should exisit") {
            TilePart2::Robot => unreachable!(),
            TilePart2::Wall => return None,
            TilePart2::Empty => continue,
            TilePart2::BoxLeft => {
                from.push(pos);
                to.push((pos.dir(dir.into(), grid).unwrap(), TilePart2::BoxLeft));
                let right = pos.dir(Direction8::East, grid).unwrap();
                let new_dir = pos.dir(dir.into(), grid).unwrap();
                stack.push(right);
                stack.push(new_dir);
            }
            TilePart2::BoxRight => {
                from.push(pos);
                to.push((pos.dir(dir.into(), grid).unwrap(), TilePart2::BoxRight));
                let left = pos.dir(Direction8::West, grid).unwrap();
                let new_dir = pos.dir(dir.into(), grid).unwrap();
                stack.push(left);
                stack.push(new_dir);
            }
        }
    }
    Some((from, to))
}
fn parse_part_1(input: &str) -> (DenseGrid<TilePart1>, Vec<Direction4>) {
    let (grid, dirs) = input.split_once("\n\n").unwrap();
    let grid = DenseGrid::from_iter_iter(grid.lines().map(|l| {
        l.chars().map(|c| match c {
            '@' => TilePart1::Robot,
            '#' => TilePart1::Wall,
            'O' => TilePart1::Box,
            '.' => TilePart1::Empty,
            x => unreachable!("worng char: {x}"),
        })
    }));
    let vec = dirs
        .lines()
        .flat_map(|l| l.chars().map(|c| Direction4::from_hat(c).unwrap()))
        .collect();
    (grid, vec)
}
fn parse_part_2(input: &str) -> (DenseGrid<TilePart2>, Vec<Direction4>) {
    let (grid, dirs) = input.split_once("\n\n").unwrap();
    let grid = DenseGrid::from_iter_iter(grid.lines().map(|l| {
        l.chars().flat_map(|c| {
            match c {
                '@' => [TilePart2::Robot, TilePart2::Empty],
                '#' => [TilePart2::Wall, TilePart2::Wall],
                'O' => [TilePart2::BoxLeft, TilePart2::BoxRight],
                '.' => [TilePart2::Empty, TilePart2::Empty],
                x => unreachable!("worng char: {x}"),
            }
            .into_iter()
        })
    }));
    let vec = dirs
        .lines()
        .flat_map(|l| l.chars().map(|c| Direction4::from_hat(c).unwrap()))
        .collect();
    (grid, vec)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(10_092));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_486_930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(9_021));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_492_011));
    }
}
