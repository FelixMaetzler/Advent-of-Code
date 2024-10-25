use all_aoc::helper::permutations::generate_combinations;

all_aoc::solution!(17, 2015);

pub fn part_one(input: &str) -> Option<usize> {
    solve_part_1(input, 150)
}
fn solve_part_1(input: &str, sum: u32) -> Option<usize> {
    let vec = parse(input);
    let comb = generate_combinations(&vec);
    Some(
        comb.into_iter()
            .filter(|c| c.iter().sum::<u32>() == sum)
            .count(),
    )
}
pub fn part_two(input: &str) -> Option<usize> {
    solve_part_2(input, 150)
}
fn solve_part_2(input: &str, sum: u32) -> Option<usize> {
    let vec = parse(input);
    let comb = generate_combinations(&vec);
    let comb = comb
        .into_iter()
        .filter(|c| c.iter().sum::<u32>() == sum)
        .collect::<Vec<_>>();
    let min = comb.iter().map(|v| v.len()).min().unwrap_or(0);
    Some(comb.into_iter().filter(|v| v.len() == min).count())
}
fn parse(input: &str) -> Vec<u32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_1(&all_aoc::cli::read_examples_file(DAY), 25);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(654));
    }

    #[test]
    fn test_part_two() {
        let result = solve_part_2(&all_aoc::cli::read_examples_file(DAY), 25);
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(57));
    }
}
