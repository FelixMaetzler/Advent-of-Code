use core::str::FromStr;
use std::collections::HashSet;

use all_aoc::helper::position::{Direction4, Position};

all_aoc::solution!(9, 2022);
struct Instruction {
    dir: Direction4,
    step_size: i32,
}
impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (d, s) = s.split_once(' ').unwrap();
        let step_size = s.parse().unwrap();
        let dir = match d {
            "U" => Direction4::North,
            "D" => Direction4::South,
            "R" => Direction4::East,
            "L" => Direction4::West,
            _ => unreachable!(),
        };
        Ok(Self { dir, step_size })
    }
}
const fn update(head: Position<i32>, tail: Position<i32>) -> Position<i32> {
    //right
    if head.x - tail.x >= 2 {
        if head.y - tail.y >= 1 {
            return Position {
                x: tail.x + 1,
                y: tail.y + 1,
            };
        } else if head.y - tail.y == 0 {
            return Position {
                x: tail.x + 1,
                y: tail.y,
            };
        } else if head.y - tail.y <= -1 {
            return Position {
                x: tail.x + 1,
                y: tail.y - 1,
            };
        }
    } else
    //left
    if head.x - tail.x <= -2 {
        if head.y - tail.y >= 1 {
            return Position {
                x: tail.x - 1,
                y: tail.y + 1,
            };
        } else if head.y - tail.y == 0 {
            return Position {
                x: tail.x - 1,
                y: tail.y,
            };
        } else if head.y - tail.y <= -1 {
            return Position {
                x: tail.x - 1,
                y: tail.y - 1,
            };
        }
    } else
    //up
    if head.y - tail.y >= 2 {
        if head.x - tail.x >= 1 {
            return Position {
                x: tail.x + 1,
                y: tail.y + 1,
            };
        } else if head.x - tail.x == 0 {
            return Position {
                x: tail.x,
                y: tail.y + 1,
            };
        } else if head.x - tail.x <= -1 {
            return Position {
                x: tail.x - 1,
                y: tail.y + 1,
            };
        }
    } else
    //Down
    if head.y - tail.y <= -2 {
        if head.x - tail.x >= 1 {
            return Position {
                x: tail.x + 1,
                y: tail.y - 1,
            };
        } else if head.x - tail.x == 0 {
            return Position {
                x: tail.x,
                y: tail.y - 1,
            };
        } else if head.x - tail.x <= -1 {
            return Position {
                x: tail.x - 1,
                y: tail.y - 1,
            };
        }
    }
    tail
}
pub fn part_one(input: &str) -> Option<usize> {
    let i = parse(input);
    let mut head = Position { x: 0, y: 0 };
    let mut tail = Position { x: 0, y: 0 };
    let mut set1 = HashSet::new();
    for ins in i {
        for _ in 0..ins.step_size {
            set1.insert(tail);
            head = head.direction(ins.dir);
            tail = update(head, tail);
        }
    }
    set1.insert(tail);
    Some(set1.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let i = parse(input);
    let mut head = Position { x: 0, y: 0 };
    let mut tail = Position { x: 0, y: 0 };
    let mut set = HashSet::new();
    let mut tails = [Position { x: 0, y: 0 }; 10];
    for ins in i {
        for _ in 0..ins.step_size {
            set.insert(*tails.last().unwrap());
            head = head.direction(ins.dir);
            tail = update(head, tail);
            tails[0] = head;
            for i in 1..tails.len() {
                let head_x = tails[i - 1];
                let tail_x = tails[i];
                let x = update(head_x, tail_x);
                tails[i] = x;
            }
        }
    }
    set.insert(*tails.last().unwrap());
    Some(set.len())
}
fn parse(input: &str) -> impl Iterator<Item = Instruction> {
    input.lines().map(|l| Instruction::from_str(l).unwrap())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(6_081));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(2_487));
    }
}
