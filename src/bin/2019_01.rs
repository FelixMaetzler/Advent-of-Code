all_aoc::solution!(1, 2019);

pub fn part_one(input: &str) -> Option<u32> {
    Some(parse(input).map(sum_part_one).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(parse(input).map(sum_part_two).sum())
}
fn parse(input: &str) -> impl Iterator<Item = u32> + use<'_> {
    input.lines().map(|s| s.parse::<u32>().unwrap())
}
#[inline(always)]
fn sum_part_one(i: u32) -> u32 {
    (i / 3) - 2
}
#[inline(always)]
fn sum_part_two(i: u32) -> u32 {
    let mut remainder = i;
    let mut sum = 0;
    while remainder != 0 {
        remainder = (remainder / 3).saturating_sub(2);
        sum += remainder;
    }
    sum
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(3_235_550));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(4_850_462));
    }
}
