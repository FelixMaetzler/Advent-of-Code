all_aoc::solution!(25, 2022);
fn snafu_to_decimal(input: &str) -> i64 {
    let mut sum = 0;
    for c in input.chars() {
        sum *= 5;
        let x = match c {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => unreachable!(),
        };
        sum += x;
    }
    sum
}
fn decimal_to_snafu(mut n: i64) -> String {
    if n == 0 {
        return "0".to_owned();
    }

    let mut result = Vec::new();

    while n != 0 {
        let mut rem = n % 5;
        n /= 5;

        if rem > 2 {
            rem -= 5;
            n += 1;
        }

        let c = match rem {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => unreachable!(),
        };
        result.push(c);
    }

    result.iter().rev().collect()
}

pub fn part_one(input: &str) -> Option<String> {
    Some(decimal_to_snafu(input.lines().map(snafu_to_decimal).sum()))
}

pub const fn part_two(_: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn snafu_to_dec() {
        assert_eq!(snafu_to_decimal("1=-0-2"), 1_747);
        assert_eq!(snafu_to_decimal("12111"), 906);
    }
    #[test]
    fn dec_to_snafu() {
        assert_eq!(decimal_to_snafu(1_747), "1=-0-2");
        assert_eq!(decimal_to_snafu(906), "12111");
    }
    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some("2=-1=0".to_owned()));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some("2=112--220-=-00=-=20".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, None);
    }
}
