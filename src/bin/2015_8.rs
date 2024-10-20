all_aoc::solution!(8, 2015);

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);
    let add: usize = input.iter().map(|l| l.chars().count()).sum();
    let sub: usize = input.iter().map(|l| count_chars(l)).sum();
    Some(add - sub)
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = parse(input);
    let add: usize = input.iter().map(|l| encode(l)).sum();
    let sub: usize = input.iter().map(|l| l.chars().count()).sum();
    Some(add - sub)
}
fn parse(input: &str) -> Vec<String> {
    input.lines().map(|l| l.into()).collect()
}
fn count_chars(input: &str) -> usize {
    let vec: Vec<_> = input.chars().collect();
    let mut sum = 0;
    let mut i = 0;
    while i < vec.len() {
        match vec[i] {
            '\\' => {
                i += 1;
                match vec[i] {
                    '\\' => sum += 1,
                    '"' => sum += 1,
                    'x' => {
                        i += 2;
                        sum += 1
                    }
                    _ => unreachable!(),
                }
            }
            _ => sum += 1,
        }
        i += 1;
    }
    sum - 2
}
fn encode(input: &str) -> usize {
    let vec: Vec<_> = input.chars().collect();
    let mut sum = 0;
    let mut i = 0;
    while i < vec.len() {
        match vec[i] {
            '\\' => sum += 2,
            '"' => sum += 2,
            _ => sum += 1,
        }
        i += 1
    }
    sum + 2
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_350));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(19));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(2_085));
    }
}
