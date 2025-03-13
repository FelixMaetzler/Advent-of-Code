use all_aoc::helper::md5::md5;

all_aoc::solution!(5, 2016);
struct Md5iterator {
    door_id: String,
    index: u32,
}
impl Md5iterator {
    fn new(door_id: String) -> Self {
        Self { door_id, index: 0 }
    }
}
impl Iterator for Md5iterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        for i in self.index.. {
            let s = i.to_string();
            let s = self.door_id.clone() + &s;
            let hash = md5(&s);
            if hash.chars().take(5).all(|c| c == '0') {
                self.index = i + 1;
                return Some(hash[5..].to_string());
            }
        }
        None
    }
}
pub fn part_one(input: &str) -> Option<String> {
    let md5 = Md5iterator::new(input.to_string());
    Some(
        md5.into_iter()
            .take(8)
            .map(|s| s.chars().next().unwrap())
            .collect(),
    )
}
pub fn part_two(input: &str) -> Option<String> {
    let md5 = Md5iterator::new(input.to_string());
    let mut array = [None; 8];
    for hash in md5 {
        let index = hash
            .chars()
            .next()
            .unwrap()
            .to_digit(10)
            .unwrap_or(u32::MAX) as usize;
        let val = hash.chars().nth(1).unwrap();
        if index >= 8 {
            continue;
        }
        if array[index].is_none() {
            array[index] = Some(val);
        }
        if array.iter().all(|t| t.is_some()) {
            return Some(array.into_iter().map(|v| v.unwrap()).collect());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some("18f47a30".to_string()));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some("f77a0e6e".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some("05ace8e3".to_string()));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some("999828ec".to_string()));
    }
}
