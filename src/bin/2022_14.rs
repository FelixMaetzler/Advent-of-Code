use core::cmp::Ordering;

use all_aoc::helper::{
    grid::{Grid as _, dense::DenseGrid, index::GridIndex as _},
    position::Position,
};

all_aoc::solution!(14, 2022);
#[derive(Default, Clone, Copy, Debug)]
enum Tile {
    #[default]
    Air,
    Rock,
    Sand,
}
pub fn part_one(input: &str) -> Option<usize> {
    let mut grid = setup_grid(input);
    let max_height = grid
        .iter()
        .enumerate()
        .filter(|(_, t)| matches!(t, Tile::Rock))
        .map(|(i, _)| i.to_position(&grid).y)
        .max()
        .unwrap();
    'outer: loop {
        let mut sand_pos = Position { x: 500, y: 0 };

        'inner: loop {
            if let Some((new, t)) =
                grid.get_dir8(sand_pos, all_aoc::helper::position::Direction8::South)
                && matches!(t, Tile::Air)
            {
                sand_pos = new.to_position(&grid);
            } else if let Some((new, t)) =
                grid.get_dir8(sand_pos, all_aoc::helper::position::Direction8::SouthWest)
                && matches!(t, Tile::Air)
            {
                sand_pos = new.to_position(&grid);
            } else if let Some((new, t)) =
                grid.get_dir8(sand_pos, all_aoc::helper::position::Direction8::SouthEast)
                && matches!(t, Tile::Air)
            {
                sand_pos = new.to_position(&grid);
            } else {
                grid.set(sand_pos, Tile::Sand);
                break 'inner;
            }
            if sand_pos.y > max_height {
                break 'outer;
            }
        }
    }

    Some(grid.iter().filter(|t| matches!(t, Tile::Sand)).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid = setup_grid(input);
    let max_height = grid
        .iter()
        .enumerate()
        .filter(|(_, t)| matches!(t, Tile::Rock))
        .map(|(i, _)| i.to_position(&grid).y)
        .max()
        .unwrap();
    for x in 0..grid.width() {
        grid.set(
            Position {
                x,
                y: max_height + 2,
            },
            Tile::Rock,
        );
    }
    loop {
        let mut sand_pos = Position { x: 500, y: 0 };
        if matches!(grid[sand_pos], Tile::Sand) {
            break;
        }
        loop {
            if let Some((new, t)) =
                grid.get_dir8(sand_pos, all_aoc::helper::position::Direction8::South)
                && matches!(t, Tile::Air)
            {
                sand_pos = new.to_position(&grid);
            } else if let Some((new, t)) =
                grid.get_dir8(sand_pos, all_aoc::helper::position::Direction8::SouthWest)
                && matches!(t, Tile::Air)
            {
                sand_pos = new.to_position(&grid);
            } else if let Some((new, t)) =
                grid.get_dir8(sand_pos, all_aoc::helper::position::Direction8::SouthEast)
                && matches!(t, Tile::Air)
            {
                sand_pos = new.to_position(&grid);
            } else {
                grid.set(sand_pos, Tile::Sand);
                break;
            }
        }
    }

    Some(grid.iter().filter(|t| matches!(t, Tile::Sand)).count())
}
fn setup_grid(input: &str) -> DenseGrid<Tile> {
    let mut grid = DenseGrid::new(1000, 1000, Tile::default());
    let input = parse(input);
    for path in input {
        let v = path.collect::<Vec<_>>();
        for w in v.windows(2) {
            let (a, b) = (w[0], w[1]);
            let pos_it = match a.x.cmp(&b.x) {
                Ordering::Less => Box::new((a.x..=b.x).map(|x| Position { x, y: a.y }))
                    as Box<dyn Iterator<Item = Position<usize>>>,
                Ordering::Equal => {
                    if a.y < b.y {
                        Box::new((a.y..=b.y).map(|y| Position { x: a.x, y }))
                            as Box<dyn Iterator<Item = Position<usize>>>
                    } else {
                        Box::new((b.y..=a.y).map(|y| Position { x: a.x, y }))
                            as Box<dyn Iterator<Item = Position<usize>>>
                    }
                }
                Ordering::Greater => Box::new((b.x..=a.x).map(|x| Position { x, y: a.y }))
                    as Box<dyn Iterator<Item = Position<usize>>>,
            };
            for p in pos_it {
                grid.set(p, Tile::Rock);
            }
        }
    }
    grid
}
fn parse(input: &str) -> impl Iterator<Item = impl Iterator<Item = Position<usize>>> {
    input
        .lines()
        .map(|l| l.split(" -> ").map(|p| p.parse().unwrap()))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(24));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(696));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(93));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(23_610));
    }
}
