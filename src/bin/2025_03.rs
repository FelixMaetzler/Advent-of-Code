all_aoc::solution!(3, 2025);

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, 2)
}
pub fn part_two(input: &str) -> Option<u64> {
    solve(input, 12)
}
fn solve(input: &str, digit_count: u8) -> Option<u64> {
    Some(parse(input).map(|v| solve_recursive(&v, digit_count)).sum())
}
fn solve_recursive(slice: &[u64], digit_count: u8) -> u64 {
    if digit_count == 0 {
        return 0;
    }
    let (index, max) = slice
        .iter()
        .enumerate()
        .rev()
        .skip(digit_count as usize - 1)
        .max_by_key(|k| k.1)
        .unwrap();
    let slice = &slice[index + 1..];
    max * 10_u64.pow(u32::from(digit_count) - 1) + solve_recursive(slice, digit_count - 1)
}
fn parse(input: &str) -> impl Iterator<Item = Vec<u64>> {
    input.lines().map(|l| {
        l.chars()
            .map(|c| u64::from(c.to_digit(10).unwrap()))
            .collect()
    })
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(17_144));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(3_121_910_778_619));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(170_371_185_255_900));
    }
}
