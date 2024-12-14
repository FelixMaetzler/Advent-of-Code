use std::{fmt::Display, iter, str::FromStr};

use all_aoc::helper::{modulo::ModuloSignedExt, position::Position};

all_aoc::solution!(14, 2024);
#[derive(Debug)]
struct Robot {
    p: Position<i32>,
    v: Position<i32>,
}
impl Robot {
    fn step(&mut self, size: Position<i32>) {
        self.p += self.v;
        self.p = self.p.modulo(size);
    }
}
#[derive(Debug)]
struct Grid {
    size: Position<i32>,
    robots: Vec<Robot>,
}
impl Grid {
    fn step(&mut self) {
        self.robots.iter_mut().for_each(|r| r.step(self.size));
    }
    fn solve_part_1(mut self) -> Option<usize> {
        // we can do 100 small steps or one big step
        self.robots.iter_mut().for_each(|r| r.v *= 100);
        self.step();

        Some(self.entropy())
    }

    fn entropy(&self) -> usize {
        let left = 0..self.size.x / 2;
        let up = 0..self.size.y / 2;
        let right = (self.size.x / 2) + 1..self.size.x;
        let down = (self.size.y / 2) + 1..self.size.y;
        let mut up_left = 0;
        let mut up_right = 0;
        let mut down_left = 0;
        let mut down_right = 0;
        for r in &self.robots {
            let p = r.p;
            if left.contains(&p.x) {
                if up.contains(&p.y) {
                    up_left += 1;
                } else if down.contains(&p.y) {
                    down_left += 1;
                }
            } else if right.contains(&p.x) {
                if up.contains(&p.y) {
                    up_right += 1;
                } else if down.contains(&p.y) {
                    down_right += 1;
                }
            }
        }
        up_left * up_right * down_left * down_right
    }
}
impl FromStr for Robot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p, v) = s.split_once(" ").unwrap();
        let p = p[2..].parse().unwrap();
        let v = v[2..].parse().unwrap();
        Ok(Self { p, v })
    }
}
impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = (0..self.size.y)
            .map(|y| {
                (0..self.size.x)
                    .map(move |x| {
                        let p = Position { x, y };
                        if self.robots.iter().any(|r| r.p == p) {
                            "#"
                        } else {
                            "."
                        }
                    })
                    .chain(iter::once("\n"))
                    .collect::<String>()
            })
            .collect::<String>();
        f.write_str(&s)
    }
}
pub fn part_one(input: &str) -> Option<usize> {
    let robots = parse(input);
    let grid = Grid {
        size: Position { x: 101, y: 103 },
        robots,
    };
    grid.solve_part_1()
}

pub fn part_two(input: &str) -> Option<u32> {
    let robots = parse(input);
    let mut grid = Grid {
        size: Position { x: 101, y: 103 },
        robots,
    };
    for i in 1.. {
        grid.step();
        let e = grid.entropy();
        // Empirical threshold
        if e < 60_000_000 {
            return Some(i);
        }
    }
    None
}

fn parse(input: &str) -> Vec<Robot> {
    input.lines().map(|l| Robot::from_str(l).unwrap()).collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let robots = parse(&all_aoc::cli::read_examples_file(DAY));
        let grid = Grid {
            size: Position { x: 11, y: 7 },
            robots,
        };
        let result = grid.solve_part_1();
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(215_987_200));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(8_050));
    }
}
