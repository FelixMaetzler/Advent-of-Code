use std::collections::HashSet;

all_aoc::solution!(6, 2017);

pub fn part_one(input: &str) -> Option<u32> {
    let mut vec = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let n = vec.len();
    let mut ctr = 1;
    let mut set = HashSet::new();
    loop {
        let max = vec.iter().max().unwrap();
        let index = vec.iter().enumerate().find(|(_, i)| *i == max).unwrap().0;
        let val = vec[index];
        vec[index] = 0;
        for i in 1..=val {
            vec[(index + i) % n] += 1;
        }
        if !set.insert(vec.clone()) {
            return Some(ctr);
        }
        ctr += 1;
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut vec = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let n = vec.len();
    let mut ctr: Option<usize> = None;
    let mut set = HashSet::new();
    let mut save = None;
    loop {
        let max = vec.iter().max().unwrap();
        let index = vec.iter().enumerate().find(|(_, i)| *i == max).unwrap().0;
        let val = vec[index];
        vec[index] = 0;
        for i in 1..=val {
            vec[(index + i) % n] += 1;
        }
        if let Some(x) = save.clone()
            && x == vec
        {
            return Some(ctr.unwrap());
        }
        if !set.insert(vec.clone()) && save.is_none() {
            save = Some(vec.clone());
            ctr = Some(0);
        }
        if ctr.is_some() {
            ctr = Some(ctr.unwrap() + 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(3_156));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1610));
    }
}
