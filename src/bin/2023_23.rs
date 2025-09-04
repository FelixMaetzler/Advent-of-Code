use std::collections::{HashMap, HashSet};

use all_aoc::helper::{
    bitmask::Bitmask as _,
    grid::{Grid as _, dense::DenseGrid, index::GridIndex as _},
    position::{Direction4 as Dir, Direction8},
};

all_aoc::solution!(23, 2023);

#[derive(Eq, PartialEq, Clone, Debug, Copy)]
enum Tile {
    Path,
    Forrest,
    Slope(Dir),
}
impl TryFrom<char> for Tile {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Path),
            '#' => Ok(Self::Forrest),
            '>' => Ok(Self::Slope(Dir::East)),
            '<' => Ok(Self::Slope(Dir::West)),
            '^' => Ok(Self::Slope(Dir::North)),
            'v' => Ok(Self::Slope(Dir::South)),
            val => Err(val),
        }
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    execute(input)
}
fn get_all_neigbors(grid: &DenseGrid<Tile>, index: usize) -> Vec<usize> {
    let n = grid.get_neigbors4(index);
    n.into_iter()
        .filter(|(_, t)| *t != &Tile::Forrest)
        .map(|(i, _)| i.to_flat_index(grid))
        .collect()
}
fn get_neigbors(grid: &DenseGrid<Tile>, index: usize) -> Vec<usize> {
    let curr = grid.get(index).unwrap();
    match curr {
        Tile::Path => {
            let mut ret = vec![];
            for dir in Dir::all_dirs().map(core::convert::Into::into) {
                if let Some((next, tile)) = grid.get_dir8(index, dir)
                    && matches!(
                        (tile, dir),
                        (Tile::Path, _)
                            | (Tile::Slope(Dir::North), Direction8::North)
                            | (Tile::Slope(Dir::South), Direction8::South)
                            | (Tile::Slope(Dir::East), Direction8::East)
                            | (Tile::Slope(Dir::West), Direction8::West)
                    )
                {
                    ret.push(next.to_flat_index(grid));
                }
            }
            ret
        }
        Tile::Slope(dir) => {
            if let Some((next, tile)) = grid.get_dir8(index, (*dir).into()) {
                match tile {
                    Tile::Path => vec![next.to_flat_index(grid)],
                    Tile::Forrest => vec![],
                    Tile::Slope(_) => unreachable!(),
                }
            } else {
                vec![]
            }
        }
        Tile::Forrest => unreachable!(),
    }
}
fn find_next_intersection(
    grid: &DenseGrid<Tile>,
    start_intersection: usize,
    start_dir: usize,
    intersections: &[usize],
) -> Option<(usize, u32)> {
    let mut visited = HashSet::new();
    visited.insert(start_intersection);
    let mut ctr = 0;
    let mut curr = start_dir;
    loop {
        if intersections.contains(&curr) {
            return Some((curr, ctr + 1));
        }
        let mut n = get_neigbors(grid, curr);
        n.retain(|e| !visited.contains(e));
        match n.len() {
            0 => return None,
            1 => {
                ctr += 1;
                visited.insert(curr);
                curr = n[0];
            }
            _ => {
                unreachable!(
                    "If there are more than 2, it is a intersection and should be handled earlier"
                )
            }
        }
    }
}
fn build_graph(
    grid: &DenseGrid<Tile>,
    start: usize,
    end: usize,
) -> HashMap<usize, HashSet<(usize, u32)>> {
    let mut map = HashMap::new();
    let intersections = grid
        .iter()
        .enumerate()
        .filter(|(_, t)| t != &&Tile::Forrest)
        .filter(|(i, _)| get_all_neigbors(grid, *i).len() > 2)
        .map(|(i, _)| i)
        .chain(vec![start, end])
        .collect::<Vec<_>>();
    for inter in &intersections {
        let neig = get_neigbors(grid, *inter);
        for n in neig {
            if let Some(erg) = find_next_intersection(grid, *inter, n, &intersections) {
                map.entry(*inter)
                    .and_modify(|s: &mut HashSet<(usize, u32)>| {
                        s.insert(erg);
                    })
                    .or_insert_with(|| HashSet::from_iter(vec![erg]));
            }
        }
        map.entry(*inter).or_default();
    }
    map
}
fn recurse(
    tree: &Vec<Vec<(usize, u32)>>,
    curr: usize,
    end: usize,
    sum: u32,
    visited: usize,
) -> Option<u32> {
    if curr == end {
        return Some(sum);
    }
    let mut neigbors = tree[curr].clone();
    neigbors.retain(|v| !visited.get_bit(v.0));
    let mut visited = visited;
    visited.set_bit(curr, true);
    neigbors
        .into_iter()
        .filter_map(|v| recurse(tree, v.0, end, sum + v.1, visited))
        .max()
}
fn execute(input: &str) -> Option<u32> {
    let grid = parse(input);
    let start = (0..grid.width())
        .map(|x| (x, grid.get((0, x)).unwrap()))
        .find(|(_, t)| t == &&Tile::Path)
        .unwrap()
        .0;
    let end = (0..grid.width())
        .map(|x| {
            (
                (grid.height() - 1, x),
                grid.get((grid.height() - 1, x)).unwrap(),
            )
        })
        .find(|(_, t)| t == &&Tile::Path)
        .unwrap()
        .0
        .to_flat_index(&grid);
    let tree = build_graph(&grid, start, end);
    let mut map = HashMap::new();
    let mut map_rev = HashMap::new();
    let mut ctr = 0;
    for v in tree.keys() {
        if map.contains_key(v) {
            continue;
        }
        map.insert(v, ctr);
        map_rev.insert(ctr, v);
        ctr += 1;
    }
    let mut vec = Vec::with_capacity(map.len());
    for i in 0..tree.len() {
        let s = &tree[map_rev[&i]];
        let s = s
            .iter()
            .map(|(d, val)| (map[d], *val))
            .collect::<HashSet<_>>();
        vec.push(s);
    }
    let erg = recurse(
        &vec.into_iter()
            .map(|s| s.into_iter().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
        map[&start],
        map[&end],
        0,
        0,
    );
    Some(erg.unwrap())
}
pub fn part_two(input: &str) -> Option<u32> {
    let input = input.replace(['<', '>', '^', 'v'], ".");
    execute(&input)
}
fn parse(input: &str) -> DenseGrid<Tile> {
    DenseGrid::from_string(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(2_414));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(154));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(6_598));
    }
}
