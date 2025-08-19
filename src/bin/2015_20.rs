all_aoc::solution!(20, 2015);

pub fn part_one(input: &str) -> Option<u32> {
    let goal = parse(input);
    Some(
        #[expect(
            clippy::maybe_infinite_iter,
            reason = "the problem description says it ends"
        )]
        (1..)
            .map(|i| (i, present_part_1(i)))
            .find(|(_, present)| present >= &goal)
            .unwrap()
            .0,
    )
}
pub fn part_two(input: &str) -> Option<u32> {
    let goal = parse(input);
    Some(
        #[expect(
            clippy::maybe_infinite_iter,
            reason = "the problem description says it ends"
        )]
        (1..)
            .map(|i| (i, present_part_2(i)))
            .find(|(_, present)| present >= &goal)
            .unwrap()
            .0,
    )
}
fn parse(input: &str) -> u32 {
    input.parse().unwrap()
}
fn present_part_1(house: u32) -> u32 {
    // https://oeis.org/A000203
    10 * divisors(house).into_iter().sum::<u32>()
}
fn present_part_2(house: u32) -> u32 {
    11 * divisors(house)
        .into_iter()
        .map(|i| if house > i * 50 { 0 } else { i })
        .sum::<u32>()
}

/// all divisors of n (including 1 and itself)
fn divisors(n: u32) -> Vec<u32> {
    let mut divisors = Vec::new();
    #[expect(clippy::cast_possible_truncation, reason = "f64 to u32")]
    #[expect(clippy::cast_sign_loss, reason = "f64 to u32")]
    for i in 1..=(f64::from(n).sqrt() as u32) {
        if n % i == 0 {
            divisors.push(i);
            if i != n / i {
                divisors.push(n / i);
            }
        }
    }
    divisors
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(786_240));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(831_600));
    }
}
