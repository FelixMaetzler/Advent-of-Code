use std::{
    collections::{HashMap, HashSet},
    iter,
};

use all_aoc::helper::{
    intcode::{InputMode, Intcode, Return},
    misc::Joinable,
    position::{Direction4, Position},
};

all_aoc::solution!(11, 2019);

pub fn part_one(input: &str) -> Option<usize> {
    let computer = parse(input);
    Some(run(computer, 0).1.len())
}

pub fn part_two(input: &str) -> Option<String> {
    let computer = parse(input);
    let map = run(computer, 1).0;
    let x_min = map.keys().map(|p| p.x).min().unwrap();
    let x_max = map.keys().map(|p| p.x).max().unwrap();
    let y_min = map.keys().map(|p| p.y).min().unwrap();
    let y_max = map.keys().map(|p| p.y).max().unwrap();
    Some(
        (y_min..=y_max)
            .rev()
            .map(|y| {
                (x_min..=x_max)
                    .map(|x| match *map.get(&Position { x, y }).unwrap_or(&0) {
                        0 => ' ',
                        1 => '#',
                        _ => unreachable!(),
                    })
                    .join("")
            })
            .join("\n"),
    )
}
fn run(computer: Intcode, start: isize) -> (HashMap<Position<i32>, isize>, HashSet<Position<i32>>) {
    let mut computer = computer;
    let mut pos = Position { x: 0, y: 0 };
    let mut grid = HashMap::new();
    grid.insert(pos, start);
    let mut dir = Direction4::North;
    let mut set = HashSet::new();
    computer.halt_at_output(true);
    loop {
        computer.set_inputs(
            iter::once(*grid.get(&pos).unwrap_or(&0)),
            InputMode::Replace,
        );
        let paint = match computer.execute() {
            Return::Finished => break,
            Return::NewOutput => *computer.get_outputs().last().expect("There has to be one"),
        };
        match computer.execute() {
            Return::Finished => break,
            Return::NewOutput => {
                match *computer.get_outputs().last().expect("There has to be one") {
                    0 => dir = dir.turn_left(),
                    1 => dir = dir.turn_right(),
                    x => unreachable!("false Turn: {x}"),
                }
            }
        }
        grid.insert(pos, paint);
        set.insert(pos);
        pos = pos.direction(dir);
    }
    (grid, set)
}
fn parse(input: &str) -> Intcode {
    Intcode::new(input.split(',').map(|n| n.parse().unwrap()).collect())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(2_392));
    }
}
