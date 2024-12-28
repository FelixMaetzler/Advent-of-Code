use std::ops::{Index, IndexMut};

pub type IntInteger = usize;
pub struct Intcode {
    program: Vec<IntInteger>,
    pointer: usize,
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
        }
    }
    pub fn execute(&mut self) {
        while self.pointer < self.program.len() {
            let pointer = self.pointer;
            match self[pointer] {
                1 => self.add(),
                2 => self.multiply(),
                99 => return,
                x => panic!("Not a valid Opcode: {x}"),
            }
        }
    }
    #[inline(always)]
    fn get_first_arg(&self) -> IntInteger {
        self[self[self.pointer + 1]]
    }
    #[inline(always)]
    fn get_second_arg(&self) -> IntInteger {
        self[self[self.pointer + 2]]
    }
    #[inline(always)]
    fn set(&mut self, val: IntInteger) {
        let idx = self[self.pointer + 3];
        self[idx] = val;
    }
    #[inline(always)]
    fn add(&mut self) {
        self.set(self.get_first_arg() + self.get_second_arg());
        self.inc_ptr(4);
    }
    #[inline(always)]
    fn multiply(&mut self) {
        self.set(self.get_first_arg() * self.get_second_arg());
        self.inc_ptr(4);
    }
    #[inline(always)]
    fn inc_ptr(&mut self, x: usize) {
        self.pointer += x;
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
    fn equal(input: Vec<IntInteger>, output: Vec<IntInteger>) -> bool {
        let mut m = Intcode::new(input);
        m.execute();
        m.program == output
    }
}
