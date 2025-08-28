use core::str::FromStr;

all_aoc::solution!(23, 2016);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Register {
    A,
    B,
    C,
    D,
}
impl FromStr for Register {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //assert_eq!(s.len(), 1);
        match s.chars().nth(0).unwrap() {
            'a' => Ok(Self::A),
            'b' => Ok(Self::B),
            'c' => Ok(Self::C),
            'd' => Ok(Self::D),
            _ => Err(format!("'{s}' ist kein Register")),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Instruction {
    Copy(CopySource, Register),
    Inc(Register),
    Dec(Register),
    Jump(CopySource, CopySource),
    Toggle(Register),
    Invalid(CopySource, CopySource),
}
impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut work_str = s.to_owned();
        let first: String = work_str.drain(0..4).collect();
        match first.as_str() {
            "cpy " => {
                let (cpysrc, reg) = work_str.split_once(' ').unwrap();
                Ok(Self::Copy(
                    CopySource::from_str(cpysrc).unwrap(),
                    Register::from_str(reg).unwrap(),
                ))
            }
            "inc " => Ok(Self::Inc(Register::from_str(&work_str).unwrap())),
            "dec " => Ok(Self::Dec(Register::from_str(&work_str).unwrap())),
            "jnz " => {
                let (reg, n) = work_str.split_once(' ').unwrap();
                Ok(Self::Jump(
                    CopySource::from_str(reg).unwrap(),
                    CopySource::from_str(n).unwrap(),
                ))
            }
            "tgl " => Ok(Self::Toggle(Register::from_str(&work_str).unwrap())),
            _ => Err("ist keine Instruction".to_owned()),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum CopySource {
    Index(isize),
    Reg(Register),
}
impl FromStr for CopySource {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num = isize::from_str(s);
        num.map_or_else(
            |_| Ok(Self::Reg(Register::from_str(s).unwrap())),
            |x| Ok(Self::Index(x)),
        )
    }
}
#[derive(Debug, Default, Clone, Copy)]
struct State {
    a: isize,
    b: isize,
    c: isize,
    d: isize,
}
impl State {
    const fn get(&self, reg: Register) -> isize {
        match reg {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
        }
    }
    const fn set(&mut self, reg: Register, val: isize) {
        match reg {
            Register::A => self.a = val,
            Register::B => self.b = val,
            Register::C => self.c = val,
            Register::D => self.d = val,
        }
    }
    const fn part1() -> Self {
        Self {
            a: 7,
            b: 0,
            c: 0,
            d: 0,
        }
    }
    const fn part2() -> Self {
        Self {
            a: 12,
            b: 0,
            c: 0,
            d: 0,
        }
    }
}
impl Instruction {
    fn execute(&self, state: &mut State, list: &mut [Self], ins_pointer: usize) -> Option<isize> {
        match self {
            Self::Copy(cpysrc, to_reg) => match cpysrc {
                CopySource::Index(val) => {
                    state.set(*to_reg, *val);
                    None
                }
                CopySource::Reg(from_reg) => {
                    let val = state.get(*from_reg);
                    state.set(*to_reg, val);
                    None
                }
            },
            Self::Inc(reg) => {
                let val = state.get(*reg) + 1;
                state.set(*reg, val);
                None
            }
            Self::Dec(reg) => {
                let val = state.get(*reg) - 1;
                state.set(*reg, val);
                None
            }
            Self::Jump(cpysrc, amount) => {
                let val = match cpysrc {
                    CopySource::Index(val) => *val,
                    CopySource::Reg(reg) => state.get(*reg),
                };
                if val == 0 {
                    None
                } else {
                    match amount {
                        CopySource::Index(x) => Some(*x),
                        CopySource::Reg(y) => Some(state.get(*y)),
                    }
                }
            }
            Self::Toggle(x) => {
                let new_index = isize::try_from(ins_pointer).unwrap() + state.get(*x);
                if new_index.is_negative() || usize::try_from(new_index).unwrap() >= list.len() {
                    return None;
                }
                let new_index = usize::try_from(new_index).unwrap();
                let new_ins = match list[new_index] {
                    Self::Copy(cpysrc, to_reg) => Self::Jump(cpysrc, CopySource::Reg(to_reg)),
                    Self::Inc(x) => Self::Dec(x),
                    Self::Dec(x) | Self::Toggle(x) => Self::Inc(x),
                    Self::Jump(src, i) => match i {
                        CopySource::Index(_) => Self::Invalid(src, i),
                        CopySource::Reg(x) => Self::Copy(src, x),
                    },
                    Self::Invalid(x, y) => Self::Jump(x, y),
                };
                list[new_index] = new_ins;
                None
            }
            Self::Invalid(_, _) => None,
        }
    }
}

pub fn part_one(input: &str) -> Option<isize> {
    Some(executer(input, State::part1()))
}

pub fn part_two(input: &str) -> Option<isize> {
    Some(executer(input, State::part2()))
}
fn executer(input: &str, start: State) -> isize {
    let mut vec = parse(input);
    let mut instruction_pointer = 0;
    let mut state = start;

    while instruction_pointer < vec.len() {
        let ins = vec[instruction_pointer];
        let jmp = ins.execute(&mut state, &mut vec, instruction_pointer);
        if let Some(x) = jmp {
            instruction_pointer = instruction_pointer.checked_add_signed(x).unwrap();
        } else {
            instruction_pointer += 1;
        }
    }
    state.a
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
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(12_663));
    }
    #[cfg(feature = "expensive")]
    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(479_009_223));
    }
}
