use all_aoc::helper::{
    graph::{WithWeights as _, build_graph4_special},
    grid::{Grid as _, dense::DenseGrid, index::GridIndex as _},
    position::{Direction4, Position},
};

all_aoc::solution!(3, 2017);
fn ulam_spiral(edge_length: usize) -> DenseGrid<u64> {
    let mut grid = DenseGrid::new(edge_length, edge_length, 0);
    debug_assert!(!edge_length.is_multiple_of(2));
    let center = (edge_length - 1) / 2;
    let mut pos = Position {
        x: center,
        y: center,
    };
    let dirs = [
        Direction4::East,
        Direction4::North,
        Direction4::West,
        Direction4::South,
    ];
    let mut step_length = 1;
    let mut val = 1;
    grid.set(pos, val);
    while usize::try_from(val).unwrap() < edge_length * edge_length {
        for d in dirs {
            for _ in 0..step_length {
                if usize::try_from(val).unwrap() >= edge_length * edge_length {
                    break;
                }
                pos = pos.dir(d.into(), &grid).unwrap().to_position(&grid);
                val += 1;
                grid.set(pos, val);
            }
            if matches!(d, Direction4::North | Direction4::South) {
                step_length += 1;
            }
        }
    }
    grid
}
pub fn part_one(input: &str) -> Option<u32> {
    #[expect(clippy::cast_possible_truncation, reason = "its fine")]
    #[expect(clippy::cast_sign_loss, reason = "its fine")]
    let n = input.parse::<f64>().unwrap().sqrt().ceil() as usize;
    let grid = ulam_spiral(if n.is_multiple_of(2) { n + 1 } else { n });
    let graph = build_graph4_special(&grid, |_, _| Some(1));
    let start = grid.iter().enumerate().find(|(_, x)| **x == 1).unwrap().0;
    let end = grid
        .iter()
        .enumerate()
        .find(|(_, x)| **x == input.parse().unwrap())
        .unwrap()
        .0;
    let map = graph.dijkstra_distances(start, Some(end));
    Some(map[&end])
}

pub fn part_two(input: &str) -> Option<u32> {
    #[expect(clippy::cast_possible_truncation, reason = "its fine")]
    #[expect(clippy::cast_sign_loss, reason = "its fine")]
    let edge_length = input.parse::<f64>().unwrap().sqrt().ceil() as usize;
    let high_val = input.parse().unwrap();
    let mut grid = DenseGrid::new(edge_length, edge_length, 0);
    debug_assert!(!edge_length.is_multiple_of(2));
    let center = (edge_length - 1) / 2;
    let mut pos = Position {
        x: center,
        y: center,
    };
    let dirs = [
        Direction4::East,
        Direction4::North,
        Direction4::West,
        Direction4::South,
    ];
    let mut step_length = 1;
    let mut val = 1;
    grid.set(pos, val);
    while (val) < high_val {
        for d in dirs {
            for _ in 0..step_length {
                if (val) >= high_val {
                    break;
                }
                pos = pos.dir(d.into(), &grid).unwrap().to_position(&grid);
                val = grid.get_neigbors8(pos).map(|(_, x)| x).sum();
                grid.set(pos, val);
            }
            if matches!(d, Direction4::North | Direction4::South) {
                step_length += 1;
            }
        }
    }

    Some(val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(371));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(369_601));
    }
}
