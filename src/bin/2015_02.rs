use std::str::FromStr;

all_aoc::solution!(2, 2015);
struct Present {
    length: u32,
    width: u32,
    height: u32,
}
impl Present {
    fn wrapping_paper(&self) -> u32 {
        self.slack()
            + 2 * self.length * self.width
            + 2 * self.length * self.height
            + 2 * self.width * self.height
    }
    fn slack(&self) -> u32 {
        (self.length * self.width)
            .min(self.length * self.height)
            .min(self.width * self.height)
    }
    fn bow(&self) -> u32 {
        self.length * self.width * self.height
    }
    fn ribbon(&self) -> u32 {
        self.bow()
            + (2 * (self.length + self.width))
                .min(2 * (self.length + self.height))
                .min(2 * (self.width + self.height))
    }
}
impl FromStr for Present {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split('x');
        let length = it.next().unwrap().parse().unwrap();
        let width = it.next().unwrap().parse().unwrap();
        let height = it.next().unwrap().parse().unwrap();
        assert!(it.next().is_none());
        Ok(Self {
            length,
            width,
            height,
        })
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    Some(parse(input).into_iter().map(|p| p.wrapping_paper()).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(parse(input).into_iter().map(|p| p.ribbon()).sum())
}
fn parse(input: &str) -> Vec<Present> {
    input
        .lines()
        .map(|l| Present::from_str(l).unwrap())
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(Some(58), part_one("2x3x4"));
        assert_eq!(Some(43), part_one("1x1x10"));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_598_415));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Some(34), part_two("2x3x4"));
        assert_eq!(Some(14), part_two("1x1x10"));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(3_812_909));
    }
}
