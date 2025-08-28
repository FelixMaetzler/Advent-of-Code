use core::{num::ParseIntError, str::FromStr};

all_aoc::solution!(15, 2016);
struct Disc {
    pos_count: usize,
    starting_pos: usize,
}
impl Disc {
    const fn is_valid(&self, time: usize) -> bool {
        (self.starting_pos + time).is_multiple_of(self.pos_count)
    }
}
impl FromStr for Disc {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<_> = s.trim_end_matches('.').split(' ').collect();
        Ok(Self {
            pos_count: v[3].parse()?,
            starting_pos: v.last().unwrap().parse()?,
        })
    }
}
pub fn part_one(input: &str) -> Option<usize> {
    let discs = parse(input);
    #[expect(clippy::maybe_infinite_iter, reason = "is terminating")]
    Some((0..).find(|&time| is_valid(time, &discs)).unwrap())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut discs = parse(input);
    discs.push(Disc {
        pos_count: 11,
        starting_pos: 0,
    });
    #[expect(clippy::maybe_infinite_iter, reason = "is terminating")]
    Some((0..).find(|&time| is_valid(time, &discs)).unwrap())
}

fn is_valid(time: usize, discs: &[Disc]) -> bool {
    discs
        .iter()
        .enumerate()
        .all(|(n, disc)| disc.is_valid(time + n + 1))
}
fn parse(input: &str) -> Vec<Disc> {
    input
        .lines()
        .map(|line| Disc::from_str(line).unwrap())
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(400_589));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(3_045_959));
    }
}
