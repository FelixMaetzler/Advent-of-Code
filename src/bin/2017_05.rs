all_aoc::solution!(5, 2017);

pub fn part_one(input: &str) -> Option<u32> {
    let mut vec = input
        .lines()
        .map(|s| s.parse::<isize>().unwrap())
        .collect::<Vec<_>>();
    let mut ptr = 0;
    let mut ctr = 0;
    while ptr >= 0 && ptr < vec.len().try_into().unwrap() {
        let val = vec[usize::try_from(ptr).unwrap()];
        vec[usize::try_from(ptr).unwrap()] += 1;
        ptr += val;

        ctr += 1;
    }
    Some(ctr)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut vec = input
        .lines()
        .map(|s| s.parse::<isize>().unwrap())
        .collect::<Vec<_>>();
    let mut ptr = 0;
    let mut ctr = 0;
    while ptr >= 0 && ptr < vec.len().try_into().unwrap() {
        let val = vec[usize::try_from(ptr).unwrap()];
        vec[usize::try_from(ptr).unwrap()] += if val >= 3 { -1 } else { 1 };
        ptr += val;

        ctr += 1;
    }
    Some(ctr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(315_613));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(22_570_529));
    }
}
