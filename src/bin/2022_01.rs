all_aoc::solution!(1, 2022);

pub fn part_one(input: &str) -> Option<u32> {
    Some(parse(input).map(core::iter::Iterator::sum).max().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut v = parse(input)
        .map(core::iter::Iterator::sum)
        .collect::<Vec<u32>>();
    v.sort_unstable_by(|a, b| b.cmp(a));
    Some(v.into_iter().take(3).sum())
}
fn parse(input: &str) -> impl Iterator<Item = impl Iterator<Item = u32>> {
    input
        .split("\n\n")
        .map(|b| b.lines().map(|l| l.parse().unwrap()))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(24_000));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(72_017));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(45_000));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(212_520));
    }
}
