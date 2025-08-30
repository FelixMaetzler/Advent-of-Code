use core::str::FromStr;
all_aoc::solution!(16, 2017);
#[derive(Clone, Copy)]
enum Instruction {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}
impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next().unwrap() {
            's' => Ok(Self::Spin(s[1..].parse().unwrap())),
            'x' => {
                let (left, right) = s[1..].split_once('/').unwrap();
                Ok(Self::Exchange(
                    left.parse().unwrap(),
                    right.parse().unwrap(),
                ))
            }
            'p' => Ok(Self::Partner(
                s.chars().nth(1).unwrap(),
                s.chars().nth(3).unwrap(),
            )),
            _ => Err(()),
        }
    }
}
pub fn part_one(input: &str) -> Option<String> {
    let vec = parse(input);
    let mut chars = ('a'..='p').collect::<Vec<_>>();
    for ins in &vec {
        execute(&mut chars, ins);
    }
    Some(chars.iter().collect())
}

pub fn part_two(input: &str) -> Option<String> {
    let vec = parse(input);
    let mut chars = ('a'..='p').collect::<Vec<_>>();
    let cycle = find_cycle(&chars, &vec).unwrap();
    let rem = 1_000_000_000 % cycle;
    for _ in 0..rem {
        for ins in &vec {
            execute(&mut chars, ins);
        }
    }
    Some(chars.iter().collect())
}
fn find_cycle(chars: &[char], vec: &[Instruction]) -> Option<usize> {
    let mut chars = <&[char]>::clone(&chars).to_owned();
    for i in 0..usize::MAX {
        for ins in vec {
            execute(&mut chars, ins);
        }
        if chars.iter().copied().eq('a'..='p') {
            return Some(i + 1);
        }
    }
    None
}
fn execute(chars: &mut [char], ins: &Instruction) {
    match ins {
        Instruction::Spin(x) => chars.rotate_right(*x),
        Instruction::Exchange(i, j) => chars.swap(*i, *j),
        Instruction::Partner(c1, c2) => {
            let j = chars
                .iter()
                .enumerate()
                .find(|&(_, &c)| c == *c1)
                .unwrap()
                .0;
            let i = chars
                .iter()
                .enumerate()
                .find(|&(_, &c)| c == *c2)
                .unwrap()
                .0;
            chars.swap(i, j);
        }
    }
}
fn parse(input: &str) -> Vec<Instruction> {
    input
        .split(',')
        .map(|s| Instruction::from_str(s).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some("fnloekigdmpajchb".to_owned()));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some("amkjepdhifolgncb".to_owned()));
    }
}
