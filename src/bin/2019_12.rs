use all_aoc::helper::{misc::lcm3, permutations::IteratorCombinator, position3d::Position3d};
use std::fmt::Debug;
all_aoc::solution!(12, 2019);
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Moon {
    pos: Position3d<i32>,
    vel: Position3d<i32>,
}

impl Moon {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            pos: Position3d { x, y, z },
            vel: Position3d::default(),
        }
    }
    fn update_pos(&mut self) {
        self.pos += self.vel
    }
    fn potential_energy(&self) -> i32 {
        self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs()
    }
    fn kinetic_energy(&self) -> i32 {
        self.vel.x.abs() + self.vel.y.abs() + self.vel.z.abs()
    }
    fn total_energy(&self) -> i32 {
        self.potential_energy() * self.kinetic_energy()
    }
}
impl Debug for Moon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "pos=<x={}, y={}, z={}>, vel=<x={}, y={}, z={}>",
            self.pos.x, self.pos.y, self.pos.z, self.vel.x, self.vel.y, self.vel.z
        )
    }
}
pub fn part_one(input: &str) -> Option<i32> {
    part_one_wrapper(input, 1000)
}

pub fn part_two(input: &str) -> Option<u64> {
    let vec = parse(input);
    assert_eq!(vec.len(), 4);
    let vec = [vec[0], vec[1], vec[2], vec[3]];
    Some(solve_part_2(vec))
}
pub fn part_one_wrapper(input: &str, steps: usize) -> Option<i32> {
    let mut vec = parse(input);
    for _ in 0..steps {
        step(&mut vec);
    }
    Some(vec.iter().map(|m| m.total_energy()).sum())
}
fn solve_part_2(mut vec: [Moon; 4]) -> u64 {
    let mut steps = 0;
    let mut erg = [None; 3];
    loop {
        step(&mut vec);
        steps += 1;
        for (axis, x) in erg.iter_mut().enumerate() {
            if x.is_none()
                && vec
                    .iter()
                    .map(|m| match axis {
                        0 => m.vel.x,
                        1 => m.vel.y,
                        2 => m.vel.z,
                        _ => unreachable!(),
                    })
                    .all(|i| i == 0)
            {
                *x = Some(steps);
            }
        }
        if erg.iter().filter(|i| i.is_none()).count() == 0 {
            return 2 * lcm3(erg[0].unwrap(), erg[1].unwrap(), erg[2].unwrap());
        }
    }
}

fn step(vec: &mut [Moon]) {
    for m in (0..vec.len()).combinations(2) {
        update(m[0], m[1], vec);
    }
    vec.iter_mut().for_each(|m| m.update_pos());
    fn update(m1: usize, m2: usize, vec: &mut [Moon]) {
        vec[m1].vel.x += (vec[m2].pos.x - vec[m1].pos.x).signum();
        vec[m2].vel.x += (vec[m1].pos.x - vec[m2].pos.x).signum();

        vec[m1].vel.y += (vec[m2].pos.y - vec[m1].pos.y).signum();
        vec[m2].vel.y += (vec[m1].pos.y - vec[m2].pos.y).signum();

        vec[m1].vel.z += (vec[m2].pos.z - vec[m1].pos.z).signum();
        vec[m2].vel.z += (vec[m1].pos.z - vec[m2].pos.z).signum();
    }
}
fn parse(input: &str) -> Vec<Moon> {
    // every line looks like this:
    // <x=-1, y=0, z=2>
    input
        .lines()
        .map(|l| &l[1..l.len() - 1]) //cutting out the < and >
        .map(|l| l.split(','))
        .map(|s| {
            s.map(|c| c.split_once('=').unwrap())
                .map(|(_, n)| n.parse().unwrap())
        })
        .map(|mut v| Moon::new(v.next().unwrap(), v.next().unwrap(), v.next().unwrap()))
        .collect()
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = &all_aoc::cli::read_examples_file(DAY);
        let (first, second) = input.split_once("\n\n").unwrap();
        assert_eq!(part_one_wrapper(first, 10), Some(179));
        assert_eq!(part_one_wrapper(second, 100), Some(1940));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(9_999));
    }

    #[test]
    fn test_part_two() {
        let input = &all_aoc::cli::read_examples_file(DAY);
        let (first, second) = input.split_once("\n\n").unwrap();
        assert_eq!(part_two(first), Some(2772));
        assert_eq!(part_two(second), Some(4_686_774_924));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(282_399_002_133_976));
    }
}
