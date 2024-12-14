use std::fmt::Debug;

use all_aoc::helper::grid::{dense_grid::DenseGrid, Grid};
all_aoc::solution!(18, 2015);
#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    On,
    Off,
}
impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::On => write!(f, "#"),
            Self::Off => write!(f, "."),
        }
    }
}
impl From<bool> for State {
    fn from(value: bool) -> Self {
        if value {
            Self::On
        } else {
            Self::Off
        }
    }
}
pub fn part_one(input: &str) -> Option<usize> {
    solve_part_1(input, 100)
}

pub fn part_two(input: &str) -> Option<usize> {
    solve_part_2(input, 100)
}
fn solve_part_1(input: &str, iterations: u32) -> Option<usize> {
    let mut grid = parse(input);
    for _ in 0..iterations {
        let grid_copy = grid.clone();
        let size = grid_copy.len();
        for i in 0..size {
            let neigbors = grid_copy
                .get_neigbors8(i)
                .map(|(_, x)| x)
                .filter(|s| **s == State::On)
                .count();
            let new_state = match grid_copy.get(i).unwrap() {
                State::On => {
                    matches!(neigbors, 2 | 3)
                }
                State::Off => neigbors == 3,
            };
            grid[i] = new_state.into();
        }
    }
    Some(grid.into_iter().filter(|s| *s == State::On).count())
}
fn solve_part_2(input: &str, iterations: u32) -> Option<usize> {
    let mut grid = parse(input);
    set_edges_to_on(&mut grid);
    for _ in 0..iterations {
        let grid_copy = grid.clone();
        let size = grid_copy.len();
        for i in 0..size {
            let neigbors = grid_copy
                .get_neigbors8(i)
                .map(|(_, x)| x)
                .filter(|s| **s == State::On)
                .count();
            let new_state = match grid_copy.get(i).unwrap() {
                State::On => {
                    if matches!(neigbors, 2 | 3) {
                        State::On
                    } else {
                        State::Off
                    }
                }
                State::Off => {
                    if neigbors == 3 {
                        State::On
                    } else {
                        State::Off
                    }
                }
            };
            grid[i] = new_state;
        }
        set_edges_to_on(&mut grid);
    }
    Some(grid.into_iter().filter(|s| *s == State::On).count())
}
fn set_edges_to_on(grid: &mut DenseGrid<State>) {
    let height = grid.height();
    let width = grid.width();
    grid[(0, 0)] = State::On;
    grid[(height - 1, 0)] = State::On;
    grid[(0, width - 1)] = State::On;
    grid[(height - 1, width - 1)] = State::On;
}
fn parse(input: &str) -> DenseGrid<State> {
    DenseGrid::from_iter_iter(input.lines().map(|l| {
        l.chars().map(|c| match c {
            '#' => State::On,
            '.' => State::Off,
            x => unreachable!("{x}"),
        })
    }))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_1(&all_aoc::cli::read_examples_file(DAY), 4);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_061));
    }

    #[test]
    fn test_part_two() {
        let result = solve_part_2(&all_aoc::cli::read_examples_file(DAY), 5);
        assert_eq!(result, Some(17));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_006));
    }
}
