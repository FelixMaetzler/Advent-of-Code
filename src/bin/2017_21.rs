use std::collections::HashMap;

all_aoc::solution!(21, 2017);
type Grid = Vec<Vec<bool>>;

fn translate_to_grid(s: &str) -> Grid {
    s.split('/')
        .map(|row| row.chars().map(|c| c == '#').collect())
        .collect()
}

fn rotate(grid: &Grid) -> Grid {
    let n = grid.len();
    let mut res = vec![vec![false; n]; n];
    for (i, v) in grid.iter().enumerate() {
        for (j, x) in v.iter().enumerate() {
            res[j][n - 1 - i] = *x;
        }
    }
    res
}

fn flip(grid: &Grid) -> Grid {
    let mut res = grid.clone();
    for row in &mut res {
        row.reverse();
    }
    res
}

fn grid_to_key(grid: &Grid) -> Vec<u8> {
    grid.iter()
        .flat_map(|row| row.iter().map(|&b| u8::from(b)))
        .collect()
}

fn parse_rules(input: &str) -> HashMap<Vec<u8>, Grid> {
    let mut mappings = HashMap::new();
    for line in input.lines() {
        let (k, v) = line.trim().split_once(" => ").unwrap();
        let key_grid = translate_to_grid(k);
        let val_grid = translate_to_grid(v);

        let variants = vec![key_grid.clone(), flip(&key_grid)];
        for g in variants {
            let mut r = g.clone();
            for _ in 0..4 {
                mappings.insert(grid_to_key(&r), val_grid.clone());
                r = rotate(&r);
            }
        }
    }
    mappings
}

fn enhance(grid: &Grid, mappings: &HashMap<Vec<u8>, Grid>) -> Grid {
    let size = grid.len();
    let by = if size.is_multiple_of(2) { 2 } else { 3 };
    let new_size = size * (by + 1) / by;
    let mut solution = vec![vec![false; new_size]; new_size];

    for (i, ni) in (0..size).step_by(by).zip((0..new_size).step_by(by + 1)) {
        for (j, nj) in (0..size).step_by(by).zip((0..new_size).step_by(by + 1)) {
            let mut square = vec![vec![false; by]; by];
            for x in 0..by {
                for y in 0..by {
                    square[x][y] = grid[i + x][j + y];
                }
            }
            let enhanced = mappings.get(&grid_to_key(&square)).unwrap();
            for x in 0..=by {
                for y in 0..=by {
                    solution[ni + x][nj + y] = enhanced[x][y];
                }
            }
        }
    }
    solution
}

fn solve(input: &str, iterations: usize) -> Option<u32> {
    let mappings = parse_rules(input);
    let mut grid = translate_to_grid(".#./..#/###");
    for _ in 0..iterations {
        grid = enhance(&grid, &mappings);
    }
    Some(
        grid.iter()
            .flatten()
            .filter(|&&b| b)
            .count()
            .try_into()
            .unwrap(),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, 5)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, 18)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(162));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(2_264_586));
    }
}
