use all_aoc::helper::graph::{Graph as _, Special, WithWeights as _};

all_aoc::solution!(12, 2017);

pub fn part_one(input: &str) -> Option<usize> {
    let graph = parse(input);
    let vec = graph.connected_components();
    Some(vec.into_iter().find(|s| s.contains(&0)).unwrap().len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let graph = parse(input);
    let vec = graph.connected_components();
    Some(vec.len())
}
fn parse(input: &str) -> Special<u8> {
    Special::from_edges(
        input
            .lines()
            .map(|l| l.split_once(" <-> ").unwrap())
            .flat_map(|(from, tos)| {
                tos.split(", ")
                    .map(|to| (from.parse().unwrap(), to.parse().unwrap(), 1))
            }),
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(141));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(171));
    }
}
