use std::collections::HashSet;

use all_aoc::helper::rand::Shuffle as _;

all_aoc::solution!(19, 2015);

pub fn part_one(input: &str) -> Option<usize> {
    let (molecule, vec) = parse(input);
    let mut set = HashSet::new();
    for (before, after) in &vec {
        let indices = match_overlapping_indices(&molecule, before);
        for &i in &indices {
            let mut copy = molecule.clone();
            let len = before.len();
            copy.replace_range(i..i + len, after);
            set.insert(copy);
        }
    }
    Some(set.len())
}

pub fn part_two(input: &str) -> Option<u32> {
    loop {
        let mut steps = 0;
        let mut r = 0;
        let (molecule, rules) = parse(input);
        let mut rules = rules;
        let mut mol = molecule.clone();
        while r < rules.len() {
            let (head, rep) = &rules[r];
            if let Some(i) = mol.find(rep) {
                steps += 1;
                let (left, right) = mol.split_at(i);
                let right = &right[rep.len()..];
                mol = format!("{left}{head}{right}");
                r = 0;
                rules.shuffle();
                if mol == "e" {
                    return Some(steps);
                }
            } else {
                r += 1;
            }
        }
    }
}

fn parse(input: &str) -> (String, Vec<(String, String)>) {
    let mut vec = vec![];
    let (input, molecule) = input.split_once("\n\n").unwrap();
    for line in input.lines() {
        let v: Vec<_> = line.split_ascii_whitespace().collect();
        let s1 = v[0].to_owned();
        let s2 = v[2].to_owned();
        vec.push((s1, s2));
    }
    (molecule.to_owned(), vec)
}
fn match_overlapping_indices(s: &str, substr: &str) -> Vec<usize> {
    let substr_len = substr.len();
    let windows: Vec<&str> = s
        .char_indices()
        .map(|(i, _)| &s[i..])
        .take_while(|window| window.len() >= substr_len)
        .map(|window| &window[..substr_len])
        .collect();
    let mut indices = Vec::new();
    for (i, window) in windows.iter().enumerate() {
        if window == &substr {
            indices.push(i);
        }
    }
    indices
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(576));
    }
    /*
    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(3));
    }
    */

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(207));
    }
}
