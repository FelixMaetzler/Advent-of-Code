use core::cmp::Reverse;

use all_aoc::helper::{permutations::IteratorCombinator as _, position::Position};

all_aoc::solution!(9, 2025);
#[derive(Debug, Clone, Copy)]
struct Candidate {
    pub corner1: Position<u64>,
    pub corner2: Position<u64>,
    pub area: u64,
}
pub fn part_one(input: &str) -> Option<u64> {
    Some(
        parse(input)
            .combinations(2)
            .map(|v| v[0].abs_diff(&v[1]) + Position { x: 1, y: 1 })
            .map(|p| p.x * p.y)
            .max()
            .unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let points: Vec<_> = parse(input).collect();

    let mut candidates: Vec<_> = calc_all_area(points.iter()).collect();
    candidates.sort_by_key(|c| Reverse(c.area));

    let edges: Vec<_> = points
        .windows(2)
        .map(|w| (w[0], w[1]))
        .chain(core::iter::once((
            *points.last().unwrap(),
            *points.first().unwrap(),
        )))
        .collect();

    candidates
        .into_iter()
        .find(|c| is_valid_rect(c, &edges))
        .map(|c| c.area)
}

fn is_valid_rect(candidate: &Candidate, edges: &[(Position<u64>, Position<u64>)]) -> bool {
    let (min_x, max_x) = (
        candidate.corner1.x.min(candidate.corner2.x),
        candidate.corner1.x.max(candidate.corner2.x),
    );
    let (min_y, max_y) = (
        candidate.corner1.y.min(candidate.corner2.y),
        candidate.corner1.y.max(candidate.corner2.y),
    );

    edges.iter().all(|(p1, p2)| {
        let (e_min_x, e_max_x) = (p1.x.min(p2.x), p1.x.max(p2.x));
        let (e_min_y, e_max_y) = (p1.y.min(p2.y), p1.y.max(p2.y));
        !(min_x < e_max_x && max_x > e_min_x && min_y < e_max_y && max_y > e_min_y)
    })
}

fn calc_all_area<'a>(
    it: impl Iterator<Item = &'a Position<u64>>,
) -> impl Iterator<Item = Candidate> {
    it.combinations(2).map(|pair| {
        let (p1, p2) = (pair[0], pair[1]);
        let diff = p1.abs_diff(p2) + Position { x: 1, y: 1 };

        Candidate {
            corner1: *p1,
            corner2: *p2,
            area: diff.x * diff.y,
        }
    })
}

fn parse(input: &str) -> impl Iterator<Item = Position<u64>> {
    input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| Position {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        })
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(4_777_816_465));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(24));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_410_501_884));
    }
}
