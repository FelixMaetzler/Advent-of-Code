use std::str::FromStr;

use all_aoc::helper::position::Position;

all_aoc::solution!(13, 2024);
#[derive(Debug)]
struct ClawMachine {
    a: Position<i64>,
    b: Position<i64>,
    prize: Position<i64>,
}

impl ClawMachine {
    fn solve(&self) -> Option<u64> {
        // through observation i see that every LGS in the input has exactly one solution (|A| != 0)
        // the only catch is to see if this solution is a integer one
        let (a, c) = self.a.as_xy_tuple();
        let (b, d) = self.b.as_xy_tuple();
        let (x, y) = self.prize.as_xy_tuple();
        let first = d * x - b * y;
        let second = -c * x + a * y;
        let det = det(a, b, c, d);
        // Here i check for the integer solution
        if first % det == 0 && second % det == 0 {
            let i = first / det;
            let j = second / det;
            debug_assert!(!i.is_negative());
            debug_assert!(!j.is_negative());
            Some((3 * i + j) as u64)
        } else {
            None
        }
    }
}
impl FromStr for ClawMachine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.lines().collect::<Vec<_>>();
        debug_assert_eq!(v.len(), 3);
        let a = v[0].split_ascii_whitespace().collect::<Vec<_>>();
        let a = Position {
            x: a[2][2..].trim_end_matches(",").parse().unwrap(),
            y: a[3][2..].parse().unwrap(),
        };
        let b = v[1].split_ascii_whitespace().collect::<Vec<_>>();
        let b = Position {
            x: b[2][2..].trim_end_matches(",").parse().unwrap(),
            y: b[3][2..].parse().unwrap(),
        };
        let prize = v[2].split_ascii_whitespace().collect::<Vec<_>>();
        let prize = Position {
            x: prize[1][2..].trim_end_matches(",").parse().unwrap(),
            y: prize[2][2..].parse().unwrap(),
        };
        Ok(ClawMachine { a, b, prize })
    }
}
pub fn part_one(input: &str) -> Option<u64> {
    let machines = parse(input);
    Some(machines.into_iter().flat_map(|m| m.solve()).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    const ADD: i64 = 10_000_000_000_000;
    let mut machines = parse(input);
    machines.iter_mut().for_each(|m| m.prize += ADD);
    Some(machines.into_iter().flat_map(|m| m.solve()).sum())
}
fn parse(input: &str) -> Vec<ClawMachine> {
    input
        .split("\n\n")
        .map(|m| ClawMachine::from_str(m).unwrap())
        .collect()
}
fn det(a: i64, b: i64, c: i64, d: i64) -> i64 {
    a * d - b * c
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(31_897));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(875_318_608_908));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(87_596_249_540_359));
    }
}
