use core::str::FromStr;
use std::collections::VecDeque;

all_aoc::solution!(11, 2022);
#[derive(Clone, Copy)]
enum Val {
    Old,
    Number(u64),
}
impl Val {
    const fn val(&self, old: u64) -> u64 {
        match self {
            Self::Old => old,
            Self::Number(x) => *x,
        }
    }
}
impl FromStr for Val {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "old" {
            Ok(Self::Old)
        } else {
            Ok(Self::Number(s.parse().unwrap()))
        }
    }
}
#[derive(Clone, Copy)]
enum Op {
    Mul(Val, Val),
    Add(Val, Val),
}

impl Op {
    const fn execute(&self, old: u64) -> u64 {
        match self {
            Self::Mul(val1, val2) => val1.val(old) * val2.val(old),
            Self::Add(val1, val2) => val1.val(old) + val2.val(old),
        }
    }
}

impl FromStr for Op {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_ascii_whitespace();
        let n1 = parts.next().unwrap().parse().unwrap();
        let op = parts.next().unwrap();
        let n2 = parts.next().unwrap().parse().unwrap();
        match op {
            "+" => Ok(Self::Add(n1, n2)),
            "*" => Ok(Self::Mul(n1, n2)),
            x => Err(x.to_owned()),
        }
    }
}
struct Monkey {
    starting_items: VecDeque<u64>,
    op: Op,
    divisibility: u64,
    when_true: u64,
    when_false: u64,
}
impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        lines.next().unwrap();
        let si = lines.next().unwrap();
        let starting_items = si
            .trim()
            .trim_start_matches("Starting items: ")
            .split(", ")
            .map(|n| n.parse().unwrap())
            .collect();
        let op = lines.next().unwrap();
        let op = Op::from_str(op.trim().trim_start_matches("Operation: new = ")).unwrap();
        let div = lines.next().unwrap();
        let divisibility = div
            .split_ascii_whitespace()
            .next_back()
            .unwrap()
            .parse()
            .unwrap();
        let t = lines.next().unwrap();
        let when_true = t
            .split_ascii_whitespace()
            .next_back()
            .unwrap()
            .parse()
            .unwrap();
        let f = lines.next().unwrap();
        let when_false = f
            .split_ascii_whitespace()
            .next_back()
            .unwrap()
            .parse()
            .unwrap();
        Ok(Self {
            starting_items,
            op,
            divisibility,
            when_true,
            when_false,
        })
    }
}
pub fn part_one(input: &str) -> Option<u64> {
    solve(input, 3, 20)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, 1, 10_000)
}
fn solve(input: &str, div_by: u64, rounds: u64) -> Option<u64> {
    let mut vec = parse(input).collect::<Vec<_>>();
    let mod_val: u64 = vec.iter().map(|m| m.divisibility).product();
    let mut activity = vec![0; vec.len()];
    for _ in 0..rounds {
        for i in 0..vec.len() {
            let m = &vec[i];
            let op = m.op;
            let div = m.divisibility;
            let when_true = m.when_true;
            let when_false = m.when_false;
            while let Some(l) = vec[i].starting_items.pop_front() {
                activity[i] += 1;
                let n = (op.execute(l) / div_by) % mod_val;

                vec[usize::try_from(if n.is_multiple_of(div) {
                    when_true
                } else {
                    when_false
                })
                .unwrap()]
                .starting_items
                .push_back(n);
            }
        }
    }
    activity.sort_unstable_by(|a, b| b.cmp(a));
    Some(activity[0] * activity[1])
}
fn parse(input: &str) -> impl Iterator<Item = Monkey> {
    input.split("\n\n").map(|l| l.parse().unwrap())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(10_605));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(58_322));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(2_713_310_158));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(13_937_702_909));
    }
}
