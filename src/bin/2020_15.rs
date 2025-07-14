use std::collections::HashMap;

all_aoc::solution!(15, 2020);

pub fn part_one(input: &str) -> Option<u32> {
    Some(number_spoken(parse(input), 2_020))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(number_spoken(parse(input), 30_000_000))
}
fn number_spoken(vec: Vec<u32>, x: u32) -> u32 {
    let mut vec = vec;
    let len = vec.len();
    vec.reserve(x as usize); // TODO: Vec is technically not needed
    if x as usize <= vec.len() {
        return vec[(x - 1) as usize];
    }
    let mut map = HashMap::new();
    for i in 1..=x {
        let curr = vec[i as usize - 1];
        let next = if let Some(e) = map.insert(curr, i) {
            i - e
        } else {
            0
        };

        if i as usize >= len {
            vec.push(next);
        }
    }
    vec[x as usize - 1]
}
fn parse(input: &str) -> Vec<u32> {
    input.split(',').map(|s| s.parse().unwrap()).collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("0,3,6"), Some(436));
        assert_eq!(part_one("1,3,2"), Some(1));
        assert_eq!(part_one("2,1,3"), Some(10));
        assert_eq!(part_one("1,2,3"), Some(27));
        assert_eq!(part_one("2,3,1"), Some(78));
        assert_eq!(part_one("3,2,1"), Some(438));
        assert_eq!(part_one("3,1,2"), Some(1_836));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_665));
    }

    #[test]
    fn test_part_two_1() {
        assert_eq!(part_two("0,3,6"), Some(175_594));
    }
    #[test]
    fn test_part_two_2() {
        assert_eq!(part_two("1,3,2"), Some(2_578));
    }
    #[test]
    fn test_part_two_3() {
        assert_eq!(part_two("2,1,3"), Some(3_544_142));
    }
    #[test]
    fn test_part_two_4() {
        assert_eq!(part_two("1,2,3"), Some(261_214));
    }
    #[test]
    fn test_part_two_5() {
        assert_eq!(part_two("2,3,1"), Some(6_895_259));
    }
    #[test]
    fn test_part_two_6() {
        assert_eq!(part_two("3,2,1"), Some(18));
    }
    #[test]
    fn test_part_two_7() {
        assert_eq!(part_two("3,1,2"), Some(362));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(16_439));
    }
}
