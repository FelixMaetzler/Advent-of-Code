all_aoc::solution!(18, 2020);
#[derive(Debug)]
enum Op {
    Add,
    Mul,
}
#[derive(Debug)]
enum Expression {
    Binary {
        left: Box<Expression>,
        op: Op,
        right: Box<Expression>,
    },
    Const(u64),
}
impl Expression {
    fn eval(&self) -> u64 {
        match self {
            Self::Binary { left, op, right } => match op {
                Op::Add => left.eval() + right.eval(),
                Op::Mul => left.eval() * right.eval(),
            },
            Self::Const(x) => *x,
        }
    }
}
fn parse_expression_part_1(input: &mut &str) -> Expression {
    let mut expr = parse_factor_part_1(input);

    loop {
        skip_whitespace(input);
        if let Some(op) = parse_operator_part_1(input) {
            let right = parse_factor_part_1(input);
            expr = Expression::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        } else {
            break;
        }
    }

    expr
}
fn parse_operator_part_1(input: &mut &str) -> Option<Op> {
    skip_whitespace(input);
    if input.starts_with('+') {
        *input = &input[1..];
        Some(Op::Add)
    } else if input.starts_with('*') {
        *input = &input[1..];
        Some(Op::Mul)
    } else {
        None
    }
}

fn parse_factor_part_1(input: &mut &str) -> Expression {
    skip_whitespace(input);

    if input.starts_with('(') {
        *input = &input[1..];
        let expr = parse_expression_part_1(input);
        skip_whitespace(input);
        if input.starts_with(')') {
            *input = &input[1..];
        } else {
            panic!("Expected ')'");
        }
        expr
    } else {
        parse_number(input)
    }
}
fn parse_expression_part_2(input: &mut &str) -> Expression {
    let mut expr = parse_term_part_2(input);

    loop {
        skip_whitespace(input);
        if input.starts_with('*') {
            *input = &input[1..];
            let rhs = parse_term_part_2(input);
            expr = Expression::Binary {
                left: Box::new(expr),
                op: Op::Mul,
                right: Box::new(rhs),
            };
        } else {
            break;
        }
    }

    expr
}

fn parse_term_part_2(input: &mut &str) -> Expression {
    let mut expr = parse_factor_part_2(input);

    loop {
        skip_whitespace(input);
        if input.starts_with('+') {
            *input = &input[1..];
            let rhs = parse_factor_part_2(input);
            expr = Expression::Binary {
                left: Box::new(expr),
                op: Op::Add,
                right: Box::new(rhs),
            };
        } else {
            break;
        }
    }

    expr
}
fn parse_factor_part_2(input: &mut &str) -> Expression {
    skip_whitespace(input);

    if input.starts_with('(') {
        *input = &input[1..];
        let expr = parse_expression_part_2(input);
        skip_whitespace(input);
        if input.starts_with(')') {
            *input = &input[1..];
        } else {
            panic!("Expected ')'");
        }
        expr
    } else {
        parse_number(input)
    }
}

fn parse_number(input: &mut &str) -> Expression {
    skip_whitespace(input);
    let mut end = 0;
    while end < input.len() && input.as_bytes()[end].is_ascii_digit() {
        end += 1;
    }
    assert!( end != 0, "Expected number at '{input}'");

    let (number_str, rest) = input.split_at(end);
    *input = rest;
    let value = number_str.parse::<u64>().unwrap();
    Expression::Const(value)
}

fn skip_whitespace(input: &mut &str) {
    *input = input.trim_start();
}
pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|mut l| parse_expression_part_1(&mut l).eval())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|mut l| parse_expression_part_2(&mut l).eval())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("1 + 2 * 3 + 4 * 5 + 6"), Some(71));
        assert_eq!(part_one("1 + (2 * 3) + (4 * (5 + 6))"), Some(51));
        assert_eq!(part_one("2 * 3 + (4 * 5)"), Some(26));
        assert_eq!(part_one("5 + (8 * 3 + 9 + 3 * 4 * 3)"), Some(437));
        assert_eq!(
            part_one("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            Some(12_240)
        );
        assert_eq!(
            part_one("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            Some(13_632)
        );
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(75_592_527_415_659));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("1 + 2 * 3 + 4 * 5 + 6"), Some(231));
        assert_eq!(part_two("1 + (2 * 3) + (4 * (5 + 6))"), Some(51));
        assert_eq!(part_two("2 * 3 + (4 * 5)"), Some(46));
        assert_eq!(part_two("5 + (8 * 3 + 9 + 3 * 4 * 3)"), Some(1_445));
        assert_eq!(
            part_two("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            Some(669_060)
        );
        assert_eq!(
            part_two("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            Some(23_340)
        );
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(360_029_542_265_462));
    }
}
