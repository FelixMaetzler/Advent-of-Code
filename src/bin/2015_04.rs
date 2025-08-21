use all_aoc::helper::md5::md5;

all_aoc::solution!(4, 2015);

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, 5)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, 6)
}
fn solve(input: &str, zero_count: usize) -> Option<u32> {
    (1..=u32::MAX).find(|i| md5(&format!("{input}{i}")).starts_with(&"0".repeat(zero_count)))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(Some(609_043), part_one("abcdef"));
        assert_eq!(Some(1_048_970), part_one("pqrstuv"));
    }
    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(117_946));
    }
    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(3_938_038));
    }
}
