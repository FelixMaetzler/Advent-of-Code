use std::{
    collections::VecDeque,
    ops::{Index, IndexMut},
};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Position,
    Immediate,
}
impl TryFrom<IntInteger> for Mode {
    type Error = IntInteger;

    fn try_from(value: IntInteger) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Position),
            1 => Ok(Self::Immediate),
            x => Err(x),
        }
    }
}
pub type IntInteger = isize;
pub struct Intcode {
    program: Vec<IntInteger>,
    pointer: usize,
    input: VecDeque<IntInteger>,
    output: Vec<IntInteger>,
    mode: [Mode; 3],
}
impl Index<usize> for Intcode {
    type Output = IntInteger;

    fn index(&self, index: usize) -> &Self::Output {
        &self.program[index]
    }
}
impl IndexMut<usize> for Intcode {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.program[index]
    }
}
impl Intcode {
    pub fn new(program: Vec<IntInteger>) -> Self {
        Self {
            program,
            pointer: 0,
            input: VecDeque::new(),
            output: vec![],
            mode: [Mode::Position; 3],
        }
    }
    pub fn execute(&mut self) {
        while self.pointer < self.program.len() {
            let pointer = self.pointer;
            let val = self[pointer];
            self.mode[0] = Mode::try_from((val / 100) % 10).unwrap();
            self.mode[1] = Mode::try_from((val / 1_000) % 10).unwrap();
            self.mode[2] = Mode::try_from((val / 10_000) % 10).unwrap();
            match val % 100 {
                1 => self.add(),
                2 => self.multiply(),
                3 => self.input(),
                4 => self.output(),
                5 => self.jump_if_true(),
                6 => self.jump_if_false(),
                7 => self.less_than(),
                8 => self.equals(),
                99 => return,
                x => panic!("Not a valid Opcode: {x}"),
            }
        }
    }
    pub fn set_inputs(&mut self, x: impl Iterator<Item = IntInteger>) {
        self.input = x.collect()
    }
    pub fn get_outputs(&self) -> Vec<IntInteger> {
        self.output.clone()
    }
    #[inline(always)]
    fn get_first_parameter(&self) -> IntInteger {
        match self.mode[0] {
            Mode::Position => self[self[self.pointer + 1] as usize],
            Mode::Immediate => self[self.pointer + 1],
        }
    }
    #[inline(always)]
    fn get_second_parameter(&self) -> IntInteger {
        match self.mode[1] {
            Mode::Position => self[self[self.pointer + 2] as usize],
            Mode::Immediate => self[self.pointer + 2],
        }
    }
    #[inline(always)]
    fn set(&mut self, val: IntInteger) {
        debug_assert_eq!(self.mode[2], Mode::Position);
        let idx = self[self.pointer + 3];
        self[idx as usize] = val;
    }
    #[inline(always)]
    fn inc_ptr(&mut self, x: usize) {
        self.pointer += x;
    }
    #[inline(always)]
    fn add(&mut self) {
        self.set(self.get_first_parameter() + self.get_second_parameter());
        self.inc_ptr(4);
    }
    #[inline(always)]
    fn multiply(&mut self) {
        self.set(self.get_first_parameter() * self.get_second_parameter());
        self.inc_ptr(4);
    }
    #[inline(always)]
    fn input(&mut self) {
        debug_assert!(self.mode.iter().all(|m| *m == Mode::Position));
        let pos = self[self.pointer + 1];
        self[pos as usize] = self.input.pop_front().expect("Nothing to input");
        self.inc_ptr(2);
    }
    #[inline(always)]
    fn output(&mut self) {
        self.output.push(self.get_first_parameter());
        self.inc_ptr(2);
    }
    #[inline(always)]
    fn jump_if_true(&mut self) {
        if self.get_first_parameter() != 0 {
            self.pointer = self.get_second_parameter() as usize;
        } else {
            self.inc_ptr(3);
        }
    }
    #[inline(always)]
    fn jump_if_false(&mut self) {
        if self.get_first_parameter() == 0 {
            self.pointer = self.get_second_parameter() as usize;
        } else {
            self.inc_ptr(3);
        }
    }
    #[inline(always)]
    fn less_than(&mut self) {
        let s = if self.get_first_parameter() < self.get_second_parameter() {
            1
        } else {
            0
        };
        self.set(s);
        self.inc_ptr(4);
    }
    #[inline(always)]
    fn equals(&mut self) {
        let s = if self.get_first_parameter() == self.get_second_parameter() {
            1
        } else {
            0
        };
        self.set(s);
        self.inc_ptr(4);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_02() {
        assert!(equal(vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99]));
        assert!(equal(vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99]));
        assert!(equal(vec![2, 4, 4, 5, 99, 0], vec![2, 4, 4, 5, 99, 9801]));
        assert!(equal(
            vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        ));
    }
    #[test]
    fn test_day_05() {
        assert!(equal(vec![1101, 100, -1, 4, 0], vec![1101, 100, -1, 4, 99]));
    }
    fn equal(input: Vec<IntInteger>, output: Vec<IntInteger>) -> bool {
        let mut m = Intcode::new(input);
        m.execute();
        m.program == output
    }
}
