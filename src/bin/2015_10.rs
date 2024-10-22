all_aoc::solution!(10, 2015);

pub fn part_one(input: &str) -> Option<usize> {
    Some(run(input, 40))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(run(input, 50))
}
fn run(input: &str, amount: u8) -> usize {
    let mut s = String::from(input);
    for _ in 0..amount {
        s = look_and_say(&s);
    }
    s.chars().count()
}
fn look_and_say(s: &str) -> String {
    let mut result = String::with_capacity(2 * s.len());
    let mut current = s.chars().next().unwrap();
    let mut count = 1;
    for c in s.chars().skip(1) {
        if c == current {
            count += 1;
        } else {
            result.push_str(&count.to_string());
            result.push(current);
            current = c;
            count = 1;
        }
    }
    result.push_str(&count.to_string());
    result.push(current);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(252_594));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(3_579_328));
    }
}
