all_aoc::solution!(11, 2015);

pub fn part_one(input: &str) -> Option<String> {
    Some(next_password(input))
}

pub fn part_two(input: &str) -> Option<String> {
    Some(next_password(&next_password(input)))
}
fn next_password(input: &str) -> String {
    let mut input = input.to_owned();
    loop {
        input = next_possible_password(&input);
        if password(&input) {
            return input;
        }
    }
}
fn next_possible_password(input: &str) -> String {
    let mut i = input.len() - 1;
    let mut str = input.chars().collect::<Vec<_>>();
    loop {
        let c = str[i];
        match c {
            'z' => {
                str[i] = 'a';
                i -= 1;
            }
            'a'..='y' => {
                str[i] = (c as u8 + 1) as char;
                break;
            }
            _ => unreachable!(),
        }
    }
    str.iter().collect()
}
fn password(input: &str) -> bool {
    first_requirement(input) && second_requirement(input) && third_requirement(input)
}
fn first_requirement(input: &str) -> bool {
    for s in input.chars().collect::<Vec<_>>().windows(3) {
        if (s[0] as u8 == s[1] as u8 - 1) && (s[1] as u8 == s[2] as u8 - 1) {
            return true;
        }
    }
    false
}
fn second_requirement(input: &str) -> bool {
    !input.chars().any(|c| c == 'i' || c == 'o' || c == 'l')
}
fn third_requirement(input: &str) -> bool {
    let vec = input.chars().collect::<Vec<_>>();
    let mut i = 0;
    let mut counter = 0;
    while i < vec.len() - 1 {
        if vec[i] == vec[i + 1] {
            counter += 1;
            i += 1;
        }
        i += 1;
    }
    counter >= 2
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(Some("abcdffaa".to_owned()), part_one("abcdefgh"));
        assert_eq!(Some("ghjaabcc".to_owned()), part_one("ghijklmn"));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some("vzbxxyzz".to_owned()));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some("vzcaabcc".to_owned()));
    }
}
