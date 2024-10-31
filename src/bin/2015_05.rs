all_aoc::solution!(5, 2015);

pub fn part_one(input: &str) -> Option<usize> {
    Some(input.lines().filter(|l| part_1_nice_string(l)).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(input.lines().filter(|l| part_2_nice_string(l)).count())
}
fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}
fn three_vowels(input: &str) -> bool {
    input.chars().filter(|&c| is_vowel(c)).count() >= 3
}
fn double_letter(input: &str) -> bool {
    let vec: Vec<_> = input.chars().collect();
    vec.windows(2).any(|c| c[0] == c[1])
}
fn disallowed_substrings(input: &str) -> bool {
    !(input.contains("ab") | input.contains("cd") | input.contains("pq") | input.contains("xy"))
}
fn part_1_nice_string(input: &str) -> bool {
    three_vowels(input) && double_letter(input) && disallowed_substrings(input)
}
fn repeats(input: &str) -> bool {
    let vec: Vec<_> = input.chars().collect();
    vec.windows(3).any(|c| c[0] == c[2])
}
fn pair_letters(input: &str) -> bool {
    for x in input.chars().enumerate().collect::<Vec<_>>().windows(2) {
        let (_, substring) = input.split_at(x[1].0 + 1);
        let string = String::from(x[0].1) + &String::from(x[1].1);
        if substring.contains(&string) {
            return true;
        }
    }
    false
}
fn part_2_nice_string(input: &str) -> bool {
    repeats(input) && pair_letters(input)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(Some(1), part_one("ugknbfddgicrmopn"));
        assert_eq!(Some(1), part_one("aaa"));

        assert_eq!(Some(0), part_one("jchzalrnumimnmhp"));
        assert_eq!(Some(0), part_one("haegwjzuvuyypxyu"));
        assert_eq!(Some(0), part_one("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(236));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Some(1), part_two("qjhvhtzxzqqjkmpb"));
        assert_eq!(Some(1), part_two("xxyxx"));
        assert_eq!(Some(0), part_two("uurcxstgmygtbstg"));
        assert_eq!(Some(0), part_two("ieodomkazucvgmuy"));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(51));
    }
}
