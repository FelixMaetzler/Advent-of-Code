all_aoc::solution!(1, 2017);

pub fn part_one(input: &str) -> Option<u32> {
    let vec = parse(input);
    Some(solve(&vec, 1))
}

pub fn part_two(input: &str) -> Option<u32> {
    let vec = parse(input);
    assert_eq!(vec.len() % 2, 0);
    Some(solve(&vec, vec.len() / 2))
}
fn solve(vec: &[u32], look_ahead: usize) -> u32 {
    let len = vec.len();
    vec.iter()
        .enumerate()
        .map(|(i, v)| {
            if *v == vec[(i + look_ahead) % len] {
                *v
            } else {
                0
            }
        })
        .sum()
}
fn parse(input: &str) -> Vec<u32> {
    input.chars().map(|c| c.to_digit(10).unwrap()).collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("1122"), Some(3));
        assert_eq!(part_one("1111"), Some(4));
        assert_eq!(part_one("1234"), Some(0));
        assert_eq!(part_one("91212129"), Some(9));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_216));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("1212"), Some(6));
        assert_eq!(part_two("1221"), Some(0));
        assert_eq!(part_two("123425"), Some(4));
        assert_eq!(part_two("123123"), Some(12));
        assert_eq!(part_two("12131415"), Some(4));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_072));
    }
}
