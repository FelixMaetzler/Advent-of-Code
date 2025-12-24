use std::collections::HashMap;

use all_aoc::helper::graph::{Graph, Special, WithWeights as _};

all_aoc::solution!(11, 2025);

pub fn part_one(input: &str) -> Option<u64> {
    let (map, graph) = build_graph(input);
    debug_assert!(graph.is_dag(), "Graph has to be a DAG");
    let start = map["you"];
    let end = map["out"];
    Some(count_paths_between(&graph, start, end))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (map, graph) = build_graph(input);
    debug_assert!(graph.is_dag(), "Graph has to be a DAG");
    let start = map["svr"];
    let end = map["out"];
    let p1 = map["dac"];
    let p2 = map["fft"];
    let first = count_paths_between(&graph, start, p1)
        * count_paths_between(&graph, p1, p2)
        * count_paths_between(&graph, p2, end);
    let second = count_paths_between(&graph, start, p2)
        * count_paths_between(&graph, p2, p1)
        * count_paths_between(&graph, p1, end);

    Some(first + second)
}
fn build_graph(input: &str) -> (HashMap<&str, usize>, Special<u8>) {
    let mut map = HashMap::new();
    let mut graph = Special::new();
    for (from, tos) in parse(input) {
        let len = map.len();
        map.entry(from).or_insert(len);
        let from = map[from];
        for to in tos {
            let len = map.len();
            map.entry(to).or_insert(len);
            let to = map[to];
            graph.add_edge(from, to, 1);
        }
    }
    (map, graph)
}
fn count_paths_between(graph: &impl Graph, start: usize, end: usize) -> u64 {
    count(graph, start, end, &mut HashMap::new())
}
fn count(graph: &impl Graph, current: usize, end: usize, cache: &mut HashMap<usize, u64>) -> u64 {
    if current == end {
        return 1;
    }
    if let Some(x) = cache.get(&current) {
        return *x;
    }
    let c = graph
        .outgoing(current)
        .map(|n| count(graph, n, end, cache))
        .sum();
    cache.insert(current, c);
    c
}
fn parse(input: &str) -> impl Iterator<Item = (&str, impl Iterator<Item = &str>)> {
    input
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|line| (line.0, line.1.split_ascii_whitespace()))
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
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(788));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            all_aoc::cli::read_examples_file(DAY)
                .split_once("\n\n")
                .unwrap()
                .1,
        );
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(316_291_887_968_000));
    }
}
