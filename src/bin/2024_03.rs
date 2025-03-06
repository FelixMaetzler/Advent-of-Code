use std::{ops::Mul, str::FromStr};

use all_aoc::helper::parser::{
    Parser,
    character::{tag, unsigned_integer},
    sequence::{delimited, right, separated_pair},
};

all_aoc::solution!(3, 2024);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    let mut input = input;
    while !input.is_empty() {
        match mul_parser::<u32>().parse(input) {
            Ok((rest, erg)) => {
                sum += erg;
                input = rest;
            }
            Err(_) => input = &input[1..],
        }
    }
    Some(sum)
}
fn mul_parser<'a, T>() -> impl Parser<'a, T>
where
    T: FromStr + Mul<Output = T> + 'a,
{
    right(
        tag("mul"),
        enclosed_in_parentheses(separated_pair(
            unsigned_integer::<T>,
            ",",
            unsigned_integer::<T>,
        )),
    )
    .map(|(l, r)| l * r)
}
fn enclosed_in_parentheses<'a, P, A>(parser: P) -> impl Parser<'a, A>
where
    P: Parser<'a, A>,
{
    delimited(tag("("), parser, tag(")"))
}
pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    let mut enable = true;
    let mut input = input;
    while !input.is_empty() {
        if let Ok((rest, _)) = tag("do()").parse(input) {
            enable = true;
            input = rest;
        } else if let Ok((rest, _)) = tag("don't()").parse(input) {
            enable = false;
            input = rest;
        } else if let Ok((rest, erg)) = mul_parser::<u32>().parse(input) {
            if enable {
                sum += erg;
            }
            input = rest;
        } else {
            input = &input[1..];
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(159_833_790));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(48));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(89_349_241));
    }
}
