use std::collections::{HashSet, VecDeque};

use all_aoc::helper::position3d::Position3d;

all_aoc::solution!(18, 2022);
fn check_neighbours(
    pos: Position3d<i32>,
    all: &[Position3d<i32>],
    cube_set: &mut HashSet<Position3d<i32>>,
    air_set: &mut HashSet<Position3d<i32>>,
) -> i32 {
    let mut counter = 0;
    let neighbours = get_potential_neigbours(pos);

    for neigbour in neighbours {
        if !all.contains(&neigbour) {
            if !cube_set.contains(&neigbour) {
                counter += 1;
            }
            air_set.insert(neigbour);
        }
    }
    cube_set.insert(pos);
    counter
}
const fn get_potential_neigbours(pos: Position3d<i32>) -> [Position3d<i32>; 6] {
    [
        Position3d {
            x: pos.x + 1,
            y: pos.y,
            z: pos.z,
        },
        Position3d {
            x: pos.x - 1,
            y: pos.y,
            z: pos.z,
        },
        Position3d {
            x: pos.x,
            y: pos.y + 1,
            z: pos.z,
        },
        Position3d {
            x: pos.x,
            y: pos.y - 1,
            z: pos.z,
        },
        Position3d {
            x: pos.x,
            y: pos.y,
            z: pos.z + 1,
        },
        Position3d {
            x: pos.x,
            y: pos.y,
            z: pos.z - 1,
        },
    ]
}
fn is_inside_boundary(pos: Position3d<i32>, pos1: Position3d<i32>, pos2: Position3d<i32>) -> bool {
    !(pos1.x.min(pos2.x) > pos.x
        || pos1.x.max(pos2.x) < pos.x
        || pos1.y.min(pos2.y) > pos.y
        || pos1.y.max(pos2.y) < pos.y
        || pos1.z.min(pos2.x) > pos.z
        || pos1.z.max(pos2.z) < pos.z)
}
pub fn part_one(input: &str) -> Option<i32> {
    let positions = parse(input).collect::<Vec<_>>();
    let mut cube_set = HashSet::new();
    let mut air_set = HashSet::new();
    let sum = positions
        .iter()
        .map(|pos| check_neighbours(*pos, &positions, &mut cube_set, &mut air_set))
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let positions = parse(input).collect::<Vec<_>>();
    let mut cube_set = HashSet::new();
    let mut air_set = HashSet::new();
    for pos in &positions {
        check_neighbours(*pos, &positions, &mut cube_set, &mut air_set);
    }
    let x_min = cube_set.iter().map(|pos| pos.x).min().unwrap() - 1;
    let x_max = cube_set.iter().map(|pos| pos.x).max().unwrap() + 1;
    let y_min = cube_set.iter().map(|pos| pos.y).min().unwrap() - 1;
    let y_max = cube_set.iter().map(|pos| pos.y).max().unwrap() + 1;
    let z_min = cube_set.iter().map(|pos| pos.z).min().unwrap() - 1;
    let z_max = cube_set.iter().map(|pos| pos.z).max().unwrap() + 1;
    let start = Position3d {
        x: x_min,
        y: y_min,
        z: z_min,
    };
    let end = Position3d {
        x: x_max,
        y: y_max,
        z: z_max,
    };
    let mut to_go = VecDeque::new();
    to_go.push_back(start);
    let mut counter = 0;
    let mut seen = HashSet::new();
    while let Some(current) = to_go.pop_front() {
        if !seen.insert(current) {
            continue;
        }
        let neigbours = get_potential_neigbours(current);
        for neigbour in neigbours {
            if !is_inside_boundary(neigbour, start, end) {
                continue;
            }
            if cube_set.contains(&neigbour) {
                counter += 1;
                continue;
            }
            to_go.push_back(neigbour);
        }
    }
    Some(counter)
}
fn parse(input: &str) -> impl Iterator<Item = Position3d<i32>> {
    input
        .lines()
        .map(|l| Position3d::from_it(l.split(',').map(|n| n.parse().unwrap())).unwrap())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(64));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(4_628));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(58));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(2_582));
    }
}
