all_aoc::solution!(24, 2017);

pub fn part_one(input: &str) -> Option<u32> {
    let vec = parse(input);

    Some(recursive_part1(&vec, 0).unwrap())
}
fn recursive_part1(available: &[(u32, u32)], last_ending: u32) -> Option<u32> {
    let it = available
        .iter()
        .filter(|(n1, n2)| *n1 == last_ending || *n2 == last_ending);
    let mut max = vec![];
    for port in it {
        let mut copy = available.to_vec();
        copy.retain(|t| t != port);
        let last = if port.0 == last_ending {
            port.1
        } else {
            port.0
        };
        let val = recursive_part1(&copy, last);
        let val = val.unwrap_or(0);

        max.push(val + port.0 + port.1);
    }
    max.into_iter().max()
}
pub fn part_two(input: &str) -> Option<u32> {
    let vec = parse(input);

    Some(recursive_part2(&vec, 0).1.unwrap())
}
fn recursive_part2(available: &[(u32, u32)], last_ending: u32) -> (u32, Option<u32>) {
    let it = available
        .iter()
        .filter(|(n1, n2)| *n1 == last_ending || *n2 == last_ending);
    let mut max = vec![];
    let mut lengths = vec![];
    for port in it {
        let mut copy = available.to_vec();
        copy.retain(|t| t != port);
        let last = if port.0 == last_ending {
            port.1
        } else {
            port.0
        };
        let (length, val) = recursive_part2(&copy, last);
        let val = val.unwrap_or(0);
        lengths.push(length);
        max.push(val + port.0 + port.1);
    }
    let index = lengths.iter().enumerate().max_by(|n1, n2| n1.1.cmp(n2.1));
    index.map_or((0, None), |index| {
        let m = max[index.0];
        let l = index.1 + 1;
        (l, Some(m))
    })
}
fn parse(input: &str) -> Vec<(u32, u32)> {
    input
        .lines()
        .map(|line| line.split_once('/').unwrap())
        .map(|(n1, n2)| (n1.parse().unwrap(), n2.parse().unwrap()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(31));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_695));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(19));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_673));
    }
}
