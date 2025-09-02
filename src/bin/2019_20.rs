use core::fmt::Debug;
use std::collections::{HashMap, HashSet};

use all_aoc::helper::{
    graph::{Graph as _, Special, WithWeights as _, build_graph4_special},
    grid::{Grid as _, dense::DenseGrid, index::GridIndex as _},
    position::{Direction4, Position},
};
enum Facing {
    Inner,
    Outer,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    pos: Position<usize>,
    level: u32,
}
fn is_facing(grid: &DenseGrid<Tile>, pos: Position<usize>) -> Facing {
    let width = grid.width();
    let height = grid.height();
    if pos.x == 2 || pos.x == width - 3 || pos.y == 2 || pos.y == height - 3 {
        Facing::Outer
    } else {
        Facing::Inner
    }
}
all_aoc::solution!(20, 2019);
#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Air,
    Path,
    Wall,
    Char(char),
}
impl Debug for Tile {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Air => write!(f, " "),
            Self::Path => write!(f, "."),
            Self::Wall => write!(f, "#"),
            Self::Char(arg0) => write!(f, "{arg0}"),
        }
    }
}
impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            ' ' => Self::Air,
            '.' => Self::Path,
            '#' => Self::Wall,
            x if x.is_ascii_alphabetic() => Self::Char(x),
            _ => unreachable!(),
        }
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse(input);
    let mut graph = build_graph4_special(&grid, |curr, neigh| {
        (curr == neigh && *curr == Tile::Path).then_some(1)
    });
    let portals = find_portals(&grid);
    let start = portals.iter().find(|(k, _)| *k == "AA").unwrap().1[0].to_flat_index(&grid);
    let end = portals.iter().find(|(k, _)| *k == "ZZ").unwrap().1[0].to_flat_index(&grid);
    for v in portals.values() {
        if v.len() == 2 {
            let a = v[0].to_flat_index(&grid);
            let b = v[1].to_flat_index(&grid);
            graph.add_edge(a, b, 1);
            graph.add_edge(b, a, 1);
        }
    }
    let d = graph.dijkstra_distances(start, Some(end));
    Some(d[&end])
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse(input);
    let easy_graph = build_graph4_special(&grid, |curr, neigh| {
        (curr == neigh && *curr == Tile::Path).then_some(1)
    });
    let portals = find_portals(&grid);
    let start = portals.iter().find(|(k, _)| *k == "AA").unwrap().1[0];
    let end = portals.iter().find(|(k, _)| *k == "ZZ").unwrap().1[0];

    let start = Node {
        pos: start,
        level: 0,
    };
    let end = Node { pos: end, level: 0 };
    let all_portals = portals.values().flatten().copied().collect::<HashSet<_>>();
    let portal_count: u32 = all_portals.len().try_into().unwrap();
    let mut all_dist = HashMap::new();
    for i in &all_portals {
        let dist = easy_graph.dijkstra_distances(i.to_flat_index(&grid), None);
        for j in &all_portals {
            if i == j {
                continue;
            }
            if let Some(d) = dist.get(&j.to_flat_index(&grid)) {
                let mut s = [*i, *j];
                s.sort();
                if let Some(x) = all_dist.insert(s, *d) {
                    debug_assert_eq!(x, *d);
                }
            }
        }
    }
    let mut node_to_index = HashMap::new();
    let mut graph = Special::new();
    for level in 0..=(2 * portal_count) {
        for p in &all_portals {
            let n = Node { pos: *p, level };
            let number = node_to_index.len();
            node_to_index.insert(n, number);
        }
    }
    for level in 0..(2 * portal_count) {
        for ([from, to], d) in &all_dist {
            let from = node_to_index[&Node { pos: *from, level }];
            let to = node_to_index[&Node { pos: *to, level }];
            graph.add_edge(from, to, *d);
            graph.add_edge(to, from, *d);
        }
        for pair in portals.values() {
            if pair.len() == 2 {
                let from = pair[0];
                let to = pair[1];
                match (is_facing(&grid, from), is_facing(&grid, to)) {
                    (Facing::Inner, Facing::Inner) | (Facing::Outer, Facing::Outer) => {
                        unreachable!()
                    }
                    (Facing::Inner, Facing::Outer) => {
                        graph.add_edge(
                            node_to_index[&Node { pos: from, level }],
                            node_to_index[&Node {
                                pos: to,
                                level: level + 1,
                            }],
                            1,
                        );
                        graph.add_edge(
                            node_to_index[&Node {
                                pos: to,
                                level: level + 1,
                            }],
                            node_to_index[&Node { pos: from, level }],
                            1,
                        );
                    }
                    (Facing::Outer, Facing::Inner) => {
                        graph.add_edge(
                            node_to_index[&Node {
                                pos: from,
                                level: level + 1,
                            }],
                            node_to_index[&Node { pos: to, level }],
                            1,
                        );
                        graph.add_edge(
                            node_to_index[&Node { pos: to, level }],
                            node_to_index[&Node {
                                pos: from,
                                level: level + 1,
                            }],
                            1,
                        );
                    }
                }
            }
        }
    }
    let start = node_to_index[&start];
    let end = node_to_index[&end];
    Some(graph.dijkstra_distances(start, Some(end))[&end])
}
fn find_portals(grid: &DenseGrid<Tile>) -> HashMap<String, Vec<Position<usize>>> {
    let mut map = HashMap::new();
    for (i, t) in grid.iter().enumerate() {
        if let Tile::Char(x) = t {
            for dir in Direction4::all_dirs() {
                if let Some((j, Tile::Char(y))) = grid.get_dir8(i, dir.into())
                    && let Some((k, Tile::Path)) = grid.get_dir8(j, dir.into())
                {
                    let s = match dir {
                        Direction4::North | Direction4::West => [y, x],
                        Direction4::South | Direction4::East => [x, y],
                    };
                    map.entry(s.into_iter().collect())
                        .and_modify(|e: &mut Vec<Position<usize>>| e.push(k.to_position(grid)))
                        .or_insert_with(|| vec![k.to_position(grid)]);
                    break;
                }
            }
        }
    }
    debug_assert!(
        map.iter()
            .filter(|(k, _)| *k != "AA" && *k != "ZZ")
            .all(|(_, v)| v.len() == 2)
    );
    debug_assert!(
        map.iter()
            .filter(|(k, _)| *k == "AA" || *k == "ZZ")
            .all(|(_, v)| v.len() == 1)
    );
    map
}
fn parse(input: &str) -> DenseGrid<Tile> {
    let width = input.lines().nth(1).unwrap().chars().count();
    let height = input.lines().count();
    let mut grid = DenseGrid::new(width, height, Tile::Air);
    for (xx, c) in input.lines().next().unwrap().chars().rev().enumerate() {
        grid.set(
            Position {
                x: width - 1 - xx,
                y: 0,
            },
            c.into(),
        );
    }
    for (y, l) in input.lines().enumerate().skip(1) {
        for (x, c) in l.chars().enumerate() {
            grid.set(Position { x, y }, c.into());
        }
    }
    grid
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(77));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(686));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(396));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(8_384));
    }
}
