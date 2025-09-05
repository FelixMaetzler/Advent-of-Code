use all_aoc::helper::{
    grid::{Grid as _, dense::DenseGrid, index::GridIndex as _},
    position::Direction4,
};

all_aoc::solution!(8, 2022);

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse(input);
    Some((0..grid.len()).filter(|i| is_visible(&grid, *i)).count())
}
fn is_visible_dir(grid: &DenseGrid<u8>, mut index: usize, dir: Direction4) -> bool {
    let n = grid.get(index).unwrap();
    while let Some((i, x)) = grid.get_dir8(index, dir.into()) {
        index = i.to_flat_index(grid);
        if x >= n {
            return false;
        }
    }
    true
}
fn scenic_score(grid: &DenseGrid<u8>, index: usize) -> u32 {
    Direction4::all_dirs()
        .into_iter()
        .map(|dir| sight_length(grid, index, dir))
        .product()
}

fn sight_length(grid: &DenseGrid<u8>, mut index: usize, dir: Direction4) -> u32 {
    let n = grid.get(index).unwrap();
    let mut cnt = 0;
    while let Some((i, x)) = grid.get_dir8(index, dir.into()) {
        index = i.to_flat_index(grid);
        cnt += 1;
        if x >= n {
            break;
        }
    }
    cnt
}
fn is_visible(grid: &DenseGrid<u8>, index: usize) -> bool {
    Direction4::all_dirs()
        .into_iter()
        .any(|dir| is_visible_dir(grid, index, dir))
}
pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse(input);
    Some(
        (0..grid.len())
            .map(|i| scenic_score(&grid, i))
            .max()
            .unwrap(),
    )
}
fn parse(input: &str) -> DenseGrid<u8> {
    DenseGrid::from_iter_iter(input.lines().map(|l| {
        l.chars()
            .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
    }))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_705));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(371_200));
    }
}
