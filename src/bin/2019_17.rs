use core::fmt::Debug;

use all_aoc::helper::{
    grid::{Grid, dense::DenseGrid, index::GridIndex},
    intcode::{InputMode, IntInteger, Intcode, Return},
    misc::Joinable,
    position::Direction4,
};
#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Space,
    Scaffolding,
    Robot(Direction4),
}
impl TryFrom<char> for Tile {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Space),
            '#' => Ok(Self::Scaffolding),
            '^' => Ok(Self::Robot(Direction4::North)),
            '>' => Ok(Self::Robot(Direction4::East)),
            '<' => Ok(Self::Robot(Direction4::West)),
            'v' => Ok(Self::Robot(Direction4::South)),
            x => Err(x),
        }
    }
}
#[derive(PartialEq, Eq)]
enum Turn {
    Left,
    Right,
}
impl Debug for Turn {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Left => write!(f, "L"),
            Self::Right => write!(f, "R"),
        }
    }
}
#[derive(PartialEq, Eq)]
struct Instruction {
    turn: Turn,
    len: u32,
}
impl Debug for Instruction {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?},", self.turn)?;
        write!(f, "{}", self.len)
    }
}
impl Debug for Tile {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Space => write!(f, "."),
            Self::Scaffolding => write!(f, "#"),
            Self::Robot(Direction4::North) => write!(f, "^"),
            Self::Robot(Direction4::South) => write!(f, "v"),
            Self::Robot(Direction4::East) => write!(f, ">"),
            Self::Robot(Direction4::West) => write!(f, "<"),
        }
    }
}
all_aoc::solution!(17, 2019);
fn get_grid(mut computer: Intcode) -> DenseGrid<Tile> {
    let erg = computer.execute();
    debug_assert_eq!(erg, Return::Finished);
    let output = computer.get_outputs();
    let s = output
        .iter()
        .map(|&i| char::from_u32(i as u32).unwrap())
        .join("");
    DenseGrid::from_string(&s)
}
pub fn part_one(input: &str) -> Option<usize> {
    let computer = parse(input);
    let grid = get_grid(computer);
    Some(
        grid.all_indices()
            .filter(|i| {
                *grid.get(*i).unwrap() != Tile::Space
                    && grid.get_neigbors4(*i).all(|(_, t)| *t != Tile::Space)
            })
            .map(|i| {
                let (n1, n2) = i.to_coordinates(&grid);
                n1 * n2
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<IntInteger> {
    let computer = parse(input);
    let grid = get_grid(computer.clone());
    let x = compress(&grid);
    let mut options = vec![];
    for a in 1..x.len() - 2 {
        for b in 1..x.len() - 2 {
            for c in 1..x.len() - 2 {
                if let Some((ins, s)) = divide(&x, a, b, c) {
                    if is_viable(ins, &s) {
                        options.push((ins, s));
                    }
                }
            }
        }
    }
    debug_assert_eq!(options.len(), 1);
    let (ins, s) = options.pop().unwrap();
    let mut computer = computer;
    computer[0] = 2;
    let mut s = s;
    s.push('\n');
    for i in ins {
        s.push_str(&to_string(i));
        s.push('\n');
    }
    s.push('n');
    s.push('\n');
    computer.set_inputs(s.chars().map(|c| c as isize), InputMode::Replace);
    computer.execute();
    let output = &computer.get_outputs()[1485..];
    Some(*output.last().unwrap())
}
fn compress(grid: &DenseGrid<Tile>) -> Vec<Instruction> {
    let start = grid
        .iter()
        .enumerate()
        .find(|(_, t)| matches!(t, Tile::Robot(_)))
        .unwrap();
    let (curr, dir) = match start.1 {
        Tile::Robot(d) => (start.0.to_coordinates(grid), *d),
        Tile::Space | Tile::Scaffolding => unreachable!(),
    };
    let mut vec = vec![];
    let mut next = curr;
    let mut prev_dir = dir;
    let mut dir = dir;
    let mut e = false;
    for d in Direction4::all_dirs() {
        if grid.get_dir8(next, d.into()).map(|(_, d)| d) == Some(&Tile::Scaffolding) {
            debug_assert!(!e);
            e = true;
            dir = d;
        }
    }
    debug_assert!(e);
    loop {
        let mut len = 0;

        while let Some(x) = grid
            .get_dir8(next, dir.into())
            .and_then(|(i, t)| (*t != Tile::Space).then_some(i))
        {
            next = x;
            len += 1;
        }
        debug_assert_ne!(*grid.get(next).unwrap(), Tile::Space);
        let turn = if prev_dir.turn_left() == dir {
            Turn::Left
        } else if prev_dir.turn_right() == dir {
            Turn::Right
        } else {
            unreachable!()
        };
        prev_dir = dir;
        vec.push(Instruction { turn, len });
        let mut e = false;
        for d in Direction4::all_dirs() {
            if d == prev_dir.opposite() {
                continue;
            }
            if grid.get_dir8(next, d.into()).map(|(_, d)| d) == Some(&Tile::Scaffolding) {
                debug_assert_ne!(d, dir);
                debug_assert!(!e);
                e = true;
                dir = d;
            }
        }
        if !e {
            break;
        }
    }

    vec
}
fn to_string(ins: &[Instruction]) -> String {
    ins.iter().map(|i| format!("{i:?}")).join(",")
}
fn is_viable(ins: [&[Instruction]; 3], s: &str) -> bool {
    if s.len() > 20 {
        return false;
    }
    ins.into_iter().all(|i| to_string(i).len() <= 20)
}

fn divide(
    vec: &[Instruction],
    a_len: usize,
    b_len: usize,
    c_len: usize,
) -> Option<([&[Instruction]; 3], String)> {
    let mut combi = String::new();
    let mut to_match = vec;
    let a = &to_match[0..a_len];
    to_match = &to_match[a.len()..];
    combi.push('A');
    while to_match.starts_with(a) {
        combi.push('A');
        to_match = &to_match[a.len()..];
    }
    if to_match.len() < b_len {
        return None;
    }
    let b = &to_match[0..b_len];
    to_match = &to_match[b.len()..];
    combi.push('B');
    let mut cont = true;
    while cont {
        cont = false;
        if to_match.starts_with(a) {
            combi.push('A');
            to_match = &to_match[a.len()..];
            cont = true;
        } else if to_match.starts_with(b) {
            combi.push('B');
            to_match = &to_match[b.len()..];
            cont = true;
        }
    }
    if to_match.len() < c_len {
        return None;
    }
    let c = &to_match[0..c_len];
    to_match = &to_match[c.len()..];
    combi.push('C');
    let mut cont = true;
    while cont {
        cont = false;
        if to_match.starts_with(a) {
            combi.push('A');
            to_match = &to_match[a.len()..];
            cont = true;
        } else if to_match.starts_with(b) {
            combi.push('B');
            to_match = &to_match[b.len()..];
            cont = true;
        } else if to_match.starts_with(c) {
            combi.push('C');
            to_match = &to_match[c.len()..];
            cont = true;
        }
    }
    to_match
        .is_empty()
        .then(|| ([a, b, c], combi.chars().join(",")))
}
fn parse(input: &str) -> Intcode {
    Intcode::new(input.split(',').map(|x| x.parse().unwrap()).collect())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(4_408));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(862_452));
    }
}
