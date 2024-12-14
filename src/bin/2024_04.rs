use all_aoc::helper::{
    dense_grid::{DenseGrid, OwnIndex},
    position::Direction8,
};

all_aoc::solution!(4, 2024);

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse(input);
    let dirs = Direction8::all_dirs();
    Some(
        (0..grid.len())
            .map(|index| {
                dirs.iter()
                    .filter(|dir| check_part1(&grid, index, **dir))
                    .count()
            })
            .sum(),
    )
}
pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse(input);
    Some(
        (0..grid.len())
            .filter(|index| check_part2(&grid, *index))
            .count(),
    )
}
fn check_part1(grid: &DenseGrid<char>, idx: impl OwnIndex<char>, dir: Direction8) -> bool {
    let mut index = idx.to_flat_index(grid);
    if grid.get(index).is_none_or(|c| *c != 'X') {
        return false;
    };
    match grid.get_dir8(index, dir) {
        Some((i, 'M')) => index = i,
        _ => return false,
    }
    match grid.get_dir8(index, dir) {
        Some((i, 'A')) => index = i,
        _ => return false,
    }
    matches!(grid.get_dir8(index, dir), Some((_, 'S')))
}
fn check_part2(grid: &DenseGrid<char>, index: impl OwnIndex<char>) -> bool {
    if grid.get(index).is_none_or(|c| *c != 'A') {
        return false;
    };
    match (
        grid.get_dir8(index, Direction8::NorthEast),
        grid.get_dir8(index, Direction8::SouthWest),
    ) {
        (Some((_, 'M')), Some((_, 'S'))) => {}
        (Some((_, 'S')), Some((_, 'M'))) => {}
        _ => return false,
    }
    matches!(
        (
            grid.get_dir8(index, Direction8::SouthEast),
            grid.get_dir8(index, Direction8::NorthWest),
        ),
        (Some((_, 'M')), Some((_, 'S'))) | (Some((_, 'S')), Some((_, 'M')))
    )
}
fn parse(input: &str) -> DenseGrid<char> {
    DenseGrid::from_iter_iter(input.lines().map(|l| l.chars()))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(2_567));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(9));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(2_029));
    }
}
