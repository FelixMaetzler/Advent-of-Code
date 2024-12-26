use std::collections::HashMap;

all_aoc::solution!(22, 2024);
struct Monkey {
    seq: Vec<u64>,
    change: Vec<i32>,
}
impl Monkey {
    fn price(&self, substring: &[i32]) -> u64 {
        match self
            .change
            .windows(4)
            .enumerate()
            .find(|(_, x)| *x == substring)
        {
            Some((i, _)) => self.seq[i + 3] % 10,
            None => 0,
        }
    }
}
fn next(sn: u64) -> u64 {
    let mut sn = sn;
    sn = prune(mix(sn * 64, sn));
    sn = prune(mix(sn / 32, sn));
    sn = prune(mix(sn * 2048, sn));
    sn
}
fn mix(x: u64, secret_number: u64) -> u64 {
    secret_number ^ x
}
fn prune(secret_number: u64) -> u64 {
    secret_number % 16_777_216
}

pub fn part_one(input: &str) -> Option<u64> {
    let vec = parse(input);
    Some(vec.into_iter().map(|sn| *sn.seq.last().unwrap()).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let vec = parse(input);
    let w = vec.iter().flat_map(|m| m.change.windows(4));
    let mut cache = HashMap::new();
    let max = w
        .into_iter()
        .map(|substring| {
            if let Some(x) = cache.get(substring) {
                *x
            } else {
                let x = vec.iter().map(|m| m.price(substring)).sum::<u64>();
                cache.insert(substring, x);
                x
            }
        })
        .max()
        .unwrap();

    Some(max)
}
fn parse(input: &str) -> Vec<Monkey> {
    input
        .lines()
        .map(|l| {
            let mut seq = Vec::with_capacity(2000);
            let mut change = Vec::with_capacity(2000);
            let start = l.parse().unwrap();
            let mut sn = start;
            for _ in 0..2000 {
                let new_sn = next(sn);
                let diff = (new_sn % 10) as i32 - (sn % 10) as i32;
                change.push(diff);
                sn = new_sn;
                seq.push(sn);
            }
            debug_assert_eq!(change.len(), 2000);
            debug_assert_eq!(seq.len(), 2000);
            Monkey { seq, change }
        })
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_operations() {
        assert_eq!(next(123), 15_887_950);
        assert_eq!(mix(15, 42), 37);
        assert_eq!(prune(100_000_000), 16_113_920);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(37_327_623));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(14_082_561_342));
    }
    #[test]
    fn test_price() {
        let vec = parse("2024");
        let x = vec.first().unwrap();
        assert_eq!(x.price(&[-2, 1, -1, 3]), 9);
    }
    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(24));
    }
    #[cfg(feature = "expensive")]
    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_568));
    }
}
