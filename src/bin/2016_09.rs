all_aoc::solution!(9, 2016);

pub fn part_one(input: &str) -> Option<usize> {
    Some(decompressd_length_1(input))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(decompressd_length_2(input))
}

fn decompressd_length_1(line: &str) -> usize {
    let mut i = 0;
    let mut new_str = "".to_string();
    while i < line.len() {
        let ch = line.chars().nth(i).unwrap();
        if ch == '(' {
            let mut first = "".to_string();
            let mut second = "".to_string();
            i += 1;
            loop {
                let ch = line.chars().nth(i).unwrap();
                if ch.is_numeric() {
                    first.push(ch);
                } else if ch == 'x' {
                    break;
                } else {
                    panic!();
                }
                i += 1;
            }
            i += 1;
            loop {
                let ch = line.chars().nth(i).unwrap();
                if ch.is_numeric() {
                    second.push(ch);
                } else if ch == ')' {
                    break;
                } else {
                    panic!();
                }
                i += 1;
            }
            i += 1;
            let first: usize = first.parse().unwrap();
            let second: usize = second.parse().unwrap();
            let curr_i = i;
            let mut substring = "".to_string();
            while i < curr_i + first {
                let ch = line.chars().nth(i).unwrap();
                substring.push(ch);
                i += 1;
            }
            for _ in 0..second {
                new_str.push_str(&substring);
            }
            i -= 1;
        } else {
            new_str.push(ch);
        }
        i += 1;
    }
    new_str.chars().count()
}
fn decompressd_length_2(line: &str) -> usize {
    let mut counter = 0;
    let mut i = 0;
    while i < line.len() {
        let ch = line.chars().nth(i).unwrap();
        if ch.is_alphabetic() {
            counter += 1;
            i += 1;
        } else if ch == '(' {
            let mut first = "".to_string();
            let mut second = "".to_string();
            i += 1;
            loop {
                let ch = line.chars().nth(i).unwrap();
                if ch.is_numeric() {
                    first.push(ch);
                } else if ch == 'x' {
                    break;
                } else {
                    panic!();
                }
                i += 1;
            }
            i += 1;
            loop {
                let ch = line.chars().nth(i).unwrap();
                if ch.is_numeric() {
                    second.push(ch);
                } else if ch == ')' {
                    break;
                } else {
                    panic!();
                }
                i += 1;
            }
            i += 1;
            let first: usize = first.parse().unwrap();
            let second: usize = second.parse().unwrap();

            let new_slice = &line[i..i + first];
            counter += second * decompressd_length_2(new_slice);
            i += first;
        }
    }
    counter
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("ADVENT"), Some(6));
        assert_eq!(part_one("A(1x5)BC"), Some(7));
        assert_eq!(part_one("(3x3)XYZ"), Some(9));
        assert_eq!(part_one("A(2x2)BCD(2x2)EFG"), Some(11));
        assert_eq!(part_one("(6x1)(1x3)A"), Some(6));
        assert_eq!(part_one("X(8x2)(3x3)ABCY"), Some(18));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(98_135));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("(3x3)XYZ"), Some(9));
        assert_eq!(part_two("X(8x2)(3x3)ABCY"), Some(20));
        assert_eq!(
            part_two("(27x12)(20x12)(13x14)(7x10)(1x12)A"),
            Some(241_920)
        );
        assert_eq!(
            part_two("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"),
            Some(445)
        );
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(10_964_557_606));
    }
}
