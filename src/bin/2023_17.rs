use std::collections::HashMap;

use all_aoc::helper::{
    graph::{Graph as _, Special, WithWeights as _},
    grid::{Grid as _, dense::DenseGrid, index::GridIndex as _},
    position::{Direction4, Position},
};

all_aoc::solution!(17, 2023);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    pos: Position<usize>,
    dir: Direction4,
    steps_taken: u8,
}
pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse(input);
    let mut map = HashMap::with_capacity(grid.len() * 4 * 3 * 2);
    for pos in 0..grid.len() {
        let pos = pos.to_position(&grid);
        for dir in Direction4::all_dirs() {
            for steps_taken in 1..=3 {
                let n = map.len();
                map.insert(
                    Node {
                        pos,
                        dir,
                        steps_taken,
                    },
                    n,
                );
            }
        }
    }
    let x = map.len();
    let start_node = Node {
        pos: Position { x: 0, y: 0 },
        dir: Direction4::East,
        steps_taken: 0,
    };
    map.insert(start_node, x);
    let mut graph = Special::new();
    graph.add_edge(
        x,
        map[&Node {
            pos: Position { x: 1, y: 0 },
            dir: Direction4::East,
            steps_taken: 1,
        }],
        grid[Position { x: 1, y: 0 }],
    );
    graph.add_edge(
        x,
        map[&Node {
            pos: Position { x: 0, y: 1 },
            dir: Direction4::South,
            steps_taken: 1,
        }],
        grid[Position { x: 0, y: 1 }],
    );
    for (node, i) in &map {
        if node.steps_taken != 3
            && let Some((x, val)) = grid.get_dir8(node.pos, node.dir.into())
        {
            let n = Node {
                pos: x.to_position(&grid),
                dir: node.dir,
                steps_taken: node.steps_taken + 1,
            };
            let n = map[&n];
            graph.add_edge(*i, n, *val);
        }
        for dir in [node.dir.turn_left(), node.dir.turn_right()] {
            if let Some((x, val)) = grid.get_dir8(node.pos, dir.into()) {
                let n = Node {
                    pos: x.to_position(&grid),
                    dir,
                    steps_taken: 1,
                };
                let n = map[&n];
                graph.add_edge(*i, n, *val);
            }
        }
    }

    let end = Position {
        x: grid.width() - 1,
        y: grid.height() - 1,
    };
    let rev_map = map.iter().map(|(k, v)| (*v, *k)).collect::<HashMap<_, _>>();
    let map_1 = graph.dijkstra_distances(map[&start_node], None);
    Some(
        map_1
            .into_iter()
            .filter(|(k, _v)| rev_map[k].pos == end)
            .map(|(_, v)| v)
            .min()
            .unwrap(),
    )
}
pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse(input);
    let mut map = HashMap::with_capacity(grid.len() * 4 * 10 * 2);
    for pos in 0..grid.len() {
        let pos = pos.to_position(&grid);
        for dir in Direction4::all_dirs() {
            for steps_taken in 1..=10 {
                let n = map.len();
                map.insert(
                    Node {
                        pos,
                        dir,
                        steps_taken,
                    },
                    n,
                );
            }
        }
    }
    let x = map.len();
    let start_node = Node {
        pos: Position { x: 0, y: 0 },
        dir: Direction4::East,
        steps_taken: 0,
    };
    map.insert(start_node, x);

    let mut graph = Special::new();
    graph.add_edge(
        x,
        map[&Node {
            pos: Position { x: 1, y: 0 },
            dir: Direction4::East,
            steps_taken: 1,
        }],
        grid[Position { x: 1, y: 0 }],
    );
    graph.add_edge(
        x,
        map[&Node {
            pos: Position { x: 0, y: 1 },
            dir: Direction4::South,
            steps_taken: 1,
        }],
        grid[Position { x: 0, y: 1 }],
    );
    for (node, i) in &map {
        if node.steps_taken < 10
            && let Some((x, val)) = grid.get_dir8(node.pos, node.dir.into())
        {
            let n = Node {
                pos: x.to_position(&grid),
                dir: node.dir,
                steps_taken: node.steps_taken + 1,
            };
            let n = map[&n];
            graph.add_edge(*i, n, *val);
        }
        if node.steps_taken >= 4 {
            for dir in [node.dir.turn_left(), node.dir.turn_right()] {
                if let Some((x, val)) = grid.get_dir8(node.pos, dir.into()) {
                    let n = Node {
                        pos: x.to_position(&grid),
                        dir,
                        steps_taken: 1,
                    };
                    let n = map[&n];
                    graph.add_edge(*i, n, *val);
                }
            }
        }
    }

    let end = Position {
        x: grid.width() - 1,
        y: grid.height() - 1,
    };
    let rev_map = map.iter().map(|(k, v)| (*v, *k)).collect::<HashMap<_, _>>();
    let map = graph.dijkstra_distances(map[&start_node], None);

    Some(
        map.into_iter()
            .filter(|(k, _v)| rev_map[k].pos == end && (4..=10).contains(&rev_map[k].steps_taken))
            .map(|(_, v)| v)
            .min()
            .unwrap(),
    )
}
fn parse(input: &str) -> DenseGrid<u32> {
    DenseGrid::from_iter_iter(
        input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap())),
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let result = part_one(
            all_aoc::cli::read_examples_file(DAY)
                .split_once("\n\n")
                .unwrap()
                .0,
        );
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(907));
    }

    #[test]
    fn test_part_two() {
        let binding = all_aoc::cli::read_examples_file(DAY);
        let input = binding.split_once("\n\n").unwrap();
        assert_eq!(part_two(input.0), Some(94));
        assert_eq!(part_two(input.1), Some(71));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_057));
    }
}
