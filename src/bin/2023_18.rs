use all_aoc::helper::position::{Direction4 as Dir, Position};
all_aoc::solution!(18, 2023);
fn str_to_dir(input: &str) -> Dir {
    match input {
        "U" => Dir::North,
        "D" => Dir::South,
        "R" => Dir::East,
        "L" => Dir::West,
        _ => unreachable!(),
    }
}
struct Instruction {
    dir: Dir,
    length: u64,
}
impl Instruction {
    fn part_1(s: &str) -> Self {
        let mut it = s.split_ascii_whitespace();
        let dir = str_to_dir(it.next().unwrap());
        let length = it.next().unwrap().parse().unwrap();
        Self { dir, length }
    }
    fn part_2(s: &str) -> Self {
        let mut it = s.split_ascii_whitespace();

        let (length, dir) = parse_hex(it.nth(2).unwrap());
        Self { dir, length }
    }
}
fn parse_hex(input: &str) -> (u64, Dir) {
    let rem = input.trim_end_matches(')');
    let rem = rem.trim_start_matches("(#");
    debug_assert_eq!(rem.len(), 6);
    let length = u64::from_str_radix(&rem[0..5], 16).unwrap();
    let dir = match rem.chars().last().unwrap() {
        '0' => Dir::East,
        '1' => Dir::South,
        '2' => Dir::West,
        '3' => Dir::North,
        _ => unreachable!(),
    };
    (length, dir)
}
pub fn part_one(input: &str) -> Option<u64> {
    let vec = &parse_part_1(input);
    solve(vec)
}

pub fn part_two(input: &str) -> Option<u64> {
    let vec = &parse_part_2(input);
    solve(vec)
}
fn solve(vec: &[Instruction]) -> Option<u64> {
    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    let shoelance = shoelance(vec);
    let perimeter = vec.iter().map(|i| i.length).sum::<u64>();
    assert_eq!(perimeter % 2, 0);

    Some(shoelance + perimeter / 2 + 1)
}
fn shoelance(vec: &[Instruction]) -> u64 {
    // https://en.wikipedia.org/wiki/Shoelace_formula
    let mut curr = Position { x: 0, y: 0 };
    let mut points = Vec::with_capacity(vec.len());
    points.push(curr);
    for ins in vec {
        curr += i64::try_from(ins.length).unwrap() * Position { x: 0, y: 0 }.direction(ins.dir);
        points.push(curr);
    }
    let mut points = points.into_iter().rev().collect::<Vec<_>>();
    let n = points.len();
    points.push(points[0]);
    let mut sum = 0;
    for i in 0..n {
        sum += (points[i].y + points[i + 1].y) * (points[i].x - points[i + 1].x);
    }
    assert!(sum >= 0);
    assert_eq!(sum % 2, 0);
    (sum / 2).try_into().unwrap()
}
fn parse_part_1(input: &str) -> Vec<Instruction> {
    input.trim().lines().map(Instruction::part_1).collect()
}
fn parse_part_2(input: &str) -> Vec<Instruction> {
    input.trim().lines().map(Instruction::part_2).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(39_194));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(952_408_144_115));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(78_242_031_808_225));
    }
}
