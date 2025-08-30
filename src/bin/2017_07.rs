use std::collections::HashMap;

use all_aoc::helper::{
    graph::{Graph as _, Special, WithWeights as _},
    misc::count_occurrences,
};

all_aoc::solution!(7, 2017);

pub fn part_one(input: &str) -> Option<String> {
    let (index_map, _, graph) = parse(input);
    let n = (0..graph.nodes_count())
        .find(|n| graph.incoming(*n).count() == 0 && graph.outgoing(*n).count() > 0)
        .unwrap();
    Some((*index_map.iter().find(|(_, v)| **v == n).unwrap().0).to_owned())
}

pub fn part_two(input: &str) -> Option<u64> {
    let (index_map, number_map, graph) = parse(input);
    let start = part_one(input).unwrap();
    Some(inbalance(
        index_map[start.as_str()],
        u64::MAX, // doesnt matter in the first call
        &number_map,
        &graph,
    ))
}

fn inbalance(
    n: usize,
    should_be: u64,
    number_map: &HashMap<usize, u64>,
    graph: &Special<u8>,
) -> u64 {
    let weights = graph
        .outgoing(n)
        .map(|i| (calc_weight(i, number_map, graph), i))
        .collect::<Vec<_>>();
    if weights.windows(2).all(|w| w[0].0 == w[1].0) {
        should_be - weights.iter().map(|i| i.0).sum::<u64>()
    } else {
        let c = count_occurrences(weights.iter().map(|(w, _)| *w));
        debug_assert_eq!(c.len(), 2);
        let more = c.iter().find(|(_, v)| **v > 1).unwrap().0;
        let less = c.iter().find(|(_, v)| **v == 1).unwrap().0;
        let index = weights.iter().find(|(w, _)| w == less).unwrap().1;
        inbalance(index, *more, number_map, graph)
    }
}
fn calc_weight(n: usize, number_map: &HashMap<usize, u64>, graph: &Special<u8>) -> u64 {
    number_map[&n]
        + graph
            .outgoing(n)
            .map(|i| calc_weight(i, number_map, graph))
            .sum::<u64>()
}
fn parse(input: &str) -> (HashMap<&str, usize>, HashMap<usize, u64>, Special<u8>) {
    let index_map = input
        .lines()
        .enumerate()
        .map(|(i, l)| (l.split_ascii_whitespace().next().unwrap(), i))
        .collect::<HashMap<_, _>>();

    let number_map = input
        .lines()
        .map(|l| {
            (
                index_map[l.split_ascii_whitespace().next().unwrap()],
                l.split_ascii_whitespace()
                    .nth(1)
                    .unwrap()
                    .trim_end_matches(')')
                    .trim_start_matches('(')
                    .parse()
                    .unwrap(),
            )
        })
        .collect::<HashMap<_, _>>();

    let graph = {
        let index_map_ref = &index_map;
        Special::from_edges(
            input
                .lines()
                .enumerate()
                .filter(|(_, l)| l.contains("->"))
                .flat_map(|(i, l)| {
                    l.split_once("-> ")
                        .unwrap()
                        .1
                        .split(", ")
                        .map(move |w| (i, index_map_ref[w], 1))
                }),
        )
    };

    (index_map, number_map, graph)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some("tknk".to_owned()));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some("bpvhwhh".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(60));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(256));
    }
}
