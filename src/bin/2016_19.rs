all_aoc::solution!(19, 2016);

pub fn part_one(input: &str) -> Option<usize> {
    Some(oesis_part1(input.parse().unwrap()))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(oesis_part2(input.parse().unwrap()))
}
const fn oesis_part1(n: usize) -> usize {
    //a(n) = 0 if n = 0 and a(n) = 2*a(floor(n/2)) - (-1)^(n mod 2) if n > 0
    if n == 0 {
        return 0;
    }
    if n.is_multiple_of(2) {
        2 * oesis_part1(n / 2) - 1
    } else {
        2 * oesis_part1(n / 2) + 1
    }
}
const fn oesis_part2(n: usize) -> usize {
    const fn highest_power_of_3(n: usize) -> usize {
        let mut option: u32 = 0;
        while 3_usize.pow(option) <= n {
            option += 1;
        }
        3_u32.pow(option - 1) as usize
    }

    let x = highest_power_of_3(n);
    if x == n {
        x
    } else if n < 2 * x {
        n % x
    } else {
        x + 2 * (n % x)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_808_357));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_407_007));
    }
}
