use core::str::FromStr;
use std::collections::HashMap;

use all_aoc::helper::rational::Rational;

all_aoc::solution!(21, 2022);
#[derive(Clone, Copy)]
enum Op {
    Add,
    Substract,
    Multiply,
    Divide,
}
impl FromStr for Op {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Substract),
            "*" => Ok(Self::Multiply),
            "/" => Ok(Self::Divide),
            _ => Err(s.to_owned()),
        }
    }
}
enum Node {
    Number(i64),
    Op(String, Op, String),
}
impl Node {
    fn calc(&self, map: &HashMap<String, Self>) -> i64 {
        match self {
            Self::Number(x) => *x,
            Self::Op(x, op, y) => {
                let x = map[x].calc(map);
                let y = map[y].calc(map);
                match op {
                    Op::Add => x + y,
                    Op::Substract => x - y,
                    Op::Multiply => x * y,
                    Op::Divide => x / y,
                }
            }
        }
    }
}
impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec = s.split_ascii_whitespace().collect::<Vec<_>>();
        if vec.len() == 1 {
            Ok(Self::Number(vec[0].parse().unwrap()))
        } else {
            Ok(Self::Op(
                vec[0].to_owned(),
                vec[1].parse().unwrap(),
                vec[2].to_owned(),
            ))
        }
    }
}
pub fn part_one(input: &str) -> Option<i64> {
    let map = parse(input);
    Some(map["root"].calc(&map))
}
enum Expr {
    Num(i64),
    Var,
    BinOp(Box<Expr>, Op, Box<Expr>),
}
fn expr_from_map(s: &str, map: &HashMap<String, Node>) -> Expr {
    if s == "humn" {
        return Expr::Var;
    }
    match &map[s] {
        Node::Number(x) => Expr::Num(*x),
        Node::Op(x, op, y) => Expr::BinOp(
            expr_from_map(x, map).into(),
            *op,
            expr_from_map(y, map).into(),
        ),
    }
}
pub fn part_two(input: &str) -> Option<i64> {
    let map = parse(input);
    let Node::Op(a, _, b) = &map["root"] else {
        unreachable!()
    };
    let expr = Expr::BinOp(
        expr_from_map(a, &map).into(),
        Op::Substract,
        expr_from_map(b, &map).into(),
    ); // has to be equal to zero
    let erg = eval(expr);
    let x = -erg.0 / erg.1;
    Some(x.get_integer().unwrap())
}
fn eval(expr: Expr) -> (Rational, Rational) {
    match expr {
        Expr::Num(x) => (x.into(), 0.into()),
        Expr::Var => (0.into(), 1.into()),
        Expr::BinOp(x, op, y) => {
            let x = eval(*x);
            let y = eval(*y);
            match op {
                Op::Add => (x.0 + y.0, x.1 + y.1),
                Op::Substract => (x.0 - y.0, x.1 - y.1),
                Op::Multiply => {
                    if x.1 == 0.into() {
                        (x.0 * y.0, x.0 * y.1)
                    } else if y.1 == 0.into() {
                        (y.0 * x.0, y.0 * x.1)
                    } else {
                        unreachable!("non linear term detected")
                    }
                }
                Op::Divide => {
                    if y.1 == 0.into() {
                        (x.0 / y.0, x.1 / y.0)
                    } else {
                        unreachable!("non linear devision")
                    }
                }
            }
        }
    }
}
fn parse(input: &str) -> HashMap<String, Node> {
    input
        .lines()
        .map(|l| l.split_once(": ").unwrap())
        .map(|(x, y)| (x.to_owned(), y.parse().unwrap()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(152));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(62_386_792_426_088));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(301));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(3_876_027_196_185));
    }
}
