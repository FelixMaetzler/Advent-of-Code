use all_aoc::helper::{
    grid::{Grid as _, dense::DenseGrid},
    position::Direction4,
};

all_aoc::solution!(2, 2016);

pub fn part_one(input: &str) -> Option<String> {
    let dirs = parse(input);
    let keypad = DenseGrid::from_iter_iter(
        vec![
            vec![Some('1'), Some('2'), Some('3')].into_iter(),
            vec![Some('4'), Some('5'), Some('6')].into_iter(),
            vec![Some('7'), Some('8'), Some('9')].into_iter(),
        ]
        .into_iter(),
    );
    let mut s = String::new();
    let mut i = (1, 1);
    for dir in dirs {
        let (x, y) = get_number(i, &dir, &keypad);
        s.push(x);
        i = y;
    }

    Some(s)
}
fn get_number(
    start: (usize, usize),
    dirs: &[Direction4],
    keypad: &DenseGrid<Option<char>>,
) -> (char, (usize, usize)) {
    let mut pos = start;
    for &dir in dirs {
        if let Some((x, y)) = keypad.get_dir8(pos, dir.into())
            && y.is_some()
        {
            pos = x;
        }
    }
    (keypad.get(pos).unwrap().unwrap(), pos)
}
pub fn part_two(input: &str) -> Option<String> {
    let dirs = parse(input);
    let keypad = DenseGrid::from_iter_iter(
        vec![
            vec![None, None, Some('1'), None, None].into_iter(),
            vec![None, Some('2'), Some('3'), Some('4'), None].into_iter(),
            vec![Some('5'), Some('6'), Some('7'), Some('8'), Some('9')].into_iter(),
            vec![None, Some('A'), Some('B'), Some('C'), None].into_iter(),
            vec![None, None, Some('D'), None, None].into_iter(),
        ]
        .into_iter(),
    );
    let mut s = String::new();
    let mut i = (2, 0);
    for dir in dirs {
        let (x, y) = get_number(i, &dir, &keypad);
        s.push(x);
        i = y;
    }

    Some(s)
}
fn parse(input: &str) -> Vec<Vec<Direction4>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'U' => Direction4::North,
                    'D' => Direction4::South,
                    'L' => Direction4::West,
                    'R' => Direction4::East,
                    x => unreachable!("false char: {x}"),
                })
                .collect()
        })
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some("1985".to_owned()));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some("98575".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some("5DB3".to_owned()));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some("CD8D4".to_owned()));
    }
}
