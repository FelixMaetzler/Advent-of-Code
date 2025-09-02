use std::collections::HashMap;

all_aoc::solution!(25, 2017);
struct State {
    write_if_zero: bool,
    write_if_one: bool,
    move_if_zero: isize,
    move_if_one: isize,
    if_zero: char,
    if_one: char,
}
pub fn part_one(input: &str) -> Option<u32> {
    let (checksum, map) = parse(input);
    let mut tape = HashMap::with_capacity(checksum);
    let mut curr = 0;
    let mut curr_state = 'A';
    for _ in 0..checksum {
        let state = &map[&curr_state];
        let curr_val = *tape.get(&curr).unwrap_or(&false);
        if curr_val {
            tape.insert(curr, state.write_if_one);
            curr += state.move_if_one;
            curr_state = state.if_one;
        } else {
            tape.insert(curr, state.write_if_zero);
            curr += state.move_if_zero;
            curr_state = state.if_zero;
        }
    }
    Some(tape.values().filter(|&e| *e).count().try_into().unwrap())
}

pub const fn part_two(_: &str) -> Option<u32> {
    None
}
fn parse(input: &str) -> (usize, HashMap<char, State>) {
    let mut map = HashMap::new();
    let mut it = input.split("\n\n");
    let first_paragraph = it.next().unwrap();
    let checksum = first_paragraph
        .lines()
        .nth(1)
        .unwrap()
        .split(' ')
        .nth(5)
        .unwrap()
        .parse()
        .unwrap();
    for paragrah in it {
        let mut it = paragrah.lines();
        let c = it
            .next()
            .unwrap()
            .split(' ')
            .nth(2)
            .unwrap()
            .chars()
            .next()
            .unwrap();
        it.next().unwrap();
        let write_if_zero = match it
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .nth(4)
            .unwrap()
            .chars()
            .next()
            .unwrap()
        {
            '1' => true,
            '0' => false,
            _ => unreachable!(),
        };
        let move_if_zero = match it.next().unwrap().split_ascii_whitespace().nth(6).unwrap() {
            "right." => 1,
            "left." => -1,
            x => unreachable!("{}", x),
        };
        let if_zero = it
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .nth(4)
            .unwrap()
            .chars()
            .next()
            .unwrap();
        it.next().unwrap();
        let write_if_one = match it
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .nth(4)
            .unwrap()
            .chars()
            .next()
            .unwrap()
        {
            '1' => true,
            '0' => false,
            _ => unreachable!(),
        };
        let move_if_one = match it.next().unwrap().split_ascii_whitespace().nth(6).unwrap() {
            "right." => 1,
            "left." => -1,
            _ => unreachable!(),
        };
        let if_one = it
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .nth(4)
            .unwrap()
            .chars()
            .next()
            .unwrap();
        let s = State {
            write_if_zero,
            write_if_one,
            move_if_zero,
            move_if_one,
            if_zero,
            if_one,
        };
        map.insert(c, s);
    }

    (checksum, map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(3_099));
    }
}
