use core::str::FromStr;

use all_aoc::helper::misc::Joinable as _;

all_aoc::solution!(6, 2025);
enum Op {
    Add,
    Mul,
}
impl FromStr for Op {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Mul),
            _ => Err(s.to_owned()),
        }
    }
}
pub fn part_one(input: &str) -> Option<u64> {
    Some(
        parse(input)
            .into_iter()
            .map(|(op, numbers)| {
                match op {
                    Op::Add => numbers.into_iter().reduce(|acc, e| acc + e),
                    Op::Mul => numbers.into_iter().reduce(|acc, e| acc * e),
                }
                .unwrap()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let max = matrix.iter().map(std::vec::Vec::len).max().unwrap();
    let last = matrix.last_mut().unwrap();
    while last.len() < max {
        last.push(' ');
    }
    let matrix = transpose(matrix);
    let s = matrix
        .iter()
        .map(|v| v.iter().join("").trim().to_owned())
        .join("\n");
    let mut sum = 0;
    for lines in s.split("\n\n") {
        let mut it = lines.lines();
        let first = it.next().unwrap().trim();
        let op = first[first.len() - 1..].parse().unwrap();
        let first: u64 = first[..first.len() - 1].trim().parse().unwrap();

        let erg = match op {
            Op::Add => it.fold(first, |acc, e| acc + e.parse::<u64>().unwrap()),
            Op::Mul => it.fold(first, |acc, e| acc * e.parse::<u64>().unwrap()),
        };
        sum += erg;
    }
    Some(sum)
}
fn parse(input: &str) -> Vec<(Op, Vec<u64>)> {
    let vec = input
        .lines()
        .map(|s| s.split_ascii_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let vec = transpose(vec);
    vec.into_iter()
        .map(|line| {
            let mut it = line.into_iter().rev();
            let op = it.next().unwrap().parse().unwrap();
            let rest = it.map(|l| l.parse().unwrap()).rev().collect();
            (op, rest)
        })
        .collect()
}
pub fn transpose<T: Clone>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if matrix.is_empty() {
        return vec![];
    }

    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut transposed = vec![Vec::with_capacity(rows); cols];

    for row in matrix {
        assert_eq!(row.len(), cols, "All rows have to be equal length");
        for (j, val) in row.into_iter().enumerate() {
            transposed[j].push(val);
        }
    }

    transposed
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(4_277_556));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(4_648_618_073_226));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(3_263_827));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(7_329_921_182_115));
    }
}
