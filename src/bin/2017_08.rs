use core::str::FromStr;
use std::collections::HashMap;

all_aoc::solution!(8, 2017);
struct Instruction {
    reg: Register,
    op: Operation,
    val: isize,
    statement: Statement,
}
impl Instruction {
    fn execute(&self, map: &mut HashMap<Register, isize>) {
        if !self.statement.is_true(map) {
            return;
        }
        let diff = match self.op {
            Operation::Inc => self.val,
            Operation::Dec => -self.val,
        };
        map.entry(self.reg.clone())
            .and_modify(|v| *v += diff)
            .or_insert(diff);
    }
}
impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_once(' ').unwrap();
        let reg = Register::from_str(first)?;
        let (first, second) = second.split_once(' ').unwrap();
        let op = Operation::from_str(first)?;
        let (first, second) = second.split_once(' ').unwrap();
        let val = first.parse().unwrap();
        let (_, second) = second.split_once(' ').unwrap();
        let statement = Statement::from_str(second)?;
        Ok(Self {
            reg,
            op,
            val,
            statement,
        })
    }
}
#[derive(Clone, PartialEq, PartialOrd, Hash, Debug, Eq)]
struct Register(String);
impl FromStr for Register {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_owned()))
    }
}
enum Operation {
    Inc,
    Dec,
}
impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "inc" {
            Ok(Self::Inc)
        } else if s == "dec" {
            Ok(Self::Dec)
        } else {
            Err(())
        }
    }
}
struct Statement {
    reg: Register,
    con: Condition,
    val: isize,
}
impl Statement {
    fn is_true(&self, map: &HashMap<Register, isize>) -> bool {
        let reg_val = *map.get(&self.reg).unwrap_or(&0);
        match self.con {
            Condition::Greater => reg_val > self.val,
            Condition::Smaller => reg_val < self.val,
            Condition::Equal => reg_val == self.val,
            Condition::GreaterThan => reg_val >= self.val,
            Condition::SmallerThan => reg_val <= self.val,
            Condition::NotEqual => reg_val != self.val,
        }
    }
}
impl FromStr for Statement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec = s.split_ascii_whitespace().collect::<Vec<_>>();
        let reg = Register::from_str(vec[0])?;
        let con = Condition::from_str(vec[1])?;
        let val = isize::from_str(vec[2]).unwrap();
        Ok(Self { reg, con, val })
    }
}
enum Condition {
    Greater,
    Smaller,
    Equal,
    GreaterThan,
    SmallerThan,
    NotEqual,
}
impl FromStr for Condition {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ">" => Ok(Self::Greater),
            "<" => Ok(Self::Smaller),
            "==" => Ok(Self::Equal),
            ">=" => Ok(Self::GreaterThan),
            "<=" => Ok(Self::SmallerThan),
            "!=" => Ok(Self::NotEqual),
            _ => Err(()),
        }
    }
}
pub fn part_one(input: &str) -> Option<isize> {
    let vec = parse(input);
    let mut map = HashMap::new();
    for ins in &vec {
        ins.execute(&mut map);
    }
    Some(*map.values().max().unwrap())
}

pub fn part_two(input: &str) -> Option<isize> {
    let vec = parse(input);
    let mut map = HashMap::new();
    let mut highest = 0;
    for ins in vec {
        ins.execute(&mut map);
        if let Some(&max) = map.values().max()
            && max > highest
        {
            highest = max;
        }
    }
    Some(highest)
}
fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| Instruction::from_str(line).unwrap())
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(4_647));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(5_590));
    }
}
