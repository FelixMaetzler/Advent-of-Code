use all_aoc::helper::grid::{Grid as _, dense::DenseGrid};

all_aoc::solution!(12, 2025);
#[derive(Debug, Clone, Copy)]
enum Tile {
    On,
    Off,
}
impl TryFrom<char> for Tile {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::On),
            '.' => Ok(Self::Off),
            c => Err(c),
        }
    }
}
fn tiles(grid: &DenseGrid<Tile>) -> usize {
    grid.iter().filter(|t| matches!(t, Tile::On)).count()
}
pub fn part_one(input: &str) -> Option<u32> {
    let (shapes, regions) = parse(input);
    let mut cnt = 0;
    for region in regions {
        let (dimensions, s) = region;
        let tiles: usize = s
            .iter()
            .enumerate()
            .map(|(i, n)| n * tiles(&shapes[i]))
            .sum();
        if tiles > dimensions.0 * dimensions.1 {
        } else {
            cnt += 1;
        }
    }
    Some(cnt)
}

pub const fn part_two(_: &str) -> Option<u32> {
    None
}
fn parse(input: &str) -> (Vec<DenseGrid<Tile>>, Vec<((usize, usize), Vec<usize>)>) {
    let mut first = input.split("\n\n").collect::<Vec<_>>();
    let second = first.pop().unwrap();
    let first = first;
    let first = first
        .into_iter()
        .map(|block| block.split_once('\n').unwrap().1)
        .map(DenseGrid::from_string)
        .collect();
    let second = second
        .lines()
        .map(|l| {
            let (first, second) = l.split_once(": ").unwrap();
            let first = first.split_once('x').unwrap();
            let first = (first.0.parse().unwrap(), first.1.parse().unwrap());
            let second = second
                .split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            (first, second)
        })
        .collect();
    (first, second)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(463));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, None);
    }
}
