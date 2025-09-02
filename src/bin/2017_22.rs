use std::collections::HashMap;

use all_aoc::helper::position::{Direction4, Position};

all_aoc::solution!(22, 2017);
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum Tile {
    #[default]
    Clean,
    Weakend,
    Infected,
    Flagged,
}
impl Tile {
    const fn next(self) -> Self {
        match self {
            Self::Clean => Self::Weakend,
            Self::Weakend => Self::Infected,
            Self::Infected => Self::Flagged,
            Self::Flagged => Self::Clean,
        }
    }
}
impl TryFrom<char> for Tile {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Clean),
            '#' => Ok(Self::Infected),
            x => Err(x),
        }
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = parse(input);
    #[expect(clippy::cast_precision_loss, reason = "f64 cast")]
    #[expect(clippy::cast_possible_truncation, reason = "f64 cast")]
    let mid = (((grid.len() as f64).sqrt()) as isize - 1) / 2;
    let mut pos = Position { x: mid, y: -mid };
    let mut dir = Direction4::North;
    let mut cnt = 0;
    for _ in 0..10_000 {
        if grid.get(&pos).copied().unwrap_or_default() == Tile::Infected {
            dir = dir.turn_right();
            grid.insert(pos, Tile::Clean);
        } else {
            dir = dir.turn_left();
            grid.insert(pos, Tile::Infected);
            cnt += 1;
        }
        pos = pos.direction(dir);
    }

    Some(cnt)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = parse(input);
    #[expect(clippy::cast_precision_loss, reason = "f64 cast")]
    #[expect(clippy::cast_possible_truncation, reason = "f64 cast")]
    let mid = ((grid.len() as f64).sqrt() as isize - 1) / 2;
    let mut pos = Position { x: mid, y: -mid };
    let mut dir = Direction4::North;
    let mut cnt = 0;
    for _ in 0..10_000_000 {
        let t = grid.get(&pos).copied().unwrap_or_default();
        grid.insert(pos, t.next());
        match t {
            Tile::Clean => dir = dir.turn_left(),
            Tile::Weakend => cnt += 1,
            Tile::Infected => dir = dir.turn_right(),
            Tile::Flagged => dir = dir.opposite(),
        }
        pos = pos.direction(dir);
    }

    Some(cnt)
}
fn parse(input: &str) -> HashMap<Position<isize>, Tile> {
    (input.lines().enumerate().flat_map(|(y, l)| {
        l.chars().enumerate().map(move |(x, c)| {
            (
                Position {
                    x: isize::try_from(x).unwrap(),
                    y: -(isize::try_from(y).unwrap()),
                },
                c.try_into().unwrap(),
            )
        })
    }))
    .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(5_587));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(5_462));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(2_511_944));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(2_512_135));
    }
}
