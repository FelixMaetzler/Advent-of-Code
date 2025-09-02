use core::str::FromStr;
use std::collections::HashMap;

all_aoc::solution!(23, 2017);
#[derive(Clone, Copy)]
enum Instruction {
    Set(Register, Value),
    Sub(Register, Value),
    Mul(Register, Value),
    Jnz(Value, Value),
}
impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (prefix, rest) = s.split_at(4);
        let (first, last) = rest.split_once(' ').unwrap();
        match prefix {
            "set " => Ok(Self::Set(first.parse().unwrap(), last.parse().unwrap())),
            "sub " => Ok(Self::Sub(first.parse().unwrap(), last.parse().unwrap())),
            "mul " => Ok(Self::Mul(first.parse().unwrap(), last.parse().unwrap())),
            "jnz " => Ok(Self::Jnz(first.parse().unwrap(), last.parse().unwrap())),
            _ => Err(s.into()),
        }
    }
}
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Register(char);
impl FromStr for Register {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 1 {
            Ok(Self(s.chars().next().unwrap()))
        } else {
            Err(s.into())
        }
    }
}
#[derive(Clone, Copy)]
enum Value {
    Val(isize),
    Reg(Register),
}
impl FromStr for Value {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        isize::from_str(s).map_or_else(
            |_| Register::from_str(s).map_or_else(|_| Err(s.into()), |x| Ok(Self::Reg(x))),
            |x| Ok(Self::Val(x)),
        )
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let vec = parse(input);
    let mut ins_ptr = 0;
    let mut map = HashMap::new();
    ('a'..='h').for_each(|c| {
        map.insert(Register(c), 0);
    });
    let mut ctr = 0;
    while ins_ptr < vec.len() {
        match vec[ins_ptr] {
            Instruction::Set(x, y) => match y {
                Value::Val(y) => {
                    map.entry(x).and_modify(|i| *i = y);
                }
                Value::Reg(y) => {
                    let y = map[&y];
                    map.entry(x).and_modify(|i| *i = y);
                }
            },
            Instruction::Sub(x, y) => match y {
                Value::Val(y) => {
                    map.entry(x).and_modify(|i| *i -= y);
                }
                Value::Reg(y) => {
                    let y = map[&y];
                    map.entry(x).and_modify(|i| *i -= y);
                }
            },
            Instruction::Mul(x, y) => {
                match y {
                    Value::Val(y) => {
                        map.entry(x).and_modify(|i| *i *= y);
                    }
                    Value::Reg(y) => {
                        let y = map[&y];
                        map.entry(x).and_modify(|i| *i *= y);
                    }
                }
                ctr += 1;
            }
            Instruction::Jnz(x, y) => {
                let x = match x {
                    Value::Val(x) => x,
                    Value::Reg(x) => map[&x],
                };
                if x != 0 {
                    match y {
                        Value::Val(y) => {
                            ins_ptr = (isize::try_from(ins_ptr).unwrap() + y).try_into().unwrap();
                        }
                        Value::Reg(y) => {
                            ins_ptr = (isize::try_from(ins_ptr).unwrap() + map[&y])
                                .try_into()
                                .unwrap();
                        }
                    }
                    ins_ptr -= 1;
                }
            }
        }
        ins_ptr += 1;
    }
    Some(ctr)
}
#[expect(clippy::many_single_char_names, reason = "its the real name of it")]
pub const fn part_two(_: &str) -> Option<u32> {
    let mut h = 0;
    let mut b = 84;
    b *= 100;
    b += 100_000;
    let mut c = b;
    c += 17_000;
    loop {
        let mut f = 1;
        let mut d = 2;

        while d != b {
            if b % d == 0 {
                f = 0;
            }
            d += 1;
        }
        if f == 0 {
            h += 1;
        }
        let mut g = b;
        g -= c;
        if g == 0 {
            return Some(h);
        }
        b += 17;
    }
}
fn parse(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(6_724));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(903));
    }
}
