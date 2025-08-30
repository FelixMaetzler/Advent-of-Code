all_aoc::solution!(9, 2017);

pub fn part_one(input: &str) -> Option<u32> {
    let mut vec: Vec<_> = input.chars().collect();
    filter_garabge(&mut vec);
    let mut counter = 0;
    let mut stack = 0;
    for c in vec {
        if c == '{' {
            stack += 1;
            counter += stack;
        } else if c == '}' {
            stack -= 1;
        }
    }
    debug_assert_eq!(stack, 0);
    Some(counter)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut vec: Vec<_> = input.chars().collect();
    let mut curr = 0;
    let mut garbage = false;
    let mut ctr = 0;
    while curr < vec.len() {
        if !garbage && vec[curr] == '<' {
            garbage = true;
            vec.remove(curr);
            continue;
        }
        if garbage {
            if vec[curr] == '>' {
                vec.remove(curr);
                garbage = false;
                continue;
            } else if vec[curr] == '!' {
                vec.remove(curr);
                vec.remove(curr);
                continue;
            }
            vec.remove(curr);
            ctr += 1;
            continue;
        }
        curr += 1;
    }
    Some(ctr)
}

fn filter_garabge(vec: &mut Vec<char>) {
    let mut curr = 0;
    let mut garbage = false;

    while curr < vec.len() {
        if !garbage && vec[curr] == '<' {
            garbage = true;
            vec.remove(curr);
            continue;
        }
        if garbage {
            if vec[curr] == '>' {
                vec.remove(curr);
                garbage = false;
                continue;
            } else if vec[curr] == '!' {
                vec.remove(curr);
                vec.remove(curr);
                continue;
            }
            vec.remove(curr);
            continue;
        }
        curr += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(12_803));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(6_425));
    }
}
