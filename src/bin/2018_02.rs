use all_aoc::helper::{misc::count_occurrences, permutations::IteratorCombinator as _};

all_aoc::solution!(2, 2018);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut x, mut y) = (0, 0);
    for line in input.lines() {
        let map = count_occurrences(line.chars());
        x += u32::from(map.values().any(|v| *v == 2));
        y += u32::from(map.values().any(|v| *v == 3));
    }
    Some(x * y)
}

pub fn part_two(input: &str) -> Option<String> {
    let x = input
        .lines()
        .combinations(2)
        .find(|c| {
            c[0].chars()
                .zip(c[1].chars())
                .filter(|(c1, c2)| c1 != c2)
                .count()
                == 1
        })
        .unwrap();
    Some(
        x[0].chars()
            .zip(x[1].chars())
            .filter_map(|(c1, c2)| (c1 == c2).then_some(c1))
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(9_139));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some("fgij".into()));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some("uqcidadzwtnhsljvxyobmkfyr".into()));
    }
}
