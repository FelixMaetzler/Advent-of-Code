use std::{collections::HashMap, hash::Hash, str::FromStr};

all_aoc::solution!(7, 2015);
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Type {
    Number(u16),
    Wire(String),
}
impl FromStr for Type {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse()
            .map_or_else(|_| Ok(Self::Wire(s.to_owned())), |x| Ok(Self::Number(x)))
    }
}
#[derive(Clone)]
enum Instruction {
    Assignment(Type, String),
    And(Type, Type, String),
    Lshift(Type, Type, String),
    RShift(Type, Type, String),
    Or(Type, Type, String),
    Not(Type, String),
}
impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec: Vec<_> = s.split(' ').collect();
        match vec.len() {
            3 => Ok(Self::Assignment(
                vec[0].parse().unwrap(),
                vec[2].parse().unwrap(),
            )),
            4 => Ok(Self::Not(vec[1].parse().unwrap(), vec[3].parse().unwrap())),
            5 => match vec[1] {
                "AND" => Ok(Self::And(
                    vec[0].parse().unwrap(),
                    vec[2].parse().unwrap(),
                    vec[4].parse().unwrap(),
                )),
                "OR" => Ok(Self::Or(
                    vec[0].parse().unwrap(),
                    vec[2].parse().unwrap(),
                    vec[4].parse().unwrap(),
                )),
                "LSHIFT" => Ok(Self::Lshift(
                    vec[0].parse().unwrap(),
                    vec[2].parse().unwrap(),
                    vec[4].parse().unwrap(),
                )),
                "RSHIFT" => Ok(Self::RShift(
                    vec[0].parse().unwrap(),
                    vec[2].parse().unwrap(),
                    vec[4].parse().unwrap(),
                )),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}
pub fn part_one(input: &str) -> Option<u16> {
    let instructions = parse(input);
    let mut erg = HashMap::new();
    solve(&instructions, &mut erg, "a");
    erg.get("a").copied()
}
pub fn part_two(input: &str) -> Option<u16> {
    let instructions = parse(input);
    let mut erg = HashMap::new();
    solve(&instructions, &mut erg, "a");
    let a = erg.get("a").copied().unwrap();
    erg.clear();
    erg.insert("b".to_owned(), a);
    solve(&instructions, &mut erg, "a");
    erg.get("a").copied()
}
fn solve(
    instructions: &HashMap<String, Instruction>,
    erg: &mut HashMap<String, u16>,
    wire: &str,
) -> u16 {
    if let Some(x) = erg.get(wire) {
        return *x;
    }
    match instructions.get(wire).unwrap() {
        Instruction::Assignment(input, output) => {
            let input = resolve(instructions, erg, input.clone());
            erg.insert(output.clone(), input);
            input
        }
        Instruction::And(input_1, input_2, output) => {
            let input_1 = resolve(instructions, erg, input_1.clone());
            let input_2 = resolve(instructions, erg, input_2.clone());
            let r = input_1 & input_2;
            erg.insert(output.clone(), r);
            r
        }
        Instruction::Lshift(input_1, input_2, output) => {
            let input_1 = resolve(instructions, erg, input_1.clone());
            let input_2 = resolve(instructions, erg, input_2.clone());
            let r = input_1 << input_2;
            erg.insert(output.clone(), r);
            r
        }
        Instruction::RShift(input_1, input_2, output) => {
            let input_1 = resolve(instructions, erg, input_1.clone());
            let input_2 = resolve(instructions, erg, input_2.clone());
            let r = input_1 >> input_2;
            erg.insert(output.clone(), r);
            r
        }
        Instruction::Or(input_1, input_2, output) => {
            let input_1 = resolve(instructions, erg, input_1.clone());
            let input_2 = resolve(instructions, erg, input_2.clone());
            let r = input_1 | input_2;
            erg.insert(output.clone(), r);
            r
        }
        Instruction::Not(input, output) => {
            let input = !resolve(instructions, erg, input.clone());
            erg.insert(output.clone(), input);
            input
        }
    }
}
fn resolve(
    instructions: &HashMap<String, Instruction>,
    erg: &mut HashMap<String, u16>,
    typ: Type,
) -> u16 {
    match typ {
        Type::Number(x) => x,
        Type::Wire(s) => solve(instructions, erg, &s),
    }
}

fn parse(input: &str) -> HashMap<String, Instruction> {
    input
        .lines()
        .map(|l| match Instruction::from_str(l).unwrap() {
            ref x @ (Instruction::Not(_, ref r)
            | Instruction::Or(_, _, ref r)
            | Instruction::Lshift(_, _, ref r)
            | Instruction::RShift(_, _, ref r)
            | Instruction::And(_, _, ref r)
            | Instruction::Assignment(_, ref r)) => (r.clone(), x.clone()),
        })
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(16_076));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(2_797));
    }
}
