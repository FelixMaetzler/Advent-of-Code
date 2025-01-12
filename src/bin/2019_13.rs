use std::{collections::HashMap, iter};

use all_aoc::helper::{
    intcode::{InputMode, IntInteger, Intcode, Return},
    position::Position,
};

all_aoc::solution!(13, 2019);
#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}
impl TryFrom<IntInteger> for Tile {
    type Error = ();

    fn try_from(value: IntInteger) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Empty),
            1 => Ok(Self::Wall),
            2 => Ok(Self::Block),
            3 => Ok(Self::Paddle),
            4 => Ok(Self::Ball),
            _ => Err(()),
        }
    }
}
pub fn part_one(input: &str) -> Option<usize> {
    let mut computer = parse(input);
    computer.execute();
    Some(
        computer
            .get_outputs()
            .as_slice()
            .chunks(3)
            .map(|s| s[2])
            .filter(|i| *i == 2)
            .count(),
    )
}
pub fn part_two(input: &str) -> Option<isize> {
    let mut computer = parse(input);
    computer.execute();
    let map = computer
        .get_outputs()
        .as_slice()
        .chunks(3)
        .map(|s| ((s[0], s[1]), Tile::try_from(s[2]).unwrap()))
        .collect::<HashMap<_, _>>();

    let mut computer = parse(input);
    computer.halt_at_output(true);
    computer[0] = 2;
    let mut score = None;

    let mut ball = map
        .iter()
        .find(|(_, v)| **v == Tile::Ball)
        .map(|((x, y), _)| Position { x: *x, y: *y })
        .unwrap();
    let mut paddle = map
        .iter()
        .find(|(_, v)| **v == Tile::Paddle)
        .map(|((x, y), _)| Position { x: *x, y: *y })
        .unwrap();
    let mut input = 0;
    let mut cnt = 0;
    loop {
        cnt += 1;
        computer.set_inputs(iter::once(input), InputMode::Replace);
        if computer.execute() == Return::Finished {
            break;
        }
        if computer.execute() == Return::Finished {
            break;
        }
        if computer.execute() == Return::Finished {
            break;
        }
        let output = computer.get_outputs();
        let x = *output.iter().rev().nth(2).unwrap();
        let y = *output.iter().rev().nth(1).unwrap();
        let t = *output.iter().next_back().unwrap();
        if x == -1 && y == 0 {
            score = Some(t);
            continue;
        } else {
            let t = Tile::try_from(t).unwrap();
            match t {
                Tile::Paddle => {
                    paddle.x = x;
                    paddle.y = y;
                    input = 0;
                }
                Tile::Ball => {
                    ball.x = x;
                    ball.y = y;
                    input = (ball.x - paddle.x).signum()
                }
                _ => {
                    input = 0;
                }
            }
        }
    }
    dbg!(cnt);
    score
}
fn parse(input: &str) -> Intcode {
    Intcode::new(input.split(',').map(|x| x.parse().unwrap()).collect())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(226));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(10_800));
    }
}
