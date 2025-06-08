all_aoc::solution!(16, 2019);
pub fn part_one(input: &str) -> Option<i32> {
    solve_part_one(input, 100)
}

pub fn part_two(input: &str) -> Option<i32> {
    let vec = parse(input);
    let start = folding_to_number(&vec[0..7]) as usize;
    let end = vec.len() * 10_000;

    let mut current = Vec::new();
    for i in start..end {
        current.push(vec[i % vec.len()]);
    }

    for _ in 0..100 {
        let mut sums = vec![0];
        let mut total = 0;

        (0..current.len()).for_each(|i| {
            total += current[i];
            sums.push(total);
        });

        for i in 0..current.len() {
            let value = sums.last().unwrap() - sums[i];
            current[i] = value % 10;
        }
    }
    Some(folding_to_number(&current[0..8]))
}
fn solve_part_one(input: &str, phase_count: usize) -> Option<i32> {
    let mut vec = parse(input);
    (0..phase_count).for_each(|_| vec = phase(&vec));
    Some(folding_to_number(&vec[..8]))
}
fn phase(vec: &[i32]) -> Vec<i32> {
    (0..vec.len())
        .map(|i| {
            let iter = vec.iter().zip(genereate_iter(i));
            let n = iter.fold(0, |acc, x| acc + x.0 * x.1);
            n.abs() % 10
        })
        .collect()
}

fn genereate_iter(i: usize) -> impl Iterator<Item = i32> {
    let it = [0, 1, 0, -1].into_iter();
    let repeated = it.flat_map(move |n| std::iter::repeat_n(n, i + 1));
    repeated.cycle().skip(1)
}
fn folding_to_number(input: &[i32]) -> i32 {
    input.iter().fold(0, |tot, cur| tot * 10 + cur)
}
fn parse(input: &str) -> Vec<i32> {
    input
        .chars()
        .map(|n| n.to_digit(10).unwrap() as i32)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = &all_aoc::cli::read_examples_file(DAY);
        let mut iter = input.lines();
        assert_eq!(solve_part_one(iter.next().unwrap(), 4), Some(1_029_498));
        assert_eq!(solve_part_one(iter.next().unwrap(), 100), Some(24_176_176));
        assert_eq!(solve_part_one(iter.next().unwrap(), 100), Some(73_745_418));
        assert_eq!(solve_part_one(iter.next().unwrap(), 100), Some(52_432_133));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(34_694_616));
    }

    #[test]
    fn test_part_two() {
        let input = &all_aoc::cli::read_examples_file(DAY);
        let mut iter = input.lines().skip(4);

        assert_eq!(part_two(iter.next().unwrap()), Some(84_462_026));
        assert_eq!(part_two(iter.next().unwrap()), Some(78_725_270));
        assert_eq!(part_two(iter.next().unwrap()), Some(53_553_731));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(17_069_048));
    }
}
