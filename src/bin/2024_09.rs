use std::iter;

all_aoc::solution!(9, 2024);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    Occupied(u64),
    Empty,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileLayout {
    layout: Layout,
    length: u32,
}

pub fn part_one(input: &str) -> Option<u64> {
    let diskmap = parse(input);
    let mut diskmap = expand(&diskmap);
    remove_empty_space_layout(&mut diskmap);
    Some(checksum(&diskmap))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut diskmap = parse(input);
    remove_empty_space_file_layout(&mut diskmap);
    let diskmap = expand(&diskmap);
    Some(checksum(&diskmap))
}
fn expand(diskmap: &[FileLayout]) -> Vec<Layout> {
    diskmap
        .iter()
        .flat_map(|l| iter::repeat_n(l.layout, l.length as usize))
        .collect()
}
fn remove_empty_space_layout(diskmap: &mut Vec<Layout>) {
    let mut pointer = diskmap
        .iter()
        .enumerate()
        .find(|(_, l)| **l == Layout::Empty)
        .unwrap()
        .0;
    while pointer < diskmap.len() {
        if *diskmap.last().unwrap() == Layout::Empty {
            diskmap.pop();
            continue;
        } else {
            let j = diskmap.len() - 1;
            diskmap.swap(pointer, j);
        }
        pointer = diskmap[pointer..]
            .iter()
            .enumerate()
            .find(|(_, l)| **l == Layout::Empty)
            .unwrap()
            .0
            + pointer;
    }
}

fn calc_length(diskmap: &[FileLayout]) -> u32 {
    diskmap.iter().map(|l| l.length).sum()
}
fn remove_empty_space_file_layout(diskmap: &mut Vec<FileLayout>) {
    let mut pointer = diskmap
        .iter()
        .enumerate()
        .rev()
        .find(|(_, l)| l.layout != Layout::Empty)
        .unwrap()
        .0;
    let constant_length = calc_length(diskmap);
    while pointer > 0 {
        let min_len = diskmap[pointer].length;
        debug_assert_ne!(diskmap[pointer].layout, Layout::Empty);
        let i = match diskmap
            .iter()
            .enumerate()
            .filter(|(i, l)| l.layout == Layout::Empty && i < &pointer)
            .find(|(_, l)| l.length >= min_len)
        {
            Some(x) => x.0,
            None => {
                pointer = diskmap[..pointer]
                    .iter()
                    .enumerate()
                    .rev()
                    .find(|(_, l)| l.layout != Layout::Empty)
                    .unwrap()
                    .0;
                continue;
            }
        };
        let len = diskmap[i].length;
        diskmap.swap(pointer, i);
        if min_len != len {
            diskmap[pointer].length = min_len;
            diskmap.insert(
                i + 1,
                FileLayout {
                    layout: Layout::Empty,
                    length: len - min_len,
                },
            );
        }
        condense(diskmap);
        pointer = diskmap[..pointer]
            .iter()
            .enumerate()
            .rev()
            .find(|(_, l)| l.layout != Layout::Empty)
            .unwrap()
            .0;
        debug_assert_eq!(constant_length, calc_length(diskmap))
    }
}
fn condense(diskmap: &mut Vec<FileLayout>) {
    let before: u32 = diskmap.iter().map(|l| l.length).sum();
    let mut changed = true;
    while changed {
        changed = false;
        if let Some((x, _)) = diskmap
            .windows(2)
            .enumerate()
            .find(|(_, l)| l[0].layout == Layout::Empty && l[1].layout == Layout::Empty)
        {
            let len: u32 = diskmap[x].length + diskmap[x + 1].length;
            diskmap[x].length = len;
            diskmap.remove(x + 1);
            changed = true;
        }
    }
    let after: u32 = diskmap.iter().map(|l| l.length).sum();
    debug_assert_eq!(before, after);
    debug_assert!(diskmap.windows(2).all(|l| l[0].layout != l[1].layout))
}
fn checksum(diskmap: &[Layout]) -> u64 {
    diskmap
        .iter()
        .enumerate()
        .map(|(i, l)| {
            i as u64
                * match l {
                    Layout::Occupied(x) => x,
                    Layout::Empty => &0,
                }
        })
        .sum()
}
fn parse(input: &str) -> Vec<FileLayout> {
    input
        .chars()
        .enumerate()
        .map(|(i, d)| (i, d.to_digit(10).unwrap()))
        .map(|(i, n)| {
            if i % 2 == 0 {
                FileLayout {
                    layout: Layout::Occupied(i as u64 / 2),
                    length: n,
                }
            } else {
                FileLayout {
                    layout: Layout::Empty,
                    length: n,
                }
            }
        })
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(1_928));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(6_299_243_228_569));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(2_858));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(6_326_952_672_104));
    }
}
