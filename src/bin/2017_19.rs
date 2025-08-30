use all_aoc::helper::{
    grid::{Grid as _, dense::DenseGrid, index::GridIndex as _},
    position::{Direction4, Position},
};

all_aoc::solution!(19, 2017);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Air,
    Horizontal,
    Vertical,
    Cross,
    Char(char),
}
impl TryFrom<char> for Tile {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            ' ' => Ok(Self::Air),
            '|' => Ok(Self::Vertical),
            '-' => Ok(Self::Horizontal),
            '+' => Ok(Self::Cross),
            x if x.is_ascii_alphabetic() => Ok(Self::Char(x)),
            x => Err(x),
        }
    }
}
pub fn part_one(input: &str) -> Option<String> {
    solve(input).0
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input).1
}
fn solve(input: &str) -> (Option<String>, Option<u32>) {
    let grid = parse(input);
    let mut curr = (0..grid.width())
        .find(|&x| *grid.get(Position { x, y: 0 }).unwrap() == Tile::Vertical)
        .unwrap()
        .to_position(&grid);
    let mut dir = Direction4::South;
    let mut s = String::new();
    let mut cnt = 0;
    loop {
        let (c, t) = grid.get_dir8(curr, dir.into()).unwrap();
        curr = c.to_position(&grid);
        cnt += 1;
        match t {
            Tile::Air => break,
            Tile::Horizontal | Tile::Vertical => {}
            Tile::Cross => {
                for d in Direction4::all_dirs()
                    .iter()
                    .filter(|d| **d != dir.opposite())
                {
                    if grid
                        .get_dir8(curr, (*d).into())
                        .is_some_and(|(_, t)| *t != Tile::Air)
                    {
                        dir = *d;

                        break;
                    }
                }
            }
            Tile::Char(c) => s.push(*c),
        }
    }
    (Some(s), Some(cnt))
}
fn parse(input: &str) -> DenseGrid<Tile> {
    let mut grid = DenseGrid::new(
        input.lines().nth(3).unwrap().chars().count(),
        input.lines().count(),
        Tile::Air,
    );
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| (Position { x, y }, c.try_into().unwrap()))
        })
        .for_each(|(pos, t)| {
            grid.set(pos, t);
        });
    grid
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some("ABCDEF".to_owned()));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some("SXWAIBUZY".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(38));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(16_676));
    }
}
