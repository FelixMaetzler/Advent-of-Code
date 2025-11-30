use std::collections::HashMap;

all_aoc::solution!(3, 2018);
struct Claim {
    id: usize,
    x_diff: usize,
    y_diff: usize,
    width: usize,
    height: usize,
}
pub fn part_one(input: &str) -> Option<usize> {
    let vec = parse(input);
    let mut map = HashMap::new();
    for claim in vec {
        for x in claim.x_diff..claim.x_diff + claim.width {
            for y in claim.y_diff..claim.y_diff + claim.height {
                map.entry((x, y)).and_modify(|e| *e += 1).or_insert(1);
            }
        }
    }
    Some(map.values().filter(|&&v| v >= 2).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut map = HashMap::new();
    for claim in parse(input) {
        for x in claim.x_diff..claim.x_diff + claim.width {
            for y in claim.y_diff..claim.y_diff + claim.height {
                map.entry((x, y)).and_modify(|e| *e += 1).or_insert(1);
            }
        }
    }
    for claim in parse(input) {
        let mut flag = false;
        for x in claim.x_diff..claim.x_diff + claim.width {
            for y in claim.y_diff..claim.y_diff + claim.height {
                if map[&(x, y)] != 1 {
                    flag = true;
                }
            }
        }
        if !flag {
            return Some(claim.id);
        }
    }
    unreachable!()
}
fn parse(input: &str) -> impl Iterator<Item = Claim> {
    input.lines().map(|line| {
        let mut split = line.split(' ');
        let id = split.next().unwrap()[1..].parse().unwrap();
        split.next().unwrap();
        let (x_diff, y_diff) = split
            .next()
            .unwrap()
            .trim_end_matches(':')
            .split_once(',')
            .unwrap();
        let x_diff = x_diff.parse().unwrap();
        let y_diff = y_diff.parse().unwrap();
        let (width, height) = split.next().unwrap().split_once('x').unwrap();
        let width = width.parse().unwrap();
        let height = height.parse().unwrap();
        Claim {
            id,
            x_diff,
            y_diff,
            width,
            height,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(110_383));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(129));
    }
}
