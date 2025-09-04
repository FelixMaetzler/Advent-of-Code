use core::fmt::Debug;
use std::collections::HashSet;

use all_aoc::helper::{
    grid::{Grid as _, dense::DenseGrid, index::GridIndex as _},
    position::Direction8,
};

all_aoc::solution!(10, 2023);

#[derive(PartialEq, Clone, Copy)]
enum Tile {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}
impl Debug for Tile {
    #[expect(clippy::non_ascii_literal, reason = "its the task")]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NorthSouth => write!(f, "|"),
            Self::EastWest => write!(f, "-"),
            Self::NorthEast => write!(f, "⌞"),
            Self::NorthWest => write!(f, "⌟"),
            Self::SouthWest => write!(f, "⌝"),
            Self::SouthEast => write!(f, "⌜"),
            Self::Ground => write!(f, "."),
            Self::Start => write!(f, "S"),
        }
    }
}
#[derive(Debug, Clone, Copy)]
enum Dir {
    North,
    South,
    West,
    East,
}
impl Dir {
    const fn step(self, pos: &mut (usize, usize)) {
        match self {
            Self::North => pos.0 -= 1,
            Self::South => pos.0 += 1,
            Self::West => pos.1 -= 1,
            Self::East => pos.1 += 1,
        }
    }

    fn turn(&mut self, t: Tile) {
        use Dir::{East, North, South, West};
        use Tile::{EastWest, NorthEast, NorthSouth, NorthWest, SouthEast, SouthWest};
        *self = match (&self, t) {
            (North, NorthSouth) | (West, NorthEast) | (East, NorthWest) => North,
            (North, SouthEast) | (South, NorthEast) | (East, EastWest) => East,
            (North, SouthWest) | (South, NorthWest) | (West, EastWest) => West,
            (South, NorthSouth) | (West, SouthEast) | (East, SouthWest) => South,
            _ => unreachable!(),
        };
    }
}
impl TryFrom<char> for Tile {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Self::NorthSouth),
            '-' => Ok(Self::EastWest),
            'L' => Ok(Self::NorthEast),
            'J' => Ok(Self::NorthWest),
            '7' => Ok(Self::SouthWest),
            'F' => Ok(Self::SouthEast),
            '.' => Ok(Self::Ground),
            'S' => Ok(Self::Start),
            val => Err(val),
        }
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse(input);
    let (start, mut dir) = start(&grid);
    let mut curr = start;
    let mut ctr = 0;
    loop {
        dir.step(&mut curr);
        ctr += 1;
        if curr == start {
            return Some(ctr / 2);
        }
        let t = grid[curr];
        dir.turn(t);
    }
}
fn start(grid: &DenseGrid<Tile>) -> ((usize, usize), Dir) {
    use Tile::{EastWest, NorthEast, NorthSouth, NorthWest, SouthEast, SouthWest, Start};
    let start = grid
        .iter()
        .enumerate()
        .find(|(_, t)| t == &&Start)
        .unwrap()
        .0;
    let dir = if grid
        .get_dir8(start, Direction8::North)
        .is_some_and(|(_, t)| matches!(t, SouthEast | SouthWest | NorthSouth))
    {
        Dir::North
    } else if grid
        .get_dir8(start, Direction8::South)
        .is_some_and(|(_, t)| matches!(t, NorthEast | NorthWest | NorthSouth))
    {
        Dir::South
    } else if grid
        .get_dir8(start, Direction8::West)
        .is_some_and(|(_, t)| matches!(t, NorthEast | EastWest | SouthEast))
    {
        Dir::West
    } else if grid
        .get_dir8(start, Direction8::East)
        .is_some_and(|(_, t)| matches!(t, EastWest | NorthEast | SouthEast))
    {
        Dir::East
    } else {
        unreachable!()
    };
    (start.to_coordinates(grid), dir)
}
fn get_start_tile(grid: &DenseGrid<Tile>) -> Tile {
    use Tile::{EastWest, NorthEast, NorthSouth, NorthWest, SouthEast, SouthWest, Start};
    let mut vec = vec![];
    let start = grid
        .iter()
        .enumerate()
        .find(|(_, t)| t == &&Start)
        .unwrap()
        .0;
    if grid
        .get_dir8(start, Direction8::North)
        .is_some_and(|(_, t)| matches!(t, SouthEast | SouthWest | NorthSouth))
    {
        vec.push(Dir::North);
    }
    if grid
        .get_dir8(start, Direction8::South)
        .is_some_and(|(_, t)| matches!(t, NorthEast | NorthWest | NorthSouth))
    {
        vec.push(Dir::South);
    }
    if grid
        .get_dir8(start, Direction8::West)
        .is_some_and(|(_, t)| matches!(t, NorthEast | EastWest | SouthEast))
    {
        vec.push(Dir::West);
    }
    if grid
        .get_dir8(start, Direction8::East)
        .is_some_and(|(_, t)| matches!(t, EastWest | NorthWest | SouthWest))
    {
        vec.push(Dir::East);
    }
    assert_eq!(vec.len(), 2);
    match (vec[0], vec[1]) {
        (Dir::North, Dir::South) => Tile::NorthSouth,
        (Dir::North, Dir::West) => Tile::NorthWest,
        (Dir::North, Dir::East) => Tile::NorthEast,

        (Dir::South, Dir::West) => Tile::SouthWest,
        (Dir::South, Dir::East) => Tile::SouthEast,

        (Dir::West, Dir::East) => Tile::EastWest,

        _ => unreachable!(),
    }
}
pub fn part_two(input: &str) -> Option<usize> {
    let mut grid = parse(input);
    let (start, mut dir) = start(&grid);
    grid[start] = get_start_tile(&grid);
    let grid = grid;
    let mut curr = start;
    let mut lop = HashSet::new();
    loop {
        dir.step(&mut curr);
        lop.insert(curr.to_flat_index(&grid));
        if curr == start {
            break;
        }
        let t = grid[curr];
        dir.turn(t);
    }
    let width = grid.width();
    let grid = DenseGrid::from_iter(
        grid.into_iter()
            .enumerate()
            .map(|(i, t)| if lop.contains(&i) { t } else { Tile::Ground }),
        width,
    );
    let mut sum = 0;

    for row in (0..grid.height()).map(|n| grid.get_row(n)) {
        let mut inside = false;
        for t in row {
            if *t == Tile::Ground {
                sum += usize::from(inside);
            } else if matches!(t, Tile::NorthSouth | Tile::NorthEast | Tile::NorthWest) {
                inside = !inside;
            }
        }
    }
    Some(sum)
}

fn parse(input: &str) -> DenseGrid<Tile> {
    DenseGrid::from_iter_iter(
        input
            .trim()
            .lines()
            .map(|l| l.chars().map(|c| Tile::try_from(c).unwrap())),
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(
            all_aoc::cli::read_examples_file(DAY)
                .split_once("\n\n")
                .unwrap()
                .0,
        );
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(6_831));
    }

    #[test]
    fn test_part_two() {
        let input = all_aoc::cli::read_examples_file(DAY);
        let mut it = input.split("\n\n").skip(1);

        assert_eq!(part_two(it.next().unwrap()), Some(4));
        assert_eq!(part_two(it.next().unwrap()), Some(4));
        assert_eq!(part_two(it.next().unwrap()), Some(8));
        assert_eq!(part_two(it.next().unwrap()), Some(10));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(305));
    }
}
