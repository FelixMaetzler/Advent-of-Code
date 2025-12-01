use core::iter;

all_aoc::solution!(1, 2025);

fn expand(dir: i32) -> impl Iterator<Item = i32> {
    iter::repeat_n(dir.signum(), dir.unsigned_abs() as usize)
}
pub fn part_one(input: &str) -> Option<u32> {
    solve(parse(input))
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(parse(input).flat_map(expand))
}
fn solve(it: impl Iterator<Item = i32>) -> Option<u32> {
    let mut x = 50;
    let mut cnt = 0;
    for i in it {
        x += i;
        x %= 100;
        if x == 0 {
            cnt += 1;
        }
    }
    Some(cnt)
}
fn parse(input: &str) -> impl Iterator<Item = i32> {
    input.lines().map(|line| {
        let i: i32 = line[1..].parse().unwrap();
        match &line[..1] {
            "R" => i,
            "L" => -i,
            _ => unreachable!(),
        }
    })
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_078));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(6_412));
    }
}
