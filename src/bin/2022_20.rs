all_aoc::solution!(20, 2022);
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
struct Unique {
    value: i64,
    id: usize,
}
pub fn part_one(input: &str) -> Option<i64> {
    solve(input, true)
}

pub fn part_two(input: &str) -> Option<i64> {
    solve(input, false)
}
fn solve(input: &str, part_one: bool) -> Option<i64> {
    let inital_vec_val = parse(input).collect::<Vec<_>>();
    let length = inital_vec_val.len();
    let inital_vec: Vec<Unique> = if part_one {
        inital_vec_val
            .iter()
            .enumerate()
            .map(|(i, val)| Unique { value: *val, id: i })
            .collect()
    } else {
        inital_vec_val
            .iter()
            .enumerate()
            .map(|(i, val)| Unique {
                value: *val * 811_589_153,
                id: i,
            })
            .collect()
    };
    let mut vec = inital_vec.clone();
    let iterations = if part_one { 1 } else { 10 };
    for _ in 0..iterations {
        for n in &inital_vec {
            let index = vec.iter().enumerate().find(|&(_i, b)| *b == *n).unwrap().0;
            vec.remove(index);
            let added = i64::try_from(index).unwrap() + n.value;
            let new_index = (added.rem_euclid(vec.len().try_into().unwrap()))
                .try_into()
                .unwrap();
            vec.insert(new_index, *n);
        }
    }
    let zero_index = vec
        .iter()
        .enumerate()
        .find(|&(_i, b)| b.value == 0)
        .unwrap()
        .0;
    let x1 = &vec[(zero_index + 1000) % length];
    let x2 = &vec[(zero_index + 2000) % length];
    let x3 = &vec[(zero_index + 3000) % length];
    Some(x1.value + x2.value + x3.value)
}
fn parse(input: &str) -> impl Iterator<Item = i64> {
    input.lines().map(|l| l.parse().unwrap())
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
        assert_eq!(result, Some(19_559));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(1_623_178_306));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(912_226_207_972));
    }
}
