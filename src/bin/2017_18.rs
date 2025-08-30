use core::str::FromStr;
use std::collections::HashMap;

all_aoc::solution!(18, 2017);

enum Value {
    Number(isize),
    Register(char),
}
impl FromStr for Value {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().next().unwrap().is_alphabetic() {
            Ok(Self::Register(s.parse().unwrap()))
        } else {
            Ok(Self::Number(s.parse().unwrap()))
        }
    }
}
enum Instruction {
    Snd(Value),
    Set(char, Value),
    Add(char, Value),
    Mul(char, Value),
    Mod(char, Value),
    Rcv(Value),
    Jgz(Value, Value),
}
impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (vgl, rest) = s.split_at(4);
        match vgl {
            "snd " => Ok(Self::Snd(Value::from_str(rest).unwrap())),
            "set " => {
                let (left, right) = rest.split_once(' ').unwrap();
                Ok(Self::Set(left.parse().unwrap(), right.parse().unwrap()))
            }
            "add " => {
                let (left, right) = rest.split_once(' ').unwrap();
                Ok(Self::Add(left.parse().unwrap(), right.parse().unwrap()))
            }
            "mul " => {
                let (left, right) = rest.split_once(' ').unwrap();
                Ok(Self::Mul(left.parse().unwrap(), right.parse().unwrap()))
            }
            "mod " => {
                let (left, right) = rest.split_once(' ').unwrap();
                Ok(Self::Mod(left.parse().unwrap(), right.parse().unwrap()))
            }
            "rcv " => Ok(Self::Rcv(rest.parse().unwrap())),
            "jgz " => {
                let (left, right) = rest.split_once(' ').unwrap();
                Ok(Self::Jgz(left.parse().unwrap(), right.parse().unwrap()))
            }
            _ => Err(()),
        }
    }
}
impl Instruction {
    fn execute_part1(
        &self,
        hashmap: &mut HashMap<char, isize>,
        sound: &mut Option<isize>,
    ) -> (Option<isize>, Option<isize>) {
        //First value is recovered Sound and second is Jumping
        match self {
            Self::Snd(x) => {
                *sound = match x {
                    Value::Number(x) => Some(*x),
                    Value::Register(c) => Some(*hashmap.get(c).unwrap()),
                };
                (None, None)
            }
            Self::Set(c, v) => {
                let n = match v {
                    Value::Number(x) => *x,
                    Value::Register(c) => *hashmap.get(c).unwrap(),
                };
                hashmap.entry(*c).and_modify(|i| *i = n).or_insert(n);
                (None, None)
            }
            Self::Add(c, v) => {
                let n = match v {
                    Value::Number(x) => *x,
                    Value::Register(c) => *hashmap.get(c).unwrap(),
                };
                hashmap.entry(*c).and_modify(|i| *i += n).or_insert(n);
                (None, None)
            }
            Self::Mul(c, v) => {
                let n = match v {
                    Value::Number(x) => *x,
                    Value::Register(c) => *hashmap.get(c).unwrap(),
                };
                hashmap.entry(*c).and_modify(|i| *i *= n).or_insert(0);
                (None, None)
            }
            Self::Mod(c, v) => {
                let n = match v {
                    Value::Number(x) => *x,
                    Value::Register(c) => *hashmap.get(c).unwrap(),
                };
                hashmap.entry(*c).and_modify(|i| *i %= n).or_insert(0);
                (None, None)
            }
            Self::Rcv(v) => {
                let n = match v {
                    Value::Number(x) => x,
                    Value::Register(c) => hashmap.get(c).unwrap(),
                };
                if *n != 0 {
                    (Some(sound.unwrap()), None)
                } else {
                    (None, None)
                }
            }
            Self::Jgz(x, y) => {
                let x_val = match x {
                    Value::Number(n) => *n,
                    Value::Register(c) => *hashmap.get(c).unwrap(),
                };
                if x_val <= 0 {
                    (None, None)
                } else {
                    let y = match y {
                        Value::Number(n) => *n,
                        Value::Register(c) => *hashmap.get(c).unwrap(),
                    };
                    (None, Some(y))
                }
            }
        }
    }
    fn execute_part2(&self, hashmap: &mut HashMap<char, isize>) -> Option<isize> {
        //First value is recovered Sound and second is Jumping
        match self {
            Self::Set(c, v) => {
                let n = match v {
                    Value::Number(x) => *x,
                    Value::Register(c) => *hashmap.get(c).unwrap(),
                };
                hashmap.entry(*c).and_modify(|i| *i = n).or_insert(n);
                None
            }
            Self::Add(c, v) => {
                let n = match v {
                    Value::Number(x) => *x,
                    Value::Register(c) => *hashmap.get(c).unwrap(),
                };
                hashmap.entry(*c).and_modify(|i| *i += n).or_insert(n);
                None
            }
            Self::Mul(c, v) => {
                let n = match v {
                    Value::Number(x) => *x,
                    Value::Register(c) => *hashmap.get(c).unwrap(),
                };
                hashmap.entry(*c).and_modify(|i| *i *= n).or_insert(0);
                None
            }
            Self::Mod(c, v) => {
                let n = match v {
                    Value::Number(x) => *x,
                    Value::Register(c) => *hashmap.get(c).unwrap(),
                };
                hashmap.entry(*c).and_modify(|i| *i %= n).or_insert(0);
                None
            }
            Self::Snd(_) | Self::Rcv(_) => panic!(),
            Self::Jgz(x, y) => {
                let x_val = match x {
                    Value::Number(n) => *n,
                    Value::Register(c) => *hashmap.get(c).unwrap(),
                };
                if x_val <= 0 {
                    None
                } else {
                    let y = match y {
                        Value::Number(n) => *n,
                        Value::Register(c) => *hashmap.get(c).unwrap(),
                    };
                    Some(y)
                }
            }
        }
    }
}
pub fn part_one(input: &str) -> Option<isize> {
    let vec = parse(input);
    let mut hashmap = HashMap::new();
    let mut sound = None;
    let mut instruction_pointer = 0;
    while instruction_pointer < vec.len() {
        let (recovered, jump) = vec[instruction_pointer].execute_part1(&mut hashmap, &mut sound);
        if let Some(x) = recovered {
            return Some(x);
        }
        if let Some(x) = jump {
            instruction_pointer = (isize::try_from(instruction_pointer).unwrap() + x)
                .try_into()
                .unwrap();
        } else {
            instruction_pointer += 1;
        }
    }
    unreachable!()
}

