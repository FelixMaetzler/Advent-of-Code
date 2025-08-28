use core::{
    fmt::{Debug, Display, Write as _},
    num::ParseIntError,
    str::FromStr,
};

use all_aoc::helper::permutations::IteratorPermutator as _;
use std::collections::{HashMap, VecDeque};
all_aoc::solution!(22, 2016);

#[derive(Clone, Copy, Debug)]
struct Node {
    pos: (usize, usize),
    //size: usize,
    used: usize,
    avail: usize,
    //usee: usize,
}
impl FromStr for Node {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<_> = s.split_ascii_whitespace().collect();
        let first: Vec<_> = v[0].split('-').collect();
        let pos = (first[1][1..].parse()?, first[2][1..].parse()?);
        //let size = v[1][..v[1].len() - 1].parse()?;
        let used = v[2][..v[2].len() - 1].parse()?;
        let avail = v[3][..v[3].len() - 1].parse()?;
        //let usee = v[4][..v[4].len() - 1].parse()?;
        Ok(Self {
            pos,
            //size,
            used,
            avail,
            //usee,
        })
    }
}
impl Display for Node {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let ch = if self.used < 3 {
            '_'
        } else if self.used > 80 {
            '#'
        } else {
            '.'
        };
        f.write_char(ch)
    }
}
pub fn part_one(input: &str) -> Option<usize> {
    Some(
        parse(input)
            .iter()
            .flatten()
            .permutations(2)
            .filter(|v| v[0].used != 0 && v[0].used <= v[1].avail)
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse(input);
    let mut map: HashMap<(usize, usize), Node> = HashMap::new();
    for row in &grid {
        for node in row {
            map.insert(node.pos, *node);
        }
    }

    let lx = grid.len();
    let ly = grid[0].len();

    let start = (0, 0);
    let mut goal = (lx - 1, 0);
    let mut empty = (0, 0);
    for (&pos, node) in &map {
        if node.used == 0 {
            empty = pos;
            break;
        }
    }

    let mut path_gs = find_path(&map, goal, start, None, lx, ly)?;
    let mut cnt = 0;
    while goal != start {
        let path_ = find_path(&map, empty, path_gs.remove(0), Some(goal), lx, ly)?;
        cnt += path_.len() + 1;
        empty = goal;
        goal = *path_.last().unwrap();
    }
    Some(cnt)
}
fn find_path(
    map: &HashMap<(usize, usize), Node>,
    start: (usize, usize),
    end: (usize, usize),
    obst: Option<(usize, usize)>,
    lx: usize,
    ly: usize,
) -> Option<Vec<(usize, usize)>> {
    let mut dist: HashMap<(usize, usize), i32> = HashMap::new();
    let mut prev: HashMap<(usize, usize), Option<(usize, usize)>> = HashMap::new();

    for key in map.keys() {
        dist.insert(*key, i32::MAX);
        prev.insert(*key, None);
    }

    let mut queue = VecDeque::new();
    dist.insert(start, 0);
    queue.push_back(start);

    while let Some(n) = queue.pop_front() {
        for next in [
            (n.0 + 1, n.1),
            (n.0.wrapping_sub(1), n.1),
            (n.0, n.1 + 1),
            (n.0, n.1.wrapping_sub(1)),
        ] {
            if next.0 < lx
                && next.1 < ly
                && let Some(node) = map.get(&next)
                && node.used < 100
                && Some(next) != obst
            {
                let ndist = dist[&n] + 1;
                if dist[&next] > ndist {
                    dist.insert(next, ndist);
                    prev.insert(next, Some(n));
                    queue.push_back(next);
                    if next == end {
                        let mut path = vec![end];
                        let mut cur = end;
                        while let Some(p) = prev[&cur] {
                            path.push(p);
                            cur = p;
                            if cur == start {
                                break;
                            }
                        }
                        path.reverse();
                        return Some(path[1..].to_vec());
                    }
                }
            }
        }
    }
    None
}
fn parse(input: &str) -> Vec<Vec<Node>> {
    let vec = input
        .lines()
        .skip(2)
        .map(|line| Node::from_str(line).unwrap())
        .collect::<Vec<_>>();

    let max_x = vec.iter().map(|n| n.pos.0).max().unwrap();
    let mut map: Vec<Vec<Node>> = vec![];
    for x in 0..=max_x {
        let mut col: Vec<_> = vec.iter().filter(|&&n| n.pos.0 == x).copied().collect();
        col.sort_by(|n1, n2| n1.pos.1.cmp(&n2.pos.1));
        map.push(col);
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(872));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(211));
    }
}
