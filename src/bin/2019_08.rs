use std::fmt::Display;

use all_aoc::helper::misc::Joinable;
all_aoc::solution!(8, 2019);

#[derive(Clone, Copy)]
enum Color {
    Black,
    White,
    Transparent,
}
impl From<u32> for Color {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Black,
            1 => Self::White,
            2 => Self::Transparent,
            _ => unreachable!(),
        }
    }
}
impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Black => write!(f, " "),
            Self::White => write!(f, "#"),
            Self::Transparent => write!(f, " "),
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    part_one_runner(input, (25, 6))
}

pub fn part_two(input: &str) -> Option<String> {
    part_two_runner(input, (25, 6))
}
fn part_one_runner(input: &str, ratio: (usize, usize)) -> Option<usize> {
    let vec = parse(input);
    let chunks = vec
        .as_slice()
        .chunks(ratio.0 * ratio.1)
        .map(|c| (c, c.iter().filter(|n| **n == 0).count()))
        .min_by_key(|s| s.1)
        .unwrap()
        .0;
    Some(chunks.iter().filter(|n| **n == 1).count() * chunks.iter().filter(|n| **n == 2).count())
}
fn part_two_runner(input: &str, ratio: (usize, usize)) -> Option<String> {
    let vec = parse(input);
    let vec: Vec<_> = vec.iter().map(|n| Color::from(*n)).collect();
    let mut ret = vec![Vec::with_capacity(vec.len() / (ratio.0 * ratio.1)); ratio.0 * ratio.1];
    vec.as_slice()
        .chunks(ratio.0 * ratio.1)
        .for_each(|c| c.iter().enumerate().for_each(|(i, n)| ret[i].push(*n)));
    let erg: Vec<_> = ret.iter().map(|v| resolve(v)).collect();
    let s = erg
        .chunks(ratio.0)
        .map(|l| l.iter().map(|c| c.to_string()).join(""))
        .join("\n");

    Some(s)
}
fn mapping(back: Color, front: Color) -> Color {
    match (back, front) {
        (_, Color::Black) => Color::Black,
        (_, Color::White) => Color::White,
        (x, Color::Transparent) => x,
    }
}
fn resolve(input: &[Color]) -> Color {
    resolve_recursive(input, 0)
}
fn resolve_recursive(input: &[Color], index: usize) -> Color {
    if index == input.len() {
        return Color::Transparent;
    }
    match mapping(input[index + 1], input[index]) {
        Color::Black => Color::Black,
        Color::White => Color::White,
        Color::Transparent => resolve_recursive(input, index + 1),
    }
}
fn parse(input: &str) -> Vec<u32> {
    input.chars().map(|c| c.to_digit(10).unwrap()).collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(2_250));
    }

    #[test]
    fn test_part_two() {
        let result = part_two_runner(&all_aoc::cli::read_examples_file(DAY), (2, 2));
        assert_eq!(result, Some(" #\n# ".to_string()));
    }
}