pub fn part_two(input: &str) -> Option<isize> {
    let vec = parse(input);
    let mut hashmap0 = HashMap::new();
    let mut hashmap1 = HashMap::new();
    let mut ins_ptr0 = 0;
    let mut ins_ptr1 = 0;
    let mut send_buff0: Vec<isize> = Vec::new();
    let mut send_buff1: Vec<isize> = Vec::new();
    let mut ctr = 0;
    let mut waiting0 = false;
    let mut waiting1 = false;

    hashmap0.insert('p', 0);
    hashmap1.insert('p', 1);
    loop {
        if ins_ptr0 < vec.len() {
            if let Instruction::Snd(x) = &vec[ins_ptr0] {
                match x {
                    Value::Number(x) => send_buff0.push(*x),
                    Value::Register(x) => send_buff0.push(hashmap0[x]),
                }
                ins_ptr0 += 1;
            } else if let Instruction::Rcv(x) = &vec[ins_ptr0] {
                match x {
                    Value::Number(_) => panic!(),
                    Value::Register(x) => {
                        if send_buff1.is_empty() {
                            waiting0 = true;
                        } else {
                            hashmap0.insert(*x, send_buff1.remove(0));
                            ins_ptr0 += 1;
                            waiting0 = false;
                        }
                    }
                }
            } else {
                let jmp = &vec[ins_ptr0].execute_part2(&mut hashmap0);
                if let Some(x) = jmp {
                    ins_ptr0 = ((isize::try_from(ins_ptr0).unwrap()) + x)
                        .try_into()
                        .unwrap();
                } else {
                    ins_ptr0 += 1;
                }
            }
        }
        if ins_ptr1 < vec.len() {
            if let Instruction::Snd(x) = &vec[ins_ptr1] {
                match x {
                    Value::Number(x) => send_buff1.push(*x),
                    Value::Register(x) => send_buff1.push(hashmap1[x]),
                }
                ins_ptr1 += 1;
                ctr += 1;
            } else if let Instruction::Rcv(x) = &vec[ins_ptr1] {
                match x {
                    Value::Number(_) => panic!(),
                    Value::Register(x) => {
                        if send_buff0.is_empty() {
                            waiting1 = true;
                        } else {
                            hashmap1.insert(*x, send_buff0.remove(0));
                            ins_ptr1 += 1;
                            waiting1 = false;
                        }
                    }
                }
            } else {
                let jmp = &vec[ins_ptr1].execute_part2(&mut hashmap1);
                if let Some(x) = jmp {
                    ins_ptr1 = ((isize::try_from(ins_ptr1).unwrap()) + x)
                        .try_into()
                        .unwrap();
                } else {
                    ins_ptr1 += 1;
                }
            }
        }
        if ins_ptr0 >= vec.len() && ins_ptr1 >= vec.len() {
            return Some(ctr);
        }
        if waiting0 && waiting1 {
            return Some(ctr);
        }
    }
}
fn parse(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(7_071));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(8_001));
    }
}
