use core::str::FromStr;

all_aoc::solution!(25, 2016);
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
        match s {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            "c" => Ok(Self::C),
            "d" => Ok(Self::D),
            _ => Err(format!("'{s}' ist kein Register")),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Instruction {
    Copy(CopySource, Register),
    Inc(Register),
    Dec(Register),
    Jump(CopySource, isize),
    Out(CopySource),
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
                    isize::from_str(n).unwrap(),
                ))
            }
            "out " => Ok(Self::Out(CopySource::from_str(&work_str).unwrap())),
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
        if s.chars().nth(0).unwrap().is_numeric() {
            Ok(Self::Index(isize::from_str(s).unwrap()))
        } else {
            Ok(Self::Reg(Register::from_str(s).unwrap()))
        }
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
    const fn part1(i: isize) -> Self {
        Self {
            a: i,
            b: 0,
            c: 0,
            d: 0,
        }
    }
}
impl Instruction {
    const fn execute(&self, state: &mut State) -> (Option<isize>, Option<isize>) {
        match self {
            Self::Copy(cpysrc, to_reg) => match cpysrc {
                CopySource::Index(val) => {
                    state.set(*to_reg, *val);
                    (None, None)
                }
                CopySource::Reg(from_reg) => {
                    let val = state.get(*from_reg);
                    state.set(*to_reg, val);
                    (None, None)
                }
            },
            Self::Inc(reg) => {
                let val = state.get(*reg) + 1;
                state.set(*reg, val);
                (None, None)
            }
            Self::Dec(reg) => {
                let val = state.get(*reg) - 1;
                state.set(*reg, val);
                (None, None)
            }
            Self::Jump(cpysrc, amount) => {
                let val = match cpysrc {
                    CopySource::Index(val) => *val,
                    CopySource::Reg(reg) => state.get(*reg),
                };
                if val == 0 {
                    (None, None)
                } else {
                    (Some(*amount), None)
                }
            }
            Self::Out(r) => match r {
                CopySource::Index(x) => (None, Some(*x)),
                CopySource::Reg(x) => (None, Some(state.get(*x))),
            },
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let vec = parse(input);
    let ret = (1..u32::MAX)
        //.into_par_iter()
        .map(|state| (State::part1(state.try_into().unwrap()), state))
        .find(|(start, _)| executer(&vec, *start, 100).is_some());
    Some(ret.unwrap().1)
}

pub const fn part_two(_: &str) -> Option<u32> {
    None
}

fn executer(vec: &[Instruction], start: State, length: usize) -> Option<usize> {
    //let vec = parse(input);
    let mut instruction_pointer = 0;
    let mut state = start;
    let mut last = None;
    let mut counter = 0;
    while instruction_pointer < vec.len() && counter <= length {
        let (jmp, out) = vec[instruction_pointer].execute(&mut state);
        if let Some(x) = jmp {
            instruction_pointer = instruction_pointer.checked_add_signed(x).unwrap();
        } else {
            instruction_pointer += 1;
        }
        if let Some(out) = out {
            if out != 0 && out != 1 {
                return None;
            }
            if last.is_none() {
                last = Some(out);
            } else if out == last.unwrap() {
                return None;
            } else {
                last = Some(out);
                counter += 1;
            }
        }
    }

    Some(length)
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
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(196));
    }
}
