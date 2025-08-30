all_aoc::solution!(2, 2017);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse(input)
            .map(|row| row.iter().max().unwrap() - row.iter().min().unwrap())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(parse(input).map(|v| number(&v)).sum())
}
fn number(slice: &[u32]) -> u32 {
    for i in 0..slice.len() {
        for j in 0..slice.len() {
            if j == i {
                continue;
            }
            if slice[i].is_multiple_of(slice[j]) {
                return slice[i] / slice[j];
            }
        }
    }
    unreachable!()
}
fn parse(input: &str) -> impl Iterator<Item = Vec<u32>> {
    input.lines().map(|line| {
        line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(42_378));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(246));
    }
}
