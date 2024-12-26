use std::collections::HashMap;

use all_aoc::helper::{
    grid::{grid_index::GridIndex, sparse_grid::SparseGrid, Grid},
    permutations::IteratorPermutator,
    position::{Direction4, Position},
};
type Pos = Position<usize>;
all_aoc::solution!(21, 2024);

struct Keypad {
    map: SparseGrid<char>,
    positions: HashMap<char, Pos>,
}

impl Keypad {
    fn new_dir() -> Self {
        let mut map = SparseGrid::new(3, 2);
        map.set(1, '^');
        map.set(2, 'A');
        map.set(3, '<');
        map.set(4, 'v');
        map.set(5, '>');
        let positions = map
            .iter_all()
            .map(|(k, v)| (*v, k.to_position(&map)))
            .collect();
        Self { map, positions }
    }
    fn new_num() -> Self {
        let mut map = SparseGrid::new(3, 4);
        map.set(0, '7');
        map.set(1, '8');
        map.set(2, '9');
        map.set(3, '4');
        map.set(4, '5');
        map.set(5, '6');
        map.set(6, '1');
        map.set(7, '2');
        map.set(8, '3');
        map.set(10, '0');
        map.set(11, 'A');

        let positions = map
            .iter_all()
            .map(|(k, v)| (*v, k.to_position(&map)))
            .collect();
        Self { map, positions }
    }
}
struct Solution {
    door: Keypad,
    robot: Keypad,
    targets: Vec<(usize, Vec<char>)>,
}

type Cache = HashMap<(char, char, usize), usize>;

impl Solution {
    fn new(input: &str) -> Self {
        Self {
            door: Keypad::new_num(),
            robot: Keypad::new_dir(),
            targets: input
                .lines()
                .map(|line| {
                    (
                        line.strip_suffix('A').unwrap().parse::<usize>().unwrap(),
                        line.chars().collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<_>>(),
        }
    }
    fn traverse(
        &self,
        keypad: &Keypad,
        a: char,
        b: char,
        depth: usize,
        cache: &mut Cache,
    ) -> usize {
        if let Some(&result) = cache.get(&(a, b, depth)) {
            return result;
        }

        let from = keypad.positions[&a];
        let to = keypad.positions[&b];

        if depth == 0 {
            return to.manhattan_distance(&from) + 1;
        }

        let mut moves = Vec::new();

        if from.x < to.x {
            moves.extend([Direction4::East].repeat(to.x - from.x));
        } else {
            moves.extend([Direction4::West].repeat(from.x - to.x));
        }

        if from.y < to.y {
            moves.extend([Direction4::South].repeat(to.y - from.y));
        } else {
            moves.extend([Direction4::North].repeat(from.y - to.y));
        }

        let result = moves
            .iter()
            .permutations(moves.len())
            .filter_map(|moves| {
                let mut p = from;

                for &&d in &moves {
                    p = keypad.map.get_dir8(p, d.into())?.0.to_position(&keypad.map);
                }
                let v = ['A']
                    .into_iter()
                    .chain(moves.into_iter().map(|d| d.to_hat()))
                    .chain(['A'])
                    .collect::<Vec<_>>();
                Some(
                    v.windows(2)
                        .map(|w| self.traverse(&self.robot, w[0], w[1], depth - 1, cache))
                        .sum::<usize>(),
                )
            })
            .min()
            .expect("failed to find move set");

        cache.insert((a, b, depth), result);
        result
    }

    fn solve(&self, depth: usize) -> Option<usize> {
        let mut cache = Cache::default();

        Some(
            self.targets
                .iter()
                .map(|(number, seq)| {
                    let v = ['A'].iter().chain(seq.iter()).collect::<Vec<_>>();

                    number
                        * v.windows(2)
                            .map(|w| self.traverse(&self.door, *w[0], *w[1], depth, &mut cache))
                            .sum::<usize>()
                })
                .sum(),
        )
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Solution::new(input).solve(2)
}

pub fn part_two(input: &str) -> Option<usize> {
    Solution::new(input).solve(25)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(126_384));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(188_384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(154_115_708_116_294));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(232_389_969_568_832));
    }
}
