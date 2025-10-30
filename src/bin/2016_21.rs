use core::{ops::RangeInclusive, str::FromStr};

use all_aoc::helper::permutations::IteratorPermutator as _;

all_aoc::solution!(21, 2016);
enum Instruction {
    SwapPos(usize, usize),
    SwapLetter(char, char),
    RotatePosRight(usize),
    RotatePosLeft(usize),
    RotateLetter(char),
    Reverse(RangeInclusive<usize>),
    Move(usize, usize),
}
impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.split(' ').collect::<Vec<_>>();
        if s.starts_with("swap position") {
            Ok(Self::SwapPos(v[2].parse().unwrap(), v[5].parse().unwrap()))
        } else if s.starts_with("swap letter") {
            Ok(Self::SwapLetter(
                v[2].parse().unwrap(),
                v[5].parse().unwrap(),
            ))
        } else if s.starts_with("rotate right") {
            Ok(Self::RotatePosRight(v[2].parse().unwrap()))
        } else if s.starts_with("rotate left") {
            Ok(Self::RotatePosLeft(v[2].parse().unwrap()))
        } else if s.starts_with("rotate based") {
            Ok(Self::RotateLetter(v[6].parse().unwrap()))
        } else if s.starts_with("reverse") {
            Ok(Self::Reverse(v[2].parse().unwrap()..=v[4].parse().unwrap()))
        } else if s.starts_with("move") {
            Ok(Self::Move(v[2].parse().unwrap(), v[5].parse().unwrap()))
        } else {
            Err(())
        }
    }
}
struct Str<const T: usize>([char; T]);
impl<const T: usize> Str<T> {
    fn execute(&mut self, ins: Instruction) {
        match ins {
            Instruction::SwapPos(x, y) => self.swap_pos(x, y),
            Instruction::SwapLetter(x, y) => self.swap_letter(x, y),
            Instruction::RotatePosRight(x) => self.rotate_right(x),
            Instruction::RotatePosLeft(x) => self.rotate_left(x),
            Instruction::RotateLetter(x) => self.rotate_letter(x),
            Instruction::Reverse(x) => self.reverse(x),
            Instruction::Move(x, y) => self.moving(x, y),
        }
    }
    fn find_index(&self, c: char) -> Option<usize> {
        self.0
            .iter()
            .enumerate()
            .find(|&(_, &char)| char == c)
            .map(|s| s.0)
    }
    const fn swap_pos(&mut self, n1: usize, n2: usize) {
        self.0.swap(n1, n2);
    }
    fn swap_letter(&mut self, c1: char, c2: char) {
        let n1 = self.find_index(c1).unwrap();
        let n2 = self.find_index(c2).unwrap();
        self.swap_pos(n1, n2);
    }
    const fn rotate_right(&mut self, count: usize) {
        self.0.rotate_right(count % T);
    }
    const fn rotate_left(&mut self, count: usize) {
        self.0.rotate_left(count);
    }
    fn rotate_letter(&mut self, c: char) {
        let index = self.find_index(c).unwrap();
        let rotate = if index >= 4 { 2 + index } else { 1 + index };
        self.rotate_right(rotate);
    }
    fn reverse(&mut self, range: RangeInclusive<usize>) {
        self.0[range].reverse();
    }
    fn moving(&mut self, n1: usize, n2: usize) {
        let mut v = Vec::from(self.0);
        let c = v.remove(n1);
        v.insert(n2, c);

        let l = self.0.len();
        self.0.copy_from_slice(&v[..l]);
    }
}
pub fn part_one(input: &str) -> Option<String> {
    let str = Str(['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h']);
    Some(solve_part1(input, str))
}

pub fn part_two(input: &str) -> Option<String> {
    const CH_ARRAY: [char; 8] = ['f', 'b', 'g', 'd', 'c', 'e', 'a', 'h'];
    const T: usize = CH_ARRAY.len();
    let pw = Str(CH_ARRAY);
    let finished: String = pw.0.iter().collect();
    let x: [char; T] =
        pw.0.into_iter()
            .permutations(T)
            .map(|v| helper(&v))
            .find(|&v| solve_part1(input, Str(v)) == finished)
            .unwrap();
    Some(x.iter().collect())
}
fn solve_part1<const T: usize>(input: &str, str: Str<T>) -> String {
    let instructions = parse(input);
    let mut str = str;
    for ins in instructions {
        str.execute(ins);
    }
    str.0.iter().collect()
}
fn helper<const T: usize>(v: &[char]) -> [char; T] {
    assert_eq!(T, v.len());
    let mut ret = ['0'; T];
    ret[..T].copy_from_slice(&v[..T]);
    ret
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
        assert_eq!(result, Some("dbfgaehc".to_owned()));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some("aghfcdeb".to_owned()));
    }
}
