use std::str::FromStr;

all_aoc::solution!(7, 2024);
struct Equation {
    result: u64,
    numbers: Vec<u64>,
}
impl FromStr for Equation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (result, rest) = s.split_once(": ").unwrap();
        let result = result.parse().unwrap();
        let numbers = rest.split(" ").map(|n| n.parse().unwrap()).collect();
        Ok(Equation { result, numbers })
    }
}
impl Equation {
    fn test_one_comb_part_1(&self, comb: u64) -> bool {
        let mut comb = comb;
        let mut sum = *self.numbers.first().expect("cant be empty");
        for n in &self.numbers[1..] {
            if comb % 2 == 0 {
                sum += n;
            } else {
                sum *= n;
            }
            comb >>= 1;
        }
        debug_assert_eq!(comb, 0);
        sum == self.result
    }
    fn test_all_combs_part_1(&self) -> bool {
        let comb = 1 << (self.numbers.len() - 1);
        (0..comb).any(|comb| self.test_one_comb_part_1(comb))
    }
    fn test_all_combs_part_2(&self) -> bool {
        let comb = 3_u64.pow((self.numbers.len() - 1) as u32);
        (0..comb).any(|comb| self.test_one_comb_part_2(comb))
    }
    fn test_one_comb_part_2(&self, comb: u64) -> bool {
        let mut comb = comb;
        let mut sum = *self.numbers.first().expect("cant be empty");
        for n in &self.numbers[1..] {
            match comb % 3 {
                0 => sum += n,
                1 => sum *= n,
                2 => sum = format!("{}{}", sum, n).parse().unwrap(),
                _ => unreachable!("Mod 3 has only 0-2"),
            }
            comb /= 3;
        }
        debug_assert_eq!(comb, 0);
        sum == self.result
    }
}
pub fn part_one(input: &str) -> Option<u64> {
    let eqs = parse(input);
    Some(
        eqs.into_iter()
            .filter(|eq| eq.test_all_combs_part_1())
            .map(|eq| eq.result)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let eqs = parse(input);
    Some(
        eqs.into_iter()
            .filter(|eq| eq.test_all_combs_part_2())
            .map(|eq| eq.result)
            .sum(),
    )
}
fn parse(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|l| Equation::from_str(l).unwrap())
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(3_749));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(2_501_605_301_465));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(11_387));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(44_841_372_855_953));
    }
}
