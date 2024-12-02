all_aoc::solution!(2, 2024);

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);
    Some(input.into_iter().filter(|vec| is_safe(vec)).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = parse(input);
    Some(
        input
            .into_iter()
            .filter(|vec| is_safe(vec) || is_safe_with_removing_one(vec))
            .count(),
    )
}
fn is_safe_with_removing_one(vec: &[i32]) -> bool {
    vec.iter().enumerate().any(|(i, _)| {
        let filtered = vec
            .iter()
            .enumerate()
            .filter(|&(j, _)| j != i)
            .map(|(_, &v)| v)
            .collect::<Vec<_>>();
        is_safe(&filtered)
    })
}
fn is_safe(vec: &[i32]) -> bool {
    all_differ(vec) && (all_increasing(vec) || all_decreasing(vec))
}
fn all_increasing(vec: &[i32]) -> bool {
    vec.windows(2).all(|x| x[0] < x[1])
}
fn all_decreasing(vec: &[i32]) -> bool {
    vec.windows(2).all(|x| x[0] > x[1])
}
fn all_differ(vec: &[i32]) -> bool {
    vec.windows(2)
        .all(|x| (1..=3).contains(&x[0].abs_diff(x[1])))
}
fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|i| i.parse().unwrap())
                .collect()
        })
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(269));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(337));
    }
}
