all_aoc::solution!(13, 2017);

pub fn part_one(input: &str) -> Option<usize> {
    let mask = parse(input);
    let mut curr = mask.iter().map(|(e, _)| (*e, 0)).collect::<Vec<_>>();
    let mut dir = vec![1; curr.len()];
    let end = mask.last().unwrap().0;
    let mut curr_pos = 0;
    let mut sum = 0;
    while curr_pos <= end {
        if caught(curr_pos, &curr) {
            let elem = mask.iter().find(|(i, _)| i == &curr_pos).unwrap();
            sum += elem.0 * elem.1;
        }
        next_step(&mask, &mut curr, &mut dir);
        curr_pos += 1;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mask = parse(input);

    let mut curr_save = mask.iter().map(|(e, _)| (*e, 0)).collect::<Vec<_>>();
    let mut dir_save = vec![1; curr_save.len()];
    let end = mask.last().unwrap().0;
    for delay in 0..usize::MAX {
        let mut curr_pos = 0;
        let mut curr = curr_save.clone();
        let mut dir = dir_save.clone();
        while curr_pos <= end {
            if caught(curr_pos, &curr) {
                break;
            }
            next_step(&mask, &mut curr, &mut dir);
            curr_pos += 1;
        }
        if curr_pos > end {
            return Some(delay);
        }
        next_step(&mask, &mut curr_save, &mut dir_save);
    }
    unreachable!()
}
fn next_step(mask: &[(usize, usize)], curr: &mut [(usize, usize)], dir: &mut [isize]) {
    for i in 0..mask.len() {
        if curr[i].1 == 0 {
            dir[i] = 1;
        } else if curr[i].1 == mask[i].1 - 1 {
            dir[i] = -1;
        }
        curr[i].1 = (isize::try_from(curr[i].1).unwrap() + dir[i])
            .try_into()
            .unwrap();
    }
}
fn caught(curr_pos: usize, curr: &[(usize, usize)]) -> bool {
    curr.iter().any(|elem| elem == &(curr_pos, 0))
}
fn parse(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(left, right)| (left.parse().unwrap(), right.parse().unwrap()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(24));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(3_184));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(3_878_062));
    }
}
