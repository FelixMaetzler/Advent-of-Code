use std::collections::HashMap;

all_aoc::solution!(17, 2024);
#[derive(Debug)]
enum Instruction {
    Adv(u64),
    Bxl(u64),
    Bst(u64),
    Jnz(u64),
    Bxc,
    Out(u64),
    Bdv(u64),
    Cdv(u64),
}
#[inline(always)]
fn combo(map: (u64, u64, u64), val: u64) -> u64 {
    match val {
        0..=3 => val,
        4 => map.0,
        5 => map.1,
        6 => map.2,
        x => unreachable!("not a valid combo operand: {x}"),
    }
}
fn machine(a: u64, vec: &[Instruction]) -> Vec<u64> {
    let mut ptr = 0;
    let mut out = vec![];
    let mut a = a;
    let mut b = 0;
    let mut c = 0;
    while ptr < vec.len() {
        match vec[ptr] {
            Instruction::Adv(x) => {
                let y = combo((a, b, c), x);
                a /= 1 << y;
                ptr += 1;
            }
            Instruction::Bxl(x) => {
                b ^= x;
                ptr += 1;
            }
            Instruction::Bst(x) => {
                let y = combo((a, b, c), x);
                b = y % 8;
                ptr += 1;
            }
            Instruction::Jnz(x) => {
                if a == 0 {
                    ptr += 1;
                } else {
                    ptr = x as usize;
                }
            }
            Instruction::Bxc => {
                b ^= c;
                ptr += 1;
            }
            Instruction::Out(x) => {
                let y = combo((a, b, c), x) % 8;
                out.push(y);
                ptr += 1;
            }
            Instruction::Bdv(x) => {
                let y = combo((a, b, c), x);
                b = a / (1 << y);
                ptr += 1;
            }
            Instruction::Cdv(x) => {
                let y = combo((a, b, c), x);
                c = a / (1 << y);
                ptr += 1;
            }
        }
    }
    out
}
pub fn part_one(input: &str) -> Option<String> {
    let ((a, b, c), vec) = parse(input);
    assert_eq!(b, 0);
    assert_eq!(c, 0);
    let erg = machine(a, &vec);
    let erg = erg.into_iter().map(|x| x.to_string()).collect::<Vec<_>>();
    Some(erg.join(","))
}

pub fn part_two(input: &str) -> Option<u64> {
    let vgl = input
        .split(": ")
        .last()
        .unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();
    let ((_, b, c), vec) = parse(input);
    assert_eq!(b, 0);
    assert_eq!(c, 0);
    let mut possible_a_s = vec![1, 2, 3, 4, 5, 6, 7];
    let mut maybe_erg = vec![];
    while !possible_a_s.is_empty() {
        let mut new_as = vec![];
        for a in &possible_a_s {
            for i in 0..8 {
                let new_a = (a << 3) + i;
                let erg = machine(new_a, &vec);
                if erg.len() == vgl.len() {
                    maybe_erg.push(new_a);
                } else if vgl.ends_with(&erg) {
                    new_as.push(new_a);
                }
            }
        }
        possible_a_s = new_as;
    }

    maybe_erg.sort();
    maybe_erg.into_iter().find(|a| machine(*a, &vec) == vgl)
}
fn parse(input: &str) -> ((u64, u64, u64), Vec<Instruction>) {
    let (mem, program) = input.split_once("\n\n").unwrap();
    let map = mem
        .lines()
        .map(|l| {
            let (reg, val) = l.split_once(": ").unwrap();
            (reg.chars().nth(9).unwrap(), val.parse().unwrap())
        })
        .collect::<HashMap<_, _>>();
    let program = program
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let ins = program
        .chunks_exact(2)
        .map(|v| match v[0] {
            0 => Instruction::Adv(v[1]),
            1 => Instruction::Bxl(v[1]),
            2 => Instruction::Bst(v[1]),
            3 => Instruction::Jnz(v[1] / 2),
            4 => Instruction::Bxc,
            5 => Instruction::Out(v[1]),
            6 => Instruction::Bdv(v[1]),
            7 => Instruction::Cdv(v[1]),
            x => unreachable!("not a valid Op Code: {x}"),
        })
        .collect();
    (
        (
            *map.get(&'A').unwrap(),
            *map.get(&'B').unwrap(),
            *map.get(&'C').unwrap(),
        ),
        ins,
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some("5,1,3,4,3,7,2,1,7".to_string()));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(216_584_205_979_245));
    }
}
