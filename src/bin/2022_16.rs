use std::collections::{HashMap, HashSet};

use all_aoc::helper::{
    bitmask::Bitmask as _,
    graph::{Graph as _, Special, WithWeights as _},
    permutations::IteratorCombinator as _,
};

all_aoc::solution!(16, 2022);

struct Node {
    curr: usize,
    time_remaining: u32,
    total_relief: u32,
    valves_opened: HashSet<usize>,
}
pub fn part_one(input: &str) -> Option<u32> {
    let map = helper(input, 30);
    Some(*map.values().max().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = helper(input, 26);
    Some(
        map.keys()
            .combinations(2)
            .filter(|c| (c[0] & c[1]) == 0)
            .map(|c| map[c[0]] + map[c[1]])
            .max()
            .unwrap(),
    )
}
fn bitmask_from_hashset(set: &HashSet<usize>) -> usize {
    let mut ret = 0;
    for x in set {
        ret.set_bit(*x, true);
    }
    ret
}
fn helper(input: &str, time: u32) -> HashMap<usize, u32> {
    let (map, flow_rate, graph) = parse(input);
    let all_pairs_shortest_distances = graph.all_pairs_shortest_distances();
    let start_index = map["AA"];
    let mut nodes = graph.nodes().collect::<HashSet<_>>();
    for i in flow_rate.iter().filter(|(_, v)| **v == 0).map(|(k, _)| k) {
        nodes.remove(i);
    }
    let start = Node {
        curr: start_index,
        time_remaining: time,
        total_relief: 0,
        valves_opened: HashSet::new(),
    };
    let mut queue = vec![start];

    let mut map = HashMap::new();
    while let Some(n) = queue.pop() {
        for next in nodes.difference(&n.valves_opened) {
            let travel_time = all_pairs_shortest_distances[&(n.curr, *next)];
            if travel_time < n.time_remaining {
                let new_time_remaining = n.time_remaining - travel_time - 1;
                let new_total_relief = n.total_relief + new_time_remaining * flow_rate[next];
                let mut new_set = n.valves_opened.clone();
                new_set.insert(*next);
                map.entry(bitmask_from_hashset(&new_set))
                    .and_modify(|old| {
                        *old = new_total_relief.max(*old);
                    })
                    .or_insert(new_total_relief);
                let new_node = Node {
                    curr: *next,
                    time_remaining: new_time_remaining,
                    total_relief: new_total_relief,
                    valves_opened: new_set,
                };

                queue.push(new_node);
            }
        }
    }
    map
}
fn parse(input: &str) -> (HashMap<&str, usize>, HashMap<usize, u32>, Special<u32>) {
    let map = input
        .lines()
        .enumerate()
        .map(|(i, l)| (l.split_ascii_whitespace().nth(1).unwrap(), i))
        .collect::<HashMap<_, _>>();
    let flow_rate = input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            (
                i,
                l.split_ascii_whitespace()
                    .nth(4)
                    .unwrap()
                    .trim_start_matches("rate=")
                    .trim_end_matches(';')
                    .parse()
                    .unwrap(),
            )
        })
        .collect();
    let mut graph = Special::new();
    for l in input.lines() {
        let v = l.split_ascii_whitespace().collect::<Vec<_>>();
        let from = map[v[1]];
        for to in &v[9..] {
            let to = map[to.trim_end_matches(',')];
            graph.add_edge(from, to, 1);
            graph.add_edge(to, from, 1);
        }
    }
    (map, flow_rate, graph)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(1_651));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_673));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(1_707));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(2_343));
    }
}
