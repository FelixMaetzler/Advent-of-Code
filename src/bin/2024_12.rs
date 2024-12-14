use std::collections::HashSet;

use all_aoc::helper::{
    grid::{dense_grid::DenseGrid, grid_index::GridIndex, Grid},
    position::Direction8,
};

all_aoc::solution!(12, 2024);
#[derive(Debug)]
struct Region {
    name: char,
    members: HashSet<usize>,
}
impl Region {
    fn area(&self) -> usize {
        self.members.len()
    }
    fn perimeter(&self, grid: &DenseGrid<char>) -> usize {
        self.members
            .iter()
            .map(|i| {
                4 - grid
                    .get_neigbors4(*i)
                    .filter(|(_, c)| **c == self.name)
                    .count()
            })
            .sum()
    }
    fn sides(&self, grid: &DenseGrid<char>) -> usize {
        self.members
            .iter()
            .map(|i| self.is_part_of_edges(grid, *i))
            .sum()
    }
    fn is_part_of_edges(&self, grid: &DenseGrid<char>, index: usize) -> usize {
        use Direction8::*;
        let mut count = 0;
        let outside_edges = [(North, East), (North, West), (South, East), (South, West)];
        for (dir1, dir2) in outside_edges {
            if grid
                .get_dir8(index, dir1)
                .is_none_or(|(i, _)| !self.members.contains(&i.to_flat_index(grid)))
                && grid
                    .get_dir8(index, dir2)
                    .is_none_or(|(i, _)| !self.members.contains(&i.to_flat_index(grid)))
            {
                count += 1;
            }
        }
        let inside_edges = [
            (South, East, SouthEast),
            (South, West, SouthWest),
            (North, East, NorthEast),
            (North, West, NorthWest),
        ];
        for (d1, d2, d3) in inside_edges {
            if grid
                .get_dir8(index, d1)
                .is_some_and(|(i, _)| self.members.contains(&i.to_flat_index(grid)))
                && grid
                    .get_dir8(index, d2)
                    .is_some_and(|(i, _)| self.members.contains(&i.to_flat_index(grid)))
                && grid
                    .get_dir8(index, d3)
                    .is_none_or(|(i, _)| !self.members.contains(&i.to_flat_index(grid)))
            {
                count += 1;
            }
        }

        count
    }
}
pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse(input);
    let regions = collect_regions(&grid);
    Some(
        regions
            .into_iter()
            .map(|r| r.area() * r.perimeter(&grid))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse(input);
    let regions = collect_regions(&grid);
    Some(regions.into_iter().map(|r| r.area() * r.sides(&grid)).sum())
}
fn collect_regions(grid: &DenseGrid<char>) -> Vec<Region> {
    let mut indices = HashSet::with_capacity(grid.len());
    let mut regions = vec![];
    for i in 0..grid.len() {
        if indices.contains(&i) {
            continue;
        }
        let r = collect_region(i, grid);
        indices.extend(r.members.iter().cloned());
        regions.push(r);
    }
    regions
}
fn collect_region(start: usize, grid: &DenseGrid<char>) -> Region {
    let name = *grid.get(start).unwrap();
    let mut stack = vec![start];
    let mut members = HashSet::new();
    while let Some(x) = stack.pop() {
        if !members.insert(x) {
            continue;
        }
        grid.get_neigbors4(x)
            .filter(|(_, c)| **c == name)
            .map(|(i, _)| i.to_flat_index(grid))
            .for_each(|i| {
                stack.push(i);
            });
    }
    Region { name, members }
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
        assert_eq!(result, Some(1_930));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_486_324));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(1_206));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(89_8684));
    }
}
