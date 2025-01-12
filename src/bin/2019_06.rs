use std::collections::HashMap;

use all_aoc::helper::graph::{Graph, GraphWithWeights, SpecialGraph};

all_aoc::solution!(6, 2019);

pub fn part_one(input: &str) -> Option<usize> {
    let (vec, _) = translate_indexes(input);
    let mut graph = SpecialGraph::new();
    for (x, y) in vec {
        graph.add_edge(y, x, 1);
    }
    Some(graph.nodes().map(|i| orbits(&graph, i)).sum())
}
pub fn part_two(input: &str) -> Option<u32> {
    let (vec, map) = translate_indexes(input);
    let mut graph = SpecialGraph::new();
    for (x, y) in vec {
        graph.add_edge(x, y, 1);
        graph.add_edge(y, x, 1);
    }
    let start = *map.get("YOU").unwrap();
    let end = *map.get("SAN").unwrap();
    let dist = graph.dijkstra_distances(start, Some(end));
    Some(*dist.get(&end).unwrap() - 2)
}
fn translate_indexes(input: &str) -> (Vec<(usize, usize)>, HashMap<&str, usize>) {
    let vec = parse(input);
    let mut map = HashMap::new();
    let mut c = 0;
    let mut new_vec = Vec::with_capacity(vec.len());
    for (x, y) in vec {
        map.entry(x).or_insert({
            c += 1;
            c - 1
        });
        map.entry(y).or_insert({
            c += 1;
            c - 1
        });
        new_vec.push((*map.get(x).unwrap(), *map.get(y).unwrap()));
    }
    (new_vec, map)
}
fn orbits(graph: &SpecialGraph<u8>, node: usize) -> usize {
    graph.outgoing(node).map(|i| 1 + orbits(graph, i)).sum()
}

fn parse(input: &str) -> Vec<(&str, &str)> {
    input.lines().map(|l| l.split_once(')').unwrap()).collect()
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
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(621_125));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            all_aoc::cli::read_examples_file(DAY)
                .split_once("\n\n")
                .unwrap()
                .1,
        );
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(550));
    }
}
