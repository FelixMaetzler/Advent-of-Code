use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    iter,
};

use all_aoc::helper::{
    graph::{GraphWithWeights, build_graph4_special},
    grid::{Grid, grid_index::GridIndex, sparse_grid::SparseGrid},
    intcode::{InputMode, IntInteger, Intcode, Return},
    position::{Direction4, Position},
};

all_aoc::solution!(15, 2019);
#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Air,
    Wall,
    OygenSystem,
    Unknown,
}
impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Air => write!(f, "."),
            Self::Wall => write!(f, " "),
            Self::OygenSystem => write!(f, "O"),
            Self::Unknown => write!(f, " "),
        }
    }
}
fn to_number(dir: Direction4) -> IntInteger {
    match dir {
        Direction4::North => 1,
        Direction4::East => 4,
        Direction4::West => 3,
        Direction4::South => 2,
    }
}
const START: Position<usize> = Position {
    x: usize::MAX >> (usize::BITS * 2 / 3),
    y: usize::MAX >> (usize::BITS * 2 / 3),
};
pub fn part_one(input: &str) -> Option<u32> {
    let maze = generate_maze(parse(input));
    let grid = SparseGrid::from_it(maze.iter().map(|(k, v)| ((k.y as usize, k.x as usize), *v)));
    let graph = build_graph4_special(&grid, |curr, n| {
        (matches!(curr, Tile::Air | Tile::OygenSystem)
            && matches!(n, Tile::Air | Tile::OygenSystem))
        .then_some(1)
    });
    let start = START.to_flat_index(&grid);
    debug_assert_eq!(
        grid.iter_all()
            .filter(|(_, t)| **t == Tile::OygenSystem)
            .count(),
        1
    );
    let end = grid
        .iter_all()
        .find(|(_, t)| **t == Tile::OygenSystem)
        .map(|(i, _)| i)
        .unwrap();
    let erg = graph.dijkstra_distances(start, None);
    Some(*erg.get(end).unwrap())
}

fn generate_maze(mut computer: Intcode) -> HashMap<Position<i32>, Tile> {
    computer.halt_at_output(true);
    let mut map = HashMap::new();
    let mut curr = Position {
        x: START.x as i32,
        y: START.y as i32,
    };
    map.insert(curr, Tile::Air);
    fill_up_with_unknown(&mut map);
    let mut path = vec![];

    'outer: while map.values().any(|t| *t == Tile::Unknown) {
        let mut no_found = true;
        for dir in Direction4::all_dirs() {
            let next_pos = curr.direction(dir);
            if map.get(&next_pos) == Some(&Tile::Unknown) {
                no_found = false;
                computer.set_inputs(iter::once(to_number(dir)), InputMode::Replace);
                let erg = computer.execute();
                debug_assert_eq!(erg, Return::NewOutput);
                let output = *computer.get_outputs().last().unwrap();
                match output {
                    0 => {
                        map.insert(next_pos, Tile::Wall);
                    }
                    x @ (1 | 2) => {
                        curr = next_pos;
                        map.insert(
                            next_pos,
                            match x {
                                1 => Tile::Air,
                                2 => Tile::OygenSystem,
                                _ => unreachable!(),
                            },
                        );
                        path.push(dir);
                        fill_up_with_unknown(&mut map);
                        continue 'outer;
                    }
                    _ => unreachable!(),
                }
            }
        }
        if no_found {
            let dir = path.pop().unwrap().opposite();
            computer.set_inputs(iter::once(to_number(dir)), InputMode::Replace);
            let erg = computer.execute();
            debug_assert_eq!(erg, Return::NewOutput);
            let output = *computer.get_outputs().last().unwrap();
            debug_assert!(matches!(output, 1 | 2));
            curr = curr.direction(dir);
        }
    }

    map
}
fn fill_up_with_unknown(map: &mut HashMap<Position<i32>, Tile>) {
    let empty = map
        .iter()
        .filter(|(_, v)| matches!(v, Tile::Air | Tile::OygenSystem))
        .map(|(k, _)| k)
        .cloned()
        .collect::<Vec<_>>();
    for p in empty {
        for dir in Direction4::all_dirs() {
            map.entry(p.direction(dir)).or_insert(Tile::Unknown);
        }
    }
}
pub fn part_two(input: &str) -> Option<u32> {
    let maze = generate_maze(parse(input));
    let grid = SparseGrid::from_it(maze.iter().map(|(k, v)| ((k.y as usize, k.x as usize), *v)));
    let mut grid = SparseGrid::from_it(grid.iter_all().flat_map(|(i, v)| {
        match v {
            Tile::Air => Some(false),
            Tile::Wall => None,
            Tile::OygenSystem => Some(true),
            Tile::Unknown => None,
        }
        .map(|x| (i.to_coordinates(&grid), x))
    }));
    let mut cnt = 0;
    while !grid.iter_all().all(|(_, v)| *v) {
        cnt += 1;
        let visit = grid
            .iter_all()
            .filter(|&(_, v)| *v)
            .map(|(i, _)| i)
            .flat_map(|s| grid.get_neigbors4(*s))
            .map(|x| x.0.to_flat_index(&grid))
            .collect::<HashSet<_>>();
        visit.into_iter().for_each(|i| {
            grid.set(i, true);
        });
    }
    Some(cnt)
}
fn parse(input: &str) -> Intcode {
    Intcode::new(input.split(',').map(|n| n.parse().unwrap()).collect())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(204));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(340));
    }
}
