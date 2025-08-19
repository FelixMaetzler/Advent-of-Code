all_aoc::solution!(25, 2024);
fn check_key_lock_pair(key: &[u32], lock: &[u32]) -> bool {
    key.iter().zip(lock).all(|(x, y)| x + y <= 5)
}
pub fn part_one(input: &str) -> Option<usize> {
    let (keys, locks) = parse(input);
    Some(
        keys.into_iter()
            .map(|k| locks.iter().filter(|l| check_key_lock_pair(&k, l)).count())
            .sum(),
    )
}

pub const fn part_two(_: &str) -> Option<u32> {
    None
}

fn parse(input: &str) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
    let mut keys = vec![];
    let mut locks = vec![];
    for block in input.split("\n\n") {
        if block.lines().next().unwrap().chars().all(|c| c == '#') {
            // Lock
            let mut lock = vec![0; 5];
            for (i, line) in block.lines().skip(1).enumerate() {
                for (j, c) in line.chars().enumerate() {
                    if c == '#' {
                        lock[j] = 1 + u32::try_from(i).unwrap();
                    }
                }
            }
            locks.push(lock);
        } else if block.lines().next().unwrap().chars().all(|c| c == '.') {
            // Key
            let mut key = vec![5; 5];
            for (i, line) in block.lines().skip(1).enumerate() {
                for (j, c) in line.chars().enumerate() {
                    if c == '.' {
                        key[j] = 4 - u32::try_from(i).unwrap();
                    }
                }
            }
            keys.push(key);
        } else {
            unreachable!()
        }
    }
    (keys, locks)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(3_439));
    }
}
