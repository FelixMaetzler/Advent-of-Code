use std::{collections::HashMap, str::FromStr};

use all_aoc::helper::misc::Joinable as _;

all_aoc::solution!(4, 2016);
struct Room {
    encrypted_name: Vec<String>,
    id: u32,
    checksum: String,
}
impl Room {
    fn is_real(&self) -> bool {
        let mut map = HashMap::new();
        self.encrypted_name
            .iter()
            .flat_map(|n| n.chars())
            .for_each(|c| {
                map.entry(c).and_modify(|v| *v += 1).or_insert(1);
            });
        let mut vec = map.into_iter().collect::<Vec<_>>();
        vec.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        let s: String = vec.into_iter().take(5).map(|n| n.0).collect();
        s == self.checksum
    }
    fn decrypt(&self) -> String {
        self.encrypted_name
            .iter()
            .map(|n| {
                n.chars()
                    .map(|c| (((u32::from(c as u8 - b'a') + self.id) % 26) as u8 + b'a') as char)
                    .collect::<String>()
            })
            .join(" ")
    }
}
impl FromStr for Room {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once('[').unwrap();
        let checksum = b[0..b.len() - 1].to_string();
        let mut encrypted_name = a
            .split('-')
            .map(std::string::ToString::to_string)
            .collect::<Vec<_>>();
        let id = encrypted_name.pop().unwrap().parse().unwrap();
        Ok(Self {
            encrypted_name,
            id,
            checksum,
        })
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse(input)
            .into_iter()
            .filter(Room::is_real)
            .map(|r| r.id)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        parse(input)
            .iter()
            .find(|r| r.decrypt() == "northpole object storage")
            .unwrap()
            .id,
    )
}
fn parse(input: &str) -> Vec<Room> {
    input.lines().map(|l| Room::from_str(l).unwrap()).collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(1_514));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(245_102));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(324));
    }
}
