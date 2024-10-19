all_aoc::solution!(1, 2015);

pub fn part_one(input: &str) -> Option<i32> {
    Some(
        input
            .chars()
            .map(|c| match c {
                '(' => 1,
                ')' => -1,
                _ => unreachable!(),
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut sum = 0;
    for (i, c) in input.char_indices() {
        sum += match c {
            '(' => 1,
            ')' => -1,
            _ => unreachable!(),
        };
        if sum < 0 {
            return Some(i + 1);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(Some(0), part_one("(())"));
        assert_eq!(Some(0), part_one("()()"));

        assert_eq!(Some(3), part_one("((("));
        assert_eq!(Some(3), part_one("(()(()("));

        assert_eq!(Some(3), part_one("))((((("));

        assert_eq!(Some(-1), part_one("())"));
        assert_eq!(Some(-1), part_one("))("));

        assert_eq!(Some(-3), part_one(")))"));
        assert_eq!(Some(-3), part_one(")())())"));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(280));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Some(1), part_two(")"));
        assert_eq!(Some(5), part_two("()())"));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_797));
    }
}
