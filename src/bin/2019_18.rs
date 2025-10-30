use core::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use all_aoc::helper::{
    graph::{Graph as _, Special, WithWeights as _},
    grid::{Grid as _, dense::DenseGrid, index::GridIndex as _},
    position::Direction8,
};

all_aoc::solution!(18, 2019);
#[derive(Debug, Clone, Copy)]
enum Tile {
    Wall,
    Open,
    Door(u8),
    Key(u8),
    Entrance,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Wall,
            '.' => Self::Open,
            '@' => Self::Entrance,
            c @ ('a'..='z') => Self::Key(c as u8 - b'a'),
            c @ ('A'..='Z') => Self::Door(c as u8 - b'A'),
            c => unimplemented!("Unkown char: {c}"),
        }
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse(input);
    let start = grid
        .iter()
        .enumerate()
        .find(|(_, t)| matches!(t, Tile::Entrance))
        .unwrap()
        .0;

    let graph = grid_to_graph(&grid);

    let d = shortest_path_keydoor(
        &graph,
        &grid,
        start,
        grid.iter()
            .filter(|t| matches!(t, Tile::Key(_)))
            .count()
            .try_into()
            .unwrap(),
    )
    .unwrap();
    Some(d)
}
fn grid_to_graph(grid: &DenseGrid<Tile>) -> Special<u32> {
    let interesting_positions = grid
        .iter()
        .enumerate()
        .filter(|(_, t)| matches!(t, Tile::Key(_) | Tile::Door(_) | Tile::Entrance))
        .map(|(i, _)| i)
        .collect::<HashSet<_>>();

    let mut graph = Special::new();
    for &pos in &interesting_positions {
        for (end, dist) in bfs_reachable(pos, grid) {
            graph.add_edge(pos, end, dist);
            graph.add_edge(end, pos, dist);
        }
    }
    graph
}
fn shortest_path_keydoor(
    graph: &Special<u32>,
    node_type: &DenseGrid<Tile>,
    start: usize,
    key_count: u8,
) -> Option<u32> {
    let mut heap = BinaryHeap::new();
    let mut best: HashMap<(usize, u64), u32> = HashMap::new();

    heap.push(Reverse((0, start, 0_u64))); // (cost, pos, keys)
    best.insert((start, 0), 0);

    while let Some(Reverse((cost, pos, keys))) = heap.pop() {
        if keys.count_ones() == key_count.into() {
            return Some(cost);
        }

        if let Some(&best_cost) = best.get(&(pos, keys))
            && cost > best_cost
        {
            continue;
        }

        for next in graph.outgoing(pos) {
            let dist = graph.weight(pos, next).unwrap();
            let tile = node_type[next];
            let mut new_keys = keys;

            match tile {
                Tile::Door(d) if keys & (1 << d) == 0 => continue,
                Tile::Key(k) => {
                    new_keys |= 1 << k;
                }
                Tile::Wall | Tile::Open | Tile::Door(_) | Tile::Entrance => {}
            }

            let new_cost = cost + dist;
            if new_cost < *best.get(&(next, new_keys)).unwrap_or(&u32::MAX) {
                best.insert((next, new_keys), new_cost);
                heap.push(Reverse((new_cost, next, new_keys)));
            }
        }
    }

    None
}
fn bfs_reachable(start: usize, grid: &DenseGrid<Tile>) -> Vec<(usize, u32)> {
    let mut reachable = Vec::new();
    let mut queue = VecDeque::new();
    let mut visited = vec![false; grid.len()];

    queue.push_back((start, 0));
    visited[start] = true;

    while let Some((pos, dist)) = queue.pop_front() {
        for (neighbor_pos, tile) in grid.get_neigbors4(pos) {
            let neighbor_idx = neighbor_pos.to_flat_index(grid);
            if visited[neighbor_idx] {
                continue;
            }

            match tile {
                Tile::Wall => {}
                Tile::Open => {
                    visited[neighbor_idx] = true;
                    queue.push_back((neighbor_idx, dist + 1));
                }
                Tile::Door(_) | Tile::Key(_) | Tile::Entrance => {
                    reachable.push((neighbor_idx, dist + 1));
                    visited[neighbor_idx] = true;
                }
            }
        }
    }

    reachable
}
pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = parse(input);
    let start = grid
        .iter()
        .enumerate()
        .find(|(_, t)| matches!(t, Tile::Entrance))
        .unwrap()
        .0;

    grid[start] = Tile::Wall;
    for d in Direction8::all_dirs() {
        let i = start.dir(d, &grid).unwrap().to_flat_index(&grid);
        grid[i] = Tile::Wall;
    }
    let starts = [
        Direction8::NorthWest,
        Direction8::NorthEast,
        Direction8::SouthWest,
        Direction8::SouthEast,
    ]
    .into_iter()
    .map(|d| {
        let idx = start.dir(d, &grid).unwrap().to_flat_index(&grid);
        grid[idx] = Tile::Entrance;
        idx
    })
    .collect::<Vec<_>>();

    let graph = grid_to_graph(&grid);

    let key_count = grid
        .iter()
        .filter(|t| matches!(t, Tile::Key(_)))
        .count()
        .try_into()
        .unwrap();

    shortest_path_multi(&graph, &grid, starts.try_into().unwrap(), key_count)
}

fn shortest_path_multi(
    graph: &Special<u32>,
    node_type: &DenseGrid<Tile>,
    starts: [usize; 4],
    key_count: u8,
) -> Option<u32> {
    let mut heap = BinaryHeap::new();
    let mut best: HashMap<([usize; 4], u64), u32> = HashMap::new();

    heap.push(Reverse((0, starts, 0_u64))); // (cost, pos[], keys)
    best.insert((starts, 0), 0);

    while let Some(Reverse((cost, pos, keys))) = heap.pop() {
        if keys.count_ones() == key_count.into() {
            return Some(cost);
        }

        if let Some(&best_cost) = best.get(&(pos, keys))
            && cost > best_cost
        {
            continue;
        }

        for i in 0..4 {
            let cur = pos[i];
            for next in graph.outgoing(cur) {
                let dist = graph.weight(cur, next).unwrap();
                let tile = node_type[next];
                let mut new_keys = keys;

                match tile {
                    Tile::Door(d) if keys & (1 << d) == 0 => continue,
                    Tile::Key(k) => new_keys |= 1 << k,
                    Tile::Wall | Tile::Open | Tile::Door(_) | Tile::Entrance => {}
                }

                let mut new_pos = pos;
                new_pos[i] = next;

                let new_cost = cost + dist;
                if new_cost < *best.get(&(new_pos, new_keys)).unwrap_or(&u32::MAX) {
                    best.insert((new_pos, new_keys), new_cost);
                    heap.push(Reverse((new_cost, new_pos, new_keys)));
                }
            }
        }
    }

    None
}
fn parse(input: &str) -> DenseGrid<Tile> {
    DenseGrid::from_string(input)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let results = all_aoc::cli::read_examples_file(DAY)
            .split("\n\n")
            .take(5)
            .map(part_one)
            .collect::<Vec<_>>();
        assert_eq!(results, [Some(8), Some(86), Some(132), Some(136), Some(81)]);
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(4_118));
    }

    #[test]
    fn test_part_two() {
        let results = all_aoc::cli::read_examples_file(DAY)
            .split("\n\n")
            .skip(5)
            .map(part_two)
            .collect::<Vec<_>>();
        assert_eq!(results, [Some(8), Some(24), Some(32), Some(72)]);
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_828));
    }
}
