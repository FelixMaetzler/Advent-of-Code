all_aoc::solution!(25, 2015);

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse(input);
    Some(index_to_number(index_to_index(input)))
}

pub const fn part_two(_: &str) -> Option<u32> {
    None
}

fn index_to_index(input: (u64, u64)) -> u64 {
    let mut i = 1;
    for add in 0..input.0 {
        i += add;
    }
    for add in 1..input.1 {
        i += add + input.0;
    }
    i
}
fn index_to_number(input: u64) -> u64 {
    let mut curr = 20_151_125;
    for _ in 1..input {
        curr = curr * 252_533 % 33_554_393;
    }
    curr
}
fn parse(input: &str) -> (u64, u64) {
    let vec = input.split_ascii_whitespace().collect::<Vec<_>>();
    let n1 = vec[15].trim_end_matches(',').parse().unwrap();
    let n2 = vec[17].trim_end_matches('.').parse().unwrap();
    (n1, n2)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(9_132_360));
    }
}
