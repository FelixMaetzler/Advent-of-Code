use all_aoc::helper::grid::{Grid as _, dense::DenseGrid, index::GridIndex as _};

all_aoc::solution!(4, 2025);
#[derive(Debug, Clone, Copy)]
enum Tile {
    Air,
    Paper,
}
impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Air,
            '@' => Self::Paper,
            x => unreachable!("unknown char: {x}"),
        }
    }
}
pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse(input);
    Some(
        (0..grid.len())
            .filter(|i| matches!(grid[*i], Tile::Paper))
            .filter(|&i| {
                grid.get_neigbors8(i)
                    .filter(|(_, x)| matches!(x, Tile::Paper))
                    .count()
                    < 4
            })
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid = parse(input);
    let mut cnt = 0;
    let mut queue = (0..grid.len())
        .filter(|i| matches!(grid[*i], Tile::Paper))
        .collect::<Vec<_>>();
    while let Some(i) = queue.pop() {
        if matches!(grid[i], Tile::Air) {
            continue;
        }
        let mut n = grid
            .get_neigbors8(i)
            .filter(|(_, t)| matches!(t, Tile::Paper))
            .map(|(x, _)| x.to_flat_index(&grid))
            .collect::<Vec<_>>();
        if n.len() < 4 {
            grid[i] = Tile::Air;
            cnt += 1;
            queue.append(&mut n);
        }
    }
    Some(cnt)
}

fn parse(input: &str) -> DenseGrid<Tile> {
    DenseGrid::from_string(input)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_493));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(43));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(9_194));
    }
}
