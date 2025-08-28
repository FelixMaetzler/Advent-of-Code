all_aoc::solution!(16, 2016);

pub fn part_one(input: &str) -> Option<String> {
    solve(input, 272)
}

pub fn part_two(input: &str) -> Option<String> {
    solve(input, 35_651_584)
}
fn solve(input: &str, length: usize) -> Option<String> {
    let mut dragon = input.to_owned();
    while dragon.len() <= length {
        dragon = dragon_curve(&dragon);
    }
    let mut check = checksum(&dragon[0..length]);
    while check.len().is_multiple_of(2) {
        check = checksum(&check);
    }
    Some(check)
}
fn checksum(input: &str) -> String {
    input
        .as_bytes()
        .chunks_exact(2)
        .map(|w| if w[0] == w[1] { '1' } else { '0' })
        .collect()
}
fn dragon_curve(a: &str) -> String {
    let change = |c: char| match c {
        '0' => '1',
        '1' => '0',
        _ => unreachable!(),
    };
    let b = a.chars().rev().map(change).collect::<String>();
    a.to_owned() + &'0'.to_string() + &b
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dragon_curve_works() {
        assert_eq!(dragon_curve("1"), "100");
        assert_eq!(dragon_curve("0"), "001");
        assert_eq!(dragon_curve("11111"), "11111000000");
        assert_eq!(dragon_curve("111100001010"), "1111000010100101011110000");
    }
    #[test]
    fn test_part_one() {
        let result = solve("10000", 20);
        assert_eq!(result, Some("01100".to_owned()));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some("10011010010010010".to_owned()));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some("10101011110100011".to_owned()));
    }
}
