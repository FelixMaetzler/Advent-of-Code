use std::collections::{HashMap, HashSet};

use all_aoc::helper::position::Position;

all_aoc::solution!(8, 2024);

pub fn part_one(input: &str) -> Option<usize> {
    let (map, pos_max) = parse(input);
    let mut set = HashSet::new();
    for (_, vec) in map {
        for i in 0..vec.len() - 1 {
            for j in i + 1..vec.len() {
                let p1 = vec[i];
                let p2 = vec[j];
                let diff = p2 - p1;
                set.insert(p1 - diff);
                set.insert(p2 + diff);
            }
        }
    }
    let set = set
        .into_iter()
        .filter(|pos| (0..pos_max.x).contains(&pos.x) && (0..pos_max.y).contains(&pos.y));

    Some(set.count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (map, pos_max) = parse(input);
    let mut set = HashSet::new();
    for (_, vec) in map {
        for i in 0..vec.len() - 1 {
            for j in i + 1..vec.len() {
                let p1 = vec[i];
                let p2 = vec[j];
                let diff = p2 - p1;
                for i in 0.. {
                    let new_pos = p1 - i * diff;
                    set.insert(new_pos);
                    if new_pos.x < 0 || new_pos.y < 0 {
                        break;
                    }
                }
                for i in 0.. {
                    let new_pos = p2 + i * diff;
                    set.insert(new_pos);
                    if new_pos.x >= pos_max.x || new_pos.y >= pos_max.y {
                        break;
                    }
                }
            }
        }
    }
    let set = set
        .into_iter()
        .filter(|pos| (0..pos_max.x).contains(&pos.x) && (0..pos_max.y).contains(&pos.y));

    Some(set.count())
}
fn parse(input: &str) -> (HashMap<char, Vec<Position<i32>>>, Position<i32>) {
    let mut map = HashMap::new();
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c != '.' {
                let x = x as i32;
                let y = y as i32;
                map.entry(c)
                    .and_modify(|v: &mut Vec<Position<i32>>| v.push(Position { x, y }))
                    .or_insert(vec![Position { x, y }]);
            }
        }
    }
    let y_max = input.lines().count() as i32;
    let x_max = input.lines().next().unwrap().chars().count() as i32;
    (map, Position { x: x_max, y: y_max })
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(228));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(34));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(766));
    }
}
