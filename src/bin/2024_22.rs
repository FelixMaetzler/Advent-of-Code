use core::iter::successors;

all_aoc::solution!(22, 2024);
const fn next(x: u64) -> u64 {
    let mut x = x;
    x = ((64 * x) ^ x) % 0x0100_0000;
    x = ((x / 32) ^ x) % 0x0100_0000;
    x = ((2048 * x) ^ x) % 0x0100_0000;
    x
}
pub fn part_one(input: &str) -> Option<u64> {
    let vec = parse(input);
    Some(
        vec.into_iter()
            .map(|i| successors(Some(i), |n| Some(next(*n))).nth(2000).unwrap())
            .sum(),
    )
}
fn index(deltas: (i8, i8, i8, i8)) -> usize {
    usize::try_from(deltas.0 + 9).unwrap() * 19 * 19 * 19
        + usize::try_from(deltas.1 + 9).unwrap() * 19 * 19
        + usize::try_from(deltas.2 + 9).unwrap() * 19
        + usize::try_from(deltas.3 + 9).unwrap()
}
pub fn part_two(input: &str) -> Option<u64> {
    let vec = parse(input);
    let mut map = vec![0; 19_usize.pow(4)];
    for mut m in vec {
        let mut seen = vec![false; 19_usize.pow(4)];
        let mut old_price = (m % 10) as i8;
        let mut deltas = Vec::with_capacity(2_001);
        for _ in 0..2000 {
            m = next(m);
            let price = (m % 10) as i8;
            let delta = price - old_price;
            deltas.push(delta);
            old_price = price;
            let n = deltas.len();
            if n < 4 {
                continue;
            }
            let deltas = (deltas[n - 4], deltas[n - 3], deltas[n - 2], deltas[n - 1]);
            let idx = index(deltas);
            if seen[idx] {
                continue;
            }
            seen[idx] = true;
            map[idx] += i32::from(price);
        }
    }
    Some(map.into_iter().max().unwrap().try_into().unwrap())
}
fn parse(input: &str) -> Vec<u64> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}
#[cfg(test)]
mod tests {
    use super::*;
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
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(24));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_568));
    }
}
