use std::collections::{HashMap, HashSet, hash_map::Entry};

use all_aoc::helper::{
    grid::{Grid as _, dense::DenseGrid, index::GridIndex as _},
    position::Position,
};

all_aoc::solution!(21, 2023);

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum Tile {
    Start,
    Rock,
    GardenPlot,
}
impl TryFrom<char> for Tile {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::GardenPlot),
            '#' => Ok(Self::Rock),
            'S' => Ok(Self::Start),
            x => Err(x),
        }
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    execute(input, 64)
}
fn execute(input: &str, n: usize) -> Option<u32> {
    let grid = parse(input);
    let mut set = HashSet::new();
    let start = grid
        .iter()
        .enumerate()
        .find(|(_, t)| t == &&Tile::Start)
        .unwrap()
        .0;
    set.insert(start);
    for _ in 0..n {
        let mut new_set = HashSet::new();
        for i in set {
            let n = get_neigbours(&grid, i);
            new_set.extend(n);
        }
        set = new_set;
    }
    Some(set.len().try_into().unwrap())
}
fn execute2(input: &str, n: usize) -> Option<u32> {
    let grid = parse(input);
    let mut set = HashSet::new();
    let start = grid
        .iter()
        .enumerate()
        .find(|(_, t)| t == &&Tile::Start)
        .unwrap()
        .0
        .to_position(&grid);
    let start: Position<isize> = start.try_into().unwrap();
    set.insert(start);
    let mut meoisation: HashMap<Position<isize>, Vec<Position<isize>>> = HashMap::new();
    for _ in 0..n {
        let mut new_set = HashSet::with_capacity(set.len());
        for i in &set {
            let n = if let Entry::Vacant(e) = meoisation.entry(*i) {
                let n = get_neigbours2(&grid, *i);
                e.insert(n.clone());
                n
            } else {
                meoisation[i].clone()
            };
            new_set.extend(n);
        }
        set = new_set;
    }
    Some(set.len().try_into().unwrap())
}
fn get_neigbours(grid: &DenseGrid<Tile>, index: usize) -> HashSet<usize> {
    let n = grid.get_neigbors4(index);
    n.into_iter()
        .filter(|(_, t)| *t != &Tile::Rock)
        .map(|(i, _)| i.to_flat_index(grid))
        .collect()
}
fn get_neigbours2(grid: &DenseGrid<Tile>, index: Position<isize>) -> Vec<Position<isize>> {
    let mut ret = Vec::new();

    let new_index = Position {
        x: index.x,
        y: index.y - 1,
    };
    if grid.get(wrap_index(new_index, grid)).unwrap() != &Tile::Rock {
        ret.push(new_index);
    }
    let new_index = Position {
        x: index.x,
        y: index.y + 1,
    };
    if grid.get(wrap_index(new_index, grid)).unwrap() != &Tile::Rock {
        ret.push(new_index);
    }
    let new_index = Position {
        x: index.x - 1,
        y: index.y,
    };
    if grid.get(wrap_index(new_index, grid)).unwrap() != &Tile::Rock {
        ret.push(new_index);
    }
    let new_index = Position {
        x: index.x + 1,
        y: index.y,
    };
    if grid.get(wrap_index(new_index, grid)).unwrap() != &Tile::Rock {
        ret.push(new_index);
    }
    ret
}
fn wrap_index(index: Position<isize>, grid: &DenseGrid<Tile>) -> Position<usize> {
    let index = Position {
        x: index.x.rem_euclid(grid.width().try_into().unwrap()),
        y: index.y.rem_euclid(grid.height().try_into().unwrap()),
    };
    index.try_into().unwrap()
}
pub fn part_two(input: &str) -> Option<u64> {
    execute2_wrapper(input, 26_501_365)
}
fn execute2_wrapper(input: &str, n: usize) -> Option<u64> {
    let grid = parse(input);

    let mut points = vec![];

    for ctr in 0..=2 {
        let val = n % grid.width() + grid.width() * ctr;
        let ret = execute2(input, val);
        points.push((val.try_into().unwrap(), ret.unwrap().into()));
    }
    let params = solve_quadratic(points[0], points[1], points[2]);
    #[expect(clippy::cast_precision_loss, reason = "its fine")]
    let ret = solve(params, n as f64);
    #[expect(clippy::cast_possible_truncation, reason = "its fine")]
    #[expect(clippy::cast_sign_loss, reason = "its fine")]
    Some(ret.ceil() as u64)
}
#[expect(clippy::cast_precision_loss, reason = "its fine")]
fn solve_quadratic(p1: (i64, i64), p2: (i64, i64), p3: (i64, i64)) -> (f64, f64, f64) {
    // https://www.wolframalpha.com/input?i=inv+%7B%7B%28x_1%29%5E2%2C+x_1%2C+1%7D%2C%7B%28x_2%29%5E2%2C+x_2%2C+1%7D%2C%7B%28x_3%29%5E2%2C+x_3%2C+1%7D%7D+*+%7B%7By_1%7D%2C%7By_2%7D%2C%7By_3%7D%7D
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let (x3, y3) = p3;
    let det = (x1 - x2) * (x1 - x3) * (x2 - x3);
    //------
    let a = x2 * y1 - x3 * y1 - x1 * y2 + x3 * y2 + x1 * y3 - x2 * y3;
    let a = a as f64 / det as f64;
    //------
    let b = x1.pow(2) * (y2 - y3) + x2.pow(2) * (y3 - y1) + x3.pow(2) * (y1 - y2);
    let b = b as f64 / det as f64;
    //------
    let c = x1.pow(2) * (x2 * y3 - x3 * y2)
        + x2.pow(2) * (x3 * y1 - x1 * y3)
        + x3.pow(2) * (x1 * y2 - x2 * y1);
    let c = c as f64 / det as f64;
    (a, b, c)
}
fn solve(params: (f64, f64, f64), x: f64) -> f64 {
    let (a, b, c) = params;
    (a * x).mul_add(x, b * x) + c
}
fn parse(input: &str) -> DenseGrid<Tile> {
    DenseGrid::from_string(input)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = &all_aoc::cli::read_examples_file(DAY);
        assert_eq!(execute(input, 1), Some(2));
        assert_eq!(execute(input, 2), Some(4));
        assert_eq!(execute(input, 3), Some(6));
        assert_eq!(execute(input, 6), Some(16));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(3_658));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(608_193_767_979_991));
    }
}
