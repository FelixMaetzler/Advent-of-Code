use std::{collections::HashSet, str::FromStr};

use all_aoc::helper::position::{Direction4, Position};

all_aoc::solution!(1, 2016);
enum Turn {
    Left(u32),
    Right(u32),
}
impl FromStr for Turn {
    type Err = char;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = s[1..].parse().unwrap();
        match s.chars().next().unwrap() {
            'L' => Ok(Self::Left(x)),
            'R' => Ok(Self::Right(x)),
            x => Err(x),
        }
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let vec = parse(input);
    let mut dir = Direction4::North;
    let mut pos: Position<i32> = Position::default();
    for t in vec {
        match t {
            Turn::Left(x) => {
                dir = dir.turn_left();
                for _ in 0..x {
                    pos = pos.direction(dir);
                }
            }
            Turn::Right(x) => {
                dir = dir.turn_right();
                for _ in 0..x {
                    pos = pos.direction(dir);
                }
            }
        }
    }

    Some((pos.x.abs() + pos.y.abs()).try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let vec = parse(input);
    let mut dir = Direction4::North;
    let mut pos: Position<i32> = Position::default();
    let mut set = HashSet::new();
    set.insert(pos);
    'outer: for t in vec {
        match t {
            Turn::Left(x) => {
                dir = dir.turn_left();
                for _ in 0..x {
                    pos = pos.direction(dir);
                    if !set.insert(pos) {
                        break 'outer;
                    }
                }
            }
            Turn::Right(x) => {
                dir = dir.turn_right();
                for _ in 0..x {
                    pos = pos.direction(dir);
                    if !set.insert(pos) {
                        break 'outer;
                    }
                }
            }
        }
    }
    Some((pos.x.abs() + pos.y.abs()).try_into().unwrap())
}
fn parse(input: &str) -> Vec<Turn> {
    input
        .split(", ")
        .map(|ins| Turn::from_str(ins).unwrap())
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(271));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(153));
    }
}
