use core::iter;

all_aoc::solution!(1, 2023);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .trim()
            .lines()
            .map(|l| {
                let n1 = l.find(|c: char| c.is_ascii_digit()).unwrap();
                let n2 = l.rfind(|c: char| c.is_ascii_digit()).unwrap();
                let n1 = l.chars().nth(n1).unwrap().to_digit(10).unwrap();
                let n2 = l.chars().nth(n2).unwrap().to_digit(10).unwrap();
                n1 * 10 + n2
            })
            .sum(),
    )
}
const MAX_CHAR_MATCH: usize = 5;
pub fn part_two(input: &str) -> Option<u32> {
    Some(input.trim().lines().map(solve_part_2).sum())
}
fn solve_part_2(line: &str) -> u32 {
    let n1 = (0..line.len())
        .flat_map(|i| iter::repeat(i).zip(i..line.len().min(i + MAX_CHAR_MATCH)))
        .find_map(|(i, j)| parse_number(&line[i..=j]))
        .unwrap();
    let n2 = (0..line.len())
        .rev()
        .flat_map(|i| iter::repeat(i).zip((i.saturating_sub(MAX_CHAR_MATCH)..=i).rev()))
        .find_map(|(i, j)| parse_number(&line[j..=i]))
        .unwrap();

    n1 * 10 + n2
}
fn parse_number(input: &str) -> Option<u32> {
    match input {
        "1" | "one" => Some(1),
        "2" | "two" => Some(2),
        "3" | "three" => Some(3),
        "4" | "four" => Some(4),
        "5" | "five" => Some(5),
        "6" | "six" => Some(6),
        "7" | "seven" => Some(7),
        "8" | "eight" => Some(8),
        "9" | "nine" => Some(9),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(54_634));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(53_855));
    }
}
