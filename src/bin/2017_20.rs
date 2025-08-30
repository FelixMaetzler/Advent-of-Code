use core::{cmp::Ordering, str::FromStr};
use std::collections::BTreeSet;

use all_aoc::helper::{permutations::IteratorCombinator as _, position3d::Position3d};

all_aoc::solution!(20, 2017);
#[derive(Clone, Copy)]
struct Particle {
    position: Position3d<isize>,
    velocity: Position3d<isize>,
    acceleration: Position3d<isize>,
}
impl Particle {
    fn update(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
    }
    const fn manhattan_distance(&self) -> isize {
        self.position.manhattan_norm_squared()
    }
    const fn vel_mag(&self) -> isize {
        self.velocity.manhattan_norm_squared()
    }
    const fn acc_mag(&self) -> isize {
        self.acceleration.manhattan_norm_squared()
    }
    pub fn cmp_particle(&self, other: &Self) -> Ordering {
        (self.acc_mag(), self.vel_mag(), self.manhattan_distance()).cmp(&(
            other.acc_mag(),
            other.vel_mag(),
            other.manhattan_distance(),
        ))
    }
    const fn position_at(&self, t: isize) -> Position3d<isize> {
        Position3d {
            x: self.position.x + self.velocity.x * t + self.acceleration.x * t * (t + 1) / 2,
            y: self.position.y + self.velocity.y * t + self.acceleration.y * t * (t + 1) / 2,
            z: self.position.z + self.velocity.z * t + self.acceleration.z * t * (t + 1) / 2,
        }
    }
}

impl FromStr for Particle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(", ");
        let p = it
            .next()
            .unwrap()
            .chars()
            .filter(|&c| !"pva<>=".contains(c))
            .collect::<String>();
        let v = it
            .next()
            .unwrap()
            .chars()
            .filter(|&c| !"pva<>=".contains(c))
            .collect::<String>();
        let a = it
            .next()
            .unwrap()
            .chars()
            .filter(|&c| !"pva<>=".contains(c))
            .collect::<String>();
        let position = Position3d::from_it(p.split(',').map(|n| n.parse().unwrap())).unwrap();
        let velocity = Position3d::from_it(v.split(',').map(|n| n.parse().unwrap())).unwrap();
        let acceleration = Position3d::from_it(a.split(',').map(|n| n.parse().unwrap())).unwrap();
        Ok(Self {
            position,
            velocity,
            acceleration,
        })
    }
}
pub fn part_one(input: &str) -> Option<usize> {
    Some(
        parse(input)
            .enumerate()
            .min_by(|p1, p2| p1.1.cmp_particle(&p2.1))
            .unwrap()
            .0,
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut vec = parse(input).collect::<Vec<_>>();
    let max = vec
        .iter()
        .combinations(2)
        .map(|w| (w[0], w[1]))
        .filter_map(|(a, b)| first_collision(a, b))
        .max()
        .unwrap();

    for _ in 0..=max {
        vec.sort_unstable_by(|p1, p2| {
            (p1.position.x, p1.position.y, p1.position.z).cmp(&(
                p2.position.x,
                p2.position.y,
                p2.position.z,
            ))
        });
        let set = vec
            .windows(2)
            .enumerate()
            .filter(|&(_, w)| w[0].position == w[1].position)
            .flat_map(|(i, _)| [i, i + 1])
            .collect();
        remove_indices(&mut vec, &set);
        for p in &mut vec {
            p.update();
        }
    }
    Some(vec.len())
}

fn solve_axis(dp: isize, dv: isize, da: isize) -> Vec<isize> {
    if da == 0 {
        if dv == 0 {
            if dp == 0 {
                return vec![];
            }
        } else if -2 * dp % (2 * dv) == 0 {
            let t = -2 * dp / (2 * dv);
            if t >= 0 {
                return vec![t];
            }
        }
        vec![]
    } else {
        let a = da;
        let b = da + 2 * dv;
        let c = 2 * dp;
        let discriminant = b * b - 4 * a * c;
        if discriminant < 0 {
            return vec![];
        }

        let mut roots = Vec::new();
        #[expect(
            clippy::cast_precision_loss,
            reason = "dont need that much precision here"
        )]
        let sqrt_d = (discriminant as f64).sqrt();
        for sign in [-1.0_f64, 1.0] {
            #[expect(
                clippy::cast_precision_loss,
                reason = "dont need that much precision here"
            )]
            let t = sign.mul_add(sqrt_d, -b as f64) / (2.0 * a as f64);
            if t.fract() == 0.0 {
                #[expect(clippy::cast_possible_truncation, reason = "checked for fract")]
                let t_int = t as isize;
                if t_int >= 0 {
                    roots.push(t_int);
                }
            }
        }
        roots.sort_unstable();
        roots.dedup();
        roots
    }
}

fn first_collision(p1: &Particle, p2: &Particle) -> Option<isize> {
    let dx = solve_axis(
        p1.position.x - p2.position.x,
        p1.velocity.x - p2.velocity.x,
        p1.acceleration.x - p2.acceleration.x,
    );
    let dy = solve_axis(
        p1.position.y - p2.position.y,
        p1.velocity.y - p2.velocity.y,
        p1.acceleration.y - p2.acceleration.y,
    );
    let dz = solve_axis(
        p1.position.z - p2.position.z,
        p1.velocity.z - p2.velocity.z,
        p1.acceleration.z - p2.acceleration.z,
    );

    let mut candidates: Vec<isize> = dx;
    if candidates.is_empty() {
        candidates = dy;
    }
    if candidates.is_empty() {
        candidates = dz;
    }

    for &t in &candidates {
        let pos1 = p1.position_at(t);
        let pos2 = p2.position_at(t);
        if pos1 == pos2 {
            return Some(t);
        }
    }
    None
}
fn parse(input: &str) -> impl Iterator<Item = Particle> {
    input.lines().map(|line| Particle::from_str(line).unwrap())
}
fn remove_indices<T>(vec: &mut Vec<T>, indices: &BTreeSet<usize>) {
    for &i in indices.iter().rev() {
        if i < vec.len() {
            vec.swap_remove(i);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(243));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(648));
    }
}
