use std::{collections::HashMap, str::FromStr};

all_aoc::solution!(23, 2015);
enum Operation {
    Half(char),
    Triple(char),
    Inc(char),
    Jump(i32),
    JumpIfEven(char, i32),
    JumpIfOne(char, i32),
}
impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace(',', "");
        let v: Vec<_> = s.split_ascii_whitespace().collect();
        match v[0] {
            "hlf" => Ok(Self::Half(v[1].parse().unwrap())),
            "tpl" => Ok(Self::Triple(v[1].parse().unwrap())),
            "inc" => Ok(Self::Inc(v[1].parse().unwrap())),
            "jmp" => Ok(Self::Jump(v[1].parse().unwrap())),
            "jie" => Ok(Self::JumpIfEven(
                v[1].parse().unwrap(),
                v[2].parse().unwrap(),
            )),
            "jio" => Ok(Self::JumpIfOne(
                v[1].parse().unwrap(),
                v[2].parse().unwrap(),
            )),
            _ => Err(()),
        }
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let vec = parse(input);
    let map = HashMap::from_iter(vec![('a', 0), ('b', 0)]);
    execute(&vec, map)
}
pub fn part_two(input: &str) -> Option<u32> {
    let vec = parse(input);
    let map = HashMap::from_iter(vec![('a', 1), ('b', 0)]);
    execute(&vec, map)
}
fn execute(vec: &[Operation], map: HashMap<char, u32>) -> Option<u32> {
    let mut map = map;
    let mut ptr = 0;
    while ptr < vec.len() {
        match vec[ptr] {
            Operation::Half(reg) => {
                map.entry(reg).and_modify(|e| *e /= 2);
                ptr += 1;
            }
            Operation::Triple(reg) => {
                map.entry(reg).and_modify(|e| *e *= 3);
                ptr += 1;
            }
            Operation::Inc(reg) => {
                map.entry(reg).and_modify(|e| *e += 1);
                ptr += 1;
            }
            Operation::Jump(offset) => {
                ptr = (i32::try_from(ptr).unwrap() + offset).try_into().unwrap();
            }
            Operation::JumpIfEven(reg, offset) => {
                if &map[&reg] % 2 == 0 {
                    ptr = (i32::try_from(ptr).unwrap() + offset).try_into().unwrap();
                } else {
                    ptr += 1;
                }
            }
            Operation::JumpIfOne(reg, offset) => {
                if map[&reg] == 1 {
                    ptr = (i32::try_from(ptr).unwrap() + offset).try_into().unwrap();
                } else {
                    ptr += 1;
                }
            }
        }
    }
    Some(map[&'b'])
}

fn parse(input: &str) -> Vec<Operation> {
    input
        .lines()
        .map(|l| Operation::from_str(l).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(170));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(247));
    }
}
