all_aoc::solution!(15, 2017);

pub fn part_one(input: &str) -> Option<usize> {
    let (a, b) = parse(input);
    let mut prev_a = a;
    let mut prev_b = b;
    let mut counter = 0;
    for _ in 0..40_000_000 {
        prev_a = (prev_a * 16_807) % 0x7FFF_FFFF;
        prev_b = (prev_b * 48_271) % 0x7FFF_FFFF;
        if prev_a % 2_usize.pow(16) == prev_b % 2_usize.pow(16) {
            counter += 1;
        }
    }
    Some(counter)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (a, b) = parse(input);
    let mut prev_a = a;
    let mut prev_b = b;
    let mut counter = 0;
    for _ in 0..5_000_000 {
        loop {
            prev_a = (prev_a * 16_807) % 0x7FFF_FFFF;
            if prev_a % 4 == 0 {
                break;
            }
        }
        loop {
            prev_b = (prev_b * 48_271) % 0x7FFF_FFFF;
            if prev_b % 8 == 0 {
                break;
            }
        }
        if prev_a % 2_usize.pow(16) == prev_b % 2_usize.pow(16) {
            counter += 1;
        }
    }
    Some(counter)
}
fn parse(input: &str) -> (usize, usize) {
    let (a, b) = input.split_once('\n').unwrap();
    (
        a.split_ascii_whitespace().last().unwrap().parse().unwrap(),
        b.split_ascii_whitespace().last().unwrap().parse().unwrap(),
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(588));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(612));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(309));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(285));
    }
}
