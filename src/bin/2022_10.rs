use core::str::FromStr;

use all_aoc::helper::grid::{Grid as _, dense::DenseGrid};

all_aoc::solution!(10, 2022);
enum Instruction {
    Noop,
    Addx(i32),
}
impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            Ok(Self::Noop)
        } else {
            Ok(Self::Addx(s.split_once(' ').unwrap().1.parse().unwrap()))
        }
    }
}
pub fn part_one(input: &str) -> Option<i32> {
    let input = parse(input);
    let input = input.flat_map(|i| match i {
        Instruction::Noop => vec![Instruction::Noop],
        Instruction::Addx(x) => vec![Instruction::Noop, Instruction::Addx(x)],
    });
    let mut x = 1;
    let mut sum = 0;
    for (i, ins) in input.enumerate() {
        let i = i32::try_from(i).unwrap() + 1;
        if matches_sequence(i) {
            sum += i * x;
        }
        match ins {
            Instruction::Noop => {}
            Instruction::Addx(y) => x += y,
        }
    }
    Some(sum)
}
const fn matches_sequence(x: i32) -> bool {
    x == 20 || (x >= 40 && x % 40 == 20)
}

pub fn part_two(input: &str) -> Option<String> {
    let input = parse(input);
    let input = input.flat_map(|i| match i {
        Instruction::Noop => vec![Instruction::Noop],
        Instruction::Addx(x) => vec![Instruction::Noop, Instruction::Addx(x)],
    });
    let mut x = 1;
    let mut sprite = [false; 6 * 40];
    for (i, ins) in input.enumerate() {
        let i = i32::try_from(i).unwrap() + 1;
        if [x - 1, x, x + 1].contains(&((i + 5 * 40 - 1) % 40)) {
            sprite[usize::try_from(i).unwrap() - 1] = true;
        }
        match ins {
            Instruction::Noop => {}
            Instruction::Addx(y) => x += y,
        }
    }

    let grid = DenseGrid::from_iter(sprite.into_iter(), 40);

    let mut chars = grid.split_width(5);
    for g in &mut chars {
        g.remove_col(4);
    }
    let chars = chars;
    Some(chars.into_iter().map(|g| grid_to_char(&g)).collect())
}
fn grid_to_char(grid: &DenseGrid<bool>) -> char {
    let vec = grid.iter().copied().collect::<Vec<_>>();

    if vec == E.iter().flatten().copied().collect::<Vec<_>>() {
        'E'
    } else if vec == C.iter().flatten().copied().collect::<Vec<_>>() {
        'C'
    } else if vec == Z.iter().flatten().copied().collect::<Vec<_>>() {
        'Z'
    } else if vec == U.iter().flatten().copied().collect::<Vec<_>>() {
        'U'
    } else if vec == A.iter().flatten().copied().collect::<Vec<_>>() {
        'A'
    } else if vec == L.iter().flatten().copied().collect::<Vec<_>>() {
        'L'
    } else if vec == R.iter().flatten().copied().collect::<Vec<_>>() {
        'R'
    } else {
        unimplemented!()
    }
}
fn parse(input: &str) -> impl Iterator<Item = Instruction> {
    input.lines().map(|l| Instruction::from_str(l).unwrap())
}
const E: [[bool; 4]; 6] = [
    [true, true, true, true],
    [true, false, false, false],
    [true, true, true, false],
    [true, false, false, false],
    [true, false, false, false],
    [true, true, true, true],
];
const C: [[bool; 4]; 6] = [
    [false, true, true, false],
    [true, false, false, true],
    [true, false, false, false],
    [true, false, false, false],
    [true, false, false, true],
    [false, true, true, false],
];
const Z: [[bool; 4]; 6] = [
    [true, true, true, true],
    [false, false, false, true],
    [false, false, true, false],
    [false, true, false, false],
    [true, false, false, false],
    [true, true, true, true],
];
const U: [[bool; 4]; 6] = [
    [true, false, false, true],
    [true, false, false, true],
    [true, false, false, true],
    [true, false, false, true],
    [true, false, false, true],
    [false, true, true, false],
];
const A: [[bool; 4]; 6] = [
    [false, true, true, false],
    [true, false, false, true],
    [true, false, false, true],
    [true, true, true, true],
    [true, false, false, true],
    [true, false, false, true],
];
const L: [[bool; 4]; 6] = [
    [true, false, false, false],
    [true, false, false, false],
    [true, false, false, false],
    [true, false, false, false],
    [true, false, false, false],
    [true, true, true, true],
];
const R: [[bool; 4]; 6] = [
    [true, true, true, false],
    [true, false, false, true],
    [true, false, false, true],
    [true, true, true, false],
    [true, false, true, false],
    [true, false, false, true],
];
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(13_140));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(16_020));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some("ECZUZALR".to_owned()));
    }
}
