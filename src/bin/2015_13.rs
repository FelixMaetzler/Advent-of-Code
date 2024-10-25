use std::collections::{HashMap, HashSet};

use all_aoc::helper::permutations::generate_permutations;

all_aoc::solution!(13, 2015);

pub fn part_one(input: &str) -> Option<i32> {
    let map = parse(input);
    let set: HashSet<_> = map.keys().flat_map(|(k1, k2)| [k1, k2]).cloned().collect();
    let vec: Vec<_> = set.into_iter().collect();
    let perms = generate_permutations(&vec);
    perms
        .into_iter()
        .map(|order| calc_change(&order, &map))
        .max()
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut map = parse(input);
    let me = "FelixMe".to_string();
    let set: HashSet<_> = map.keys().flat_map(|(k1, k2)| [k1, k2]).cloned().collect();
    let mut vec: Vec<_> = set.into_iter().collect();
    for x in &vec {
        map.insert((me.clone(), x.clone()), 0);
        map.insert((x.clone(), me.clone()), 0);
    }
    vec.push(me);
    let vec = vec;
    let map = map;
    let perms = generate_permutations(&vec);
    perms
        .into_iter()
        .map(|order| calc_change(&order, &map))
        .max()
}
fn calc_change(order: &[String], map: &HashMap<(String, String), i32>) -> i32 {
    let mut order = order.to_vec();
    order.push(order[0].clone());
    let order = order;
    order
        .windows(2)
        .map(|x| (x[0].clone(), x[1].clone()))
        .map(|(x1, x2)| map.get(&(x1.clone(), x2.clone())).unwrap() + map.get(&(x2, x1)).unwrap())
        .sum()
}
fn parse(input: &str) -> HashMap<(String, String), i32> {
    input
        .lines()
        .map(|line| {
            let vec = line.split(' ').collect::<Vec<_>>();
            let s1 = vec[0].to_string();
            let s2 = vec[10].trim_end_matches('.').to_string();
            let mul = match vec[2] {
                "gain" => 1,
                "lose" => -1,
                x => unreachable!("should be impossible but got {x}"),
            };
            let i = mul * vec[3].parse::<i32>().unwrap();
            ((s1, s2), i)
        })
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(330));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(733));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(725));
    }
}
