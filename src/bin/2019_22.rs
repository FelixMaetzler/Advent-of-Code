use core::str::FromStr;

all_aoc::solution!(22, 2019);
enum Instruction {
    DealIncrement(i128),
    Cut(i128),
    NewStack,
}
impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("deal with increment") {
            Ok(Self::DealIncrement(
                s.split_ascii_whitespace().last().unwrap().parse().unwrap(),
            ))
        } else if s.starts_with("cut") {
            Ok(Self::Cut(
                s.split_ascii_whitespace().last().unwrap().parse().unwrap(),
            ))
        } else if s == "deal into new stack" {
            Ok(Self::NewStack)
        } else {
            Err(s.to_owned())
        }
    }
}
pub fn part_one(input: &str) -> Option<i128> {
    const CARD_COUNT: i128 = 10_007;
    const REPETITIONS: i128 = 1;
    const CARD_NUMBER: i128 = 2_019;

    let mut a: i128 = 1;
    let mut b: i128 = 0;
    for i in parse(input) {
        match i {
            Instruction::DealIncrement(x) => {
                a = a * x % CARD_COUNT;
                b = b * x % CARD_COUNT;
            }
            Instruction::Cut(x) => {
                b = (b - x).rem_euclid(CARD_COUNT);
            }
            Instruction::NewStack => {
                a = (-a).rem_euclid(CARD_COUNT);
                b = (CARD_COUNT - 1 - b).rem_euclid(CARD_COUNT);
            }
        }
    }
    let r = b * mod_inv(1 - a, CARD_COUNT) % CARD_COUNT;
    let a_n = mod_pow(a, REPETITIONS * (CARD_COUNT - 2), CARD_COUNT); // a^(n*(m-2)) % m
    Some(
        (0..CARD_COUNT)
            .find(|i| ((i - r) * a_n + r).rem_euclid(CARD_COUNT) == CARD_NUMBER)
            .unwrap(),
    )
}
pub fn part_two(input: &str) -> Option<i128> {
    const CARD_COUNT: i128 = 119_315_717_514_047;
    const REPETITIONS: i128 = 101_741_582_076_661;
    const POS: i128 = 2_020;

    let mut a: i128 = 1;
    let mut b: i128 = 0;
    for i in parse(input) {
        match i {
            Instruction::DealIncrement(x) => {
                a = a * x % CARD_COUNT;
                b = b * x % CARD_COUNT;
            }
            Instruction::Cut(x) => {
                b = (b - x).rem_euclid(CARD_COUNT);
            }
            Instruction::NewStack => {
                a = (-a).rem_euclid(CARD_COUNT);
                b = (CARD_COUNT - 1 - b).rem_euclid(CARD_COUNT);
            }
        }
    }
    let r = b * mod_inv(1 - a, CARD_COUNT) % CARD_COUNT;
    let a_n = mod_pow(a, REPETITIONS * (CARD_COUNT - 2), CARD_COUNT); // a^(n*(m-2)) % m
    let card = ((POS - r) * a_n + r).rem_euclid(CARD_COUNT);
    Some(card)
}
const fn mod_inv(x: i128, m: i128) -> i128 {
    // Fermat-Inverse: x^(m-2) % m, m muss prim sein
    mod_pow(x, m - 2, m)
}
const fn mod_pow(mut base: i128, mut exp: i128, m: i128) -> i128 {
    let mut result = 1;
    base = base % m;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % m;
        }
        base = base * base % m;
        exp /= 2;
    }
    result
}
fn parse(input: &str) -> impl Iterator<Item = Instruction> {
    input.lines().map(|l| Instruction::from_str(l).unwrap())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(3_074));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(104_073_967_000_066));
    }
}
