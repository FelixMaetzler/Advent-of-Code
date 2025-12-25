use core::str::FromStr;
use std::collections::HashMap;

use all_aoc::helper::{bitmask::Bitmask as _, permutations::IteratorCombinator as _};

all_aoc::solution!(10, 2025);

type Pattern = Vec<u16>;
type PatternCost = u32;

#[derive(Debug)]
struct PatternDB {
    by_parity: Vec<HashMap<Pattern, PatternCost>>,
}
fn parity_index(v: &[u16]) -> usize {
    v.iter()
        .enumerate()
        .fold(0_usize, |acc, (i, &x)| acc | (((x & 1) as usize) << i))
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Machine {
    indicator_lights: u16,
    buttons: Vec<u16>,
    joltage: Vec<u16>,
}
impl Machine {
    fn calc_min_button_presses_for_indicator_lights(&self) -> u32 {
        self.single_button_presses()
            .map(|v| v.len().try_into().unwrap())
            .min()
            .unwrap()
    }
    fn single_button_presses(&self) -> impl Iterator<Item = Vec<usize>> {
        (0..self.buttons.len()).powerset().filter(|v| {
            v.iter().fold(0, |acc, &i| next(acc, self.buttons[i])) == self.indicator_lights
        })
    }
    fn build_patterns(&self) -> PatternDB {
        let n = self.buttons.len();
        let vars = self.joltage.len();
        let num_parities = 1 << vars;

        let mut by_parity = vec![HashMap::<Pattern, PatternCost>::new(); num_parities];

        for mask in 0..(1 << n) {
            let mut pattern = vec![0; vars];
            let mut cost = 0;

            for b in 0..n {
                if (mask >> b) & 1 == 1 {
                    cost += 1;
                    for (i, item) in pattern.iter_mut().enumerate().take(vars) {
                        *item += u16::from(self.buttons[b].get_bit(i));
                    }
                }
            }

            let pidx = parity_index(&pattern);
            let entry = by_parity[pidx].entry(pattern).or_insert(cost);
            *entry = (*entry).min(cost);
        }

        PatternDB { by_parity }
    }

    fn calc_min_joltage(&self) -> Option<u32> {
        let patterns = self.build_patterns();
        let mut cache = HashMap::new();
        solve_goal(&self.joltage, &patterns, &mut cache)
    }
}
fn solve_goal(
    goal: &[u16],
    patterns: &PatternDB,
    cache: &mut HashMap<Vec<u16>, Option<u32>>,
) -> Option<u32> {
    if let Some(ret) = cache.get(goal) {
        return *ret;
    }
    if goal.iter().all(|&v| v == 0) {
        return Some(0);
    }

    let mut ret = None;
    let pidx = parity_index(goal);

    'outer: for (pattern, &cost) in &patterns.by_parity[pidx] {
        let mut next = goal.to_vec();

        for i in 0..next.len() {
            if pattern[i] > next[i] {
                continue 'outer;
            }
            next[i] = (next[i] - pattern[i]) / 2;
        }

        if let Some(sub) = solve_goal(&next, patterns, cache) {
            let cand = cost + 2 * sub;
            ret = Some(ret.map_or(cand, |x: u32| x.min(cand)));
        }
    }

    cache.insert(goal.to_vec(), ret);
    ret
}
const fn next(from: u16, button: u16) -> u16 {
    from ^ button
}
impl FromStr for Machine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_start_matches('[').trim_end_matches('}');
        let (indicator_lights, rest) = s.split_once(']').unwrap();
        let (buttons, joltage) = rest.split_once('{').unwrap();
        let indicator_lights = indicator_lights.chars().rev().fold(0, |acc, e| {
            (acc << 1)
                + match e {
                    '.' => 0,
                    '#' => 1,
                    _ => unreachable!(),
                }
        });
        let buttons = buttons
            .trim()
            .split_ascii_whitespace()
            .map(|b| b.trim_start_matches('(').trim_end_matches(')'))
            .map(|l| {
                let mut x: u16 = 0;
                l.split(',')
                    .map(|n| n.parse().unwrap())
                    .for_each(|n| x.set_bit(n, true));
                x
            })
            .collect();
        let joltage = joltage.split(',').map(|n| n.parse().unwrap()).collect();
        Ok(Self {
            indicator_lights,
            buttons,
            joltage,
        })
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse(input)
            .map(|m| m.calc_min_button_presses_for_indicator_lights())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(parse(input).map(|m| m.calc_min_joltage().unwrap()).sum())
}
fn parse(input: &str) -> impl Iterator<Item = Machine> {
    input.lines().map(|l| l.parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, (Some(558)));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(33));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(20_317));
    }
}
