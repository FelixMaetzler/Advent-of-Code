use core::fmt::Write as _;

use all_aoc::helper::{
    graph::{Graph as _, WithoutWeights as _, build_graph4},
    grid::{Grid as _, dense::DenseGrid},
};
all_aoc::solution!(14, 2017);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        (0..128)
            .map(|n| format!("{input}-{n}"))
            .map(|s| knot_hash(&s))
            .map(u128::count_ones)
            .sum(),
    )
}
fn bits_msb_to_lsb(n: u128) -> impl Iterator<Item = bool> {
    (0..128).rev().map(move |i| (n >> i) & 1 != 0)
}
pub fn part_two(input: &str) -> Option<usize> {
    let grid = DenseGrid::from_iter_iter(
        (0..128)
            .map(|n| format!("{input}-{n}"))
            .map(|s| knot_hash(&s))
            .map(bits_msb_to_lsb),
    );
    let mut graph = build_graph4(&grid, |curr, neigh| *curr && *neigh);
    for (x, _) in grid.iter().enumerate().filter(|(_, b)| **b) {
        graph.add_edge(x, x);
    }
    let components = graph.connected_components();
    Some(components.len())
}
fn knot_hash(input: &str) -> u128 {
    fn execute(
        vec: &Vec<usize>,
        list: &mut [usize],
        curr_pos: &mut usize,
        curr_skip_size: &mut usize,
    ) {
        for length in vec {
            rev(list, *curr_pos, *length);
            *curr_pos += length + *curr_skip_size;
            *curr_pos %= list.len();
            *curr_skip_size += 1;
        }
    }
    fn rev(list: &mut [usize], pos: usize, length: usize) {
        let vgl: Vec<_> = (pos..pos + length)
            .map(|i| list[i % list.len()])
            .rev()
            .collect();
        (0..length).for_each(|i| list[(i + pos) % list.len()] = vgl[i]);
    }

    fn parse(input: &str) -> Vec<usize> {
        let mut v: Vec<_> = input.trim().chars().map(|c| c as usize).collect();
        v.append(&mut vec![17, 31, 73, 47, 23]);
        v
    }
    let vec = parse(input);
    let mut list: Vec<_> = (0..256).collect();
    let mut curr_pos = 0;
    let mut curr_skip_size = 0;
    for _ in 0..64 {
        execute(&vec, &mut list, &mut curr_pos, &mut curr_skip_size);
    }
    let dense_hash: Vec<_> = list
        .chunks(16)
        .map(|c| c.iter().copied().reduce(core::ops::BitXor::bitxor).unwrap())
        .collect();
    let string = dense_hash.iter().fold(String::new(), |mut output, b| {
        let _ = write!(output, "{b:02X}");
        output
    });

    u128::from_str_radix(&string, 16).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(8_108));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(8_226));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(1_242));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_128));
    }
}
