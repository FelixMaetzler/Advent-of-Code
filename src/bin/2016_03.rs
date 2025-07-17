use all_aoc::helper::permutations::IteratorPermutator as _;

all_aoc::solution!(3, 2016);
fn is_valid(a: u32, b: u32, c: u32) -> bool {
    let tri = [a, b, c];
    tri.iter().permutation().all(|w| w[0] + w[1] > *w[2])
}
pub fn part_one(input: &str) -> Option<usize> {
    let vec = parse(input);
    Some(
        (0..vec.len() / 3)
            .filter(|i| is_valid(vec[3 * i], vec[3 * i + 1], vec[3 * i + 2]))
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let vec = parse(input);
    Some(
        (0..vec.len() / 3)
            .map(|i| (i / 3) * 9 + (i % 3))
            .filter(|i| is_valid(vec[*i], vec[i + 3], vec[i + 6]))
            .count(),
    )
}
fn parse(input: &str) -> Vec<u32> {
    input
        .lines()
        .flat_map(|l| l.split_ascii_whitespace().map(|n| n.parse().unwrap()))
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_032));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_838));
    }
}
