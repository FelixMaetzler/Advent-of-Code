use std::collections::HashSet;

use all_aoc::helper::{
    grid::{Grid, dense_grid::DenseGrid, grid_index::GridIndex},
    misc::gcd,
    position::Position,
};

all_aoc::solution!(10, 2019);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Asteroid,
}
struct Asteroid {
    pos: Position<i32>,
    angle: f64,
    mag: f64,
}
impl Asteroid {
    fn new(pos: Position<i32>, reference: Position<i32>) -> Self {
        let diff = pos - reference;
        let angle = (-diff.y as f64).atan2(diff.x as f64).to_degrees();
        let mag = ((diff.x * diff.x + diff.y * diff.y) as f64).sqrt();
        Self { pos, angle, mag }
    }
}
impl PartialEq for Asteroid {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}
impl Ord for Asteroid {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match transform(self.angle).partial_cmp(&transform(other.angle)) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord.unwrap(),
        }
        self.mag.partial_cmp(&other.mag).unwrap()
    }
}
impl PartialOrd for Asteroid {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for Asteroid {}
fn transform(x: f64) -> f64 {
    //Shifts the angle from the mathematic definiton to the puzzles definiton
    let test = (x + 360.0) % 360.0;
    let test = -test;
    let test = test + 90.0;
    (test + 360.0) % 360.0
}
pub fn part_one(input: &str) -> Option<usize> {
    solve_part_one(&parse(input)).map(|e| e.0)
}
pub fn part_two(input: &str) -> Option<i32> {
    let grid = parse(input);
    let station = solve_part_one(&grid).unwrap().1;
    let mut asteroids = grid
        .iter()
        .enumerate()
        .flat_map(|(i, t)| (*t == Tile::Asteroid).then(|| i.to_position(&grid)))
        .map(|Position { x, y }| Position {
            x: x as i32,
            y: y as i32,
        })
        .map(|p| Asteroid::new(p, station))
        .collect::<Vec<_>>();
    asteroids.sort_unstable();
    let mut i = 0;
    let mut counter = 0;
    loop {
        let a = asteroids.remove(i);
        counter += 1;
        if counter == 200 {
            return Some(a.pos.x * 100 + a.pos.y);
        }
        while asteroids[i % asteroids.len()].angle == a.angle {
            i = (i + 1) % asteroids.len();
        }
    }
}
fn solve_part_one(grid: &DenseGrid<Tile>) -> Option<(usize, Position<i32>)> {
    let all_astroids = grid
        .iter()
        .enumerate()
        .flat_map(|(i, t)| (*t == Tile::Asteroid).then(|| i.to_position(grid)))
        .map(|Position { x, y }| Position {
            x: x as i32,
            y: y as i32,
        })
        .collect::<HashSet<_>>();
    Some(
        all_astroids
            .iter()
            .map(|pos| {
                (
                    calc(
                        all_astroids.clone(),
                        *pos,
                        grid.width() as i32,
                        grid.height() as i32,
                    ),
                    *pos,
                )
            })
            .max_by_key(|(k, _)| *k)
            .unwrap(),
    )
}
fn calc(mut set: HashSet<Position<i32>>, pos: Position<i32>, width: i32, height: i32) -> usize {
    set.remove(&pos);
    let mut editable_set: HashSet<Position<i32>> = set.clone();
    for n in &set {
        let dist = pos - *n;
        let dist = shrink(dist);

        let mut curr = pos + dist;
        let mut found = false;
        while (0..width).contains(&curr.x) && (0..height).contains(&curr.y) {
            if editable_set.contains(&curr) {
                if found {
                    editable_set.remove(&curr);
                } else {
                    found = true;
                }
            }
            curr += dist;
        }

        curr = pos - dist;
        found = false;
        while (0..width).contains(&curr.x) && (0..height).contains(&curr.y) {
            if editable_set.contains(&curr) {
                if found {
                    editable_set.remove(&curr);
                } else {
                    found = true;
                }
            }
            curr -= dist;
        }
    }
    editable_set.len()
}
fn shrink(pos: Position<i32>) -> Position<i32> {
    let Position { x, y } = pos;
    let gcd = gcd(x.unsigned_abs(), y.unsigned_abs()) as i32;
    Position {
        x: x / gcd,
        y: y / gcd,
    }
}

fn parse(input: &str) -> DenseGrid<Tile> {
    DenseGrid::from_iter_iter(input.lines().map(|l| {
        l.chars().map(|c| match c {
            '.' => Tile::Empty,
            '#' => Tile::Asteroid,
            x => unreachable!("wrong char: {x}"),
        })
    }))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(210));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(299));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(802));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1419));
    }
}
