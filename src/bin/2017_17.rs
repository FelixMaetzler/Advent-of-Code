all_aoc::solution!(17, 2017);

pub fn part_one(input: &str) -> Option<usize> {
    let input: usize = input.parse().unwrap();
    let constant = 2017;
    let mut vec = Vec::with_capacity(constant);
    vec.push(0);
    let mut curr_pos = 0;
    for i in 1..=constant {
        let new = ((curr_pos + input) % vec.len()) + 1;
        if new == vec.len() {
            vec.push(i);
            curr_pos = new;
        } else {
            vec.insert(new % vec.len(), i);
            curr_pos = new % vec.len();
        }
    }
    let (index, _) = vec
        .iter()
        .enumerate()
        .find(|&(_, &val)| val == constant)
        .unwrap();
    Some(vec[(index + 1) % vec.len()])
}

pub fn part_two(input: &str) -> Option<usize> {
    let input: usize = input.parse().unwrap();
    let constant = 50_000_000;
    let mut curr_pos = 0;
    let mut curr_zero = 0;
    let mut next_after_zero = 0;
    for i in 1..=constant {
        let next = (curr_pos + input) % i;
        match next.cmp(&curr_zero) {
            core::cmp::Ordering::Less => curr_zero += 1,
            core::cmp::Ordering::Equal => next_after_zero = i,
            core::cmp::Ordering::Greater => {}
        }

        curr_pos = next + 1;
    }
    Some(next_after_zero)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(808));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(47_465_686));
    }
}
