use std::{collections::HashMap, fmt::Display};

use all_aoc::helper::graph::{Graph, Special, WithWeights};

all_aoc::solution!(24, 2024);
#[derive(PartialEq)]
enum Instruction {
    Or,
    And,
    Xor,
}
impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Or => "OR",
            Self::And => "AND",
            Self::Xor => "XOR",
        })
    }
}
pub fn part_one(input: &str) -> Option<u64> {
    let (mut start, gates) = parse(input);
    let mut g = Special::new();
    let mut map = HashMap::new();
    for (&z, (x, _, y)) in &gates {
        let len = map.len();
        map.entry(z).or_insert(len);
        let len = map.len();
        map.entry(x).or_insert(len);
        let len = map.len();
        map.entry(y).or_insert(len);
        g.add_edge(map[x], map[z], 1);
        g.add_edge(map[y], map[z], 1);
    }
    let rev = map.iter().map(|(k, v)| (v, k)).collect::<HashMap<_, _>>();
    let order = g.topologocal_order();

    for w in order {
        let z = rev[&w];
        if start.contains_key(z) {
            continue;
        }
        let (x, ins, y) = &gates[z];
        let x = start[x];
        let y = start[y];
        let erg = match ins {
            Instruction::Or => x | y,
            Instruction::And => x & y,
            Instruction::Xor => x != y,
        };
        start.insert(z, erg);
    }
    let mut zs = start
        .into_iter()
        .filter(|(k, _)| k.starts_with('z'))
        .collect::<Vec<_>>();
    zs.sort_by_key(|s| s.0);
    let mut erg = 0;
    while let Some((_, x)) = zs.pop() {
        erg <<= 1;
        erg += u64::from(x);
    }
    Some(erg)
}

pub fn part_two(input: &str) -> Option<String> {
    let (_, gates) = parse(input);
    let mut g = Special::new();
    let mut map = HashMap::new();
    for (&z, (x, _, y)) in &gates {
        let len = map.len();
        map.entry(z).or_insert(len);
        let len = map.len();
        map.entry(x).or_insert(len);
        let len = map.len();
        map.entry(y).or_insert(len);
        g.add_edge(map[x], map[z], 1);
        g.add_edge(map[y], map[z], 1);
    }
    let highest = map
        .keys()
        .filter(|s| s.starts_with('z'))
        .map(|s| s.trim_start_matches('z'))
        .max()
        .unwrap();
    let mut bad = vec![];
    for (output, (in0, op, in1)) in &gates {
        let (in0, in1) = if in0 < in1 { (in0, in1) } else { (in1, in0) };
        if output.starts_with('z') && !output.ends_with(highest) {
            if *op != Instruction::Xor {
                bad.push((*output).to_owned());
            }
        } else if !(in0.starts_with('x') || in1.starts_with('y')) {
            if *op == Instruction::Xor {
                bad.push((*output).to_owned());
            }
        } else if (in0.starts_with('x')) && in1.starts_with('y')
            || (in0.starts_with('y')) && in1.starts_with('x')
        {
            if in0.ends_with("00") || in1.ends_with("00") {
                continue;
            }
            let mut ops = vec![];
            for (ins_l2, opb, ins_l3) in gates.values() {
                if *ins_l2 == *output || *ins_l3 == *output {
                    ops.push(opb);
                }
            }
            if *op == Instruction::Xor && !ops.contains(&&Instruction::Xor)
                || *op == Instruction::And && !ops.contains(&&Instruction::Or)
            {
                bad.push((*output).to_owned());
            }
        }
    }
    bad.sort();
    Some(bad.join(","))
}
type Gates<'a> = HashMap<&'a str, (&'a str, Instruction, &'a str)>;
fn parse(input: &'_ str) -> (HashMap<&'_ str, bool>, Gates<'_>) {
    let (first, second) = input.split_once("\n\n").unwrap();
    let first = first
        .lines()
        .map(|l| l.split_once(": ").unwrap())
        .map(|(l, r)| {
            (
                l,
                match r {
                    "0" => false,
                    "1" => true,
                    x => unreachable!("parse error: {x}"),
                },
            )
        })
        .collect();
    let second = second
        .lines()
        .map(|l| {
            let v = l.split_ascii_whitespace().collect::<Vec<_>>();
            let ins = match v[1] {
                "AND" => Instruction::And,
                "OR" => Instruction::Or,
                "XOR" => Instruction::Xor,
                x => unreachable!("not an instruction: {x}"),
            };
            (v[4], (v[0], ins, v[2]))
        })
        .collect();
    (first, second)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(2_024));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(42_410_633_905_894));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some("cqm,mps,vcv,vjv,vwp,z13,z19,z25".to_owned()));
    }
}
