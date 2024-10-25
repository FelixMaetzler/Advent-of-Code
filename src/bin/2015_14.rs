use std::str::FromStr;

all_aoc::solution!(14, 2015);
struct Reindeer {
    speed: u32,
    fly_time: u32,
    rest_time: u32,
}
impl FromStr for Reindeer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec: Vec<_> = s.split(' ').collect();
        let speed = vec[3].parse().unwrap();
        let fly_time = vec[6].parse().unwrap();
        let rest_time = vec[13].parse().unwrap();
        Ok(Reindeer {
            speed,
            fly_time,
            rest_time,
        })
    }
}
impl Reindeer {
    fn dist(&self, max_time: u32) -> u32 {
        let mut time_left = max_time;
        let mut distance = 0;

        loop {
            let x = self.fly_time.min(time_left);
            distance += x * self.speed;
            time_left -= x;
            if time_left == 0 {
                return distance;
            }
            let x = self.rest_time.min(time_left);
            time_left -= x;
            if time_left == 0 {
                return distance;
            }
        }
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    solve_part_1(input, 2_503)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve_part_2(input, 2_503)
}
fn solve_part_1(input: &str, time: u32) -> Option<u32> {
    parse(input).into_iter().map(|r| r.dist(time)).max()
}
fn solve_part_2(input: &str, time: u32) -> Option<u32> {
    let vec = parse(input);
    let mut points = vec![0; vec.len()];
    for i in 1..time {
        let maxs = vec.iter().map(|r| r.dist(i)).collect::<Vec<_>>();
        let max = maxs.iter().max().unwrap();
        for i in 0..points.len() {
            if maxs[i] == *max {
                points[i] += 1;
            }
        }
    }
    points.into_iter().max()
}
fn parse(input: &str) -> Vec<Reindeer> {
    input
        .lines()
        .map(|l| Reindeer::from_str(l).unwrap())
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_1(&all_aoc::cli::read_examples_file(DAY), 1_000);
        assert_eq!(result, Some(1_120));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(2_655));
    }

    #[test]
    fn test_part_two() {
        let result = solve_part_2(&all_aoc::cli::read_examples_file(DAY), 1_000);
        assert_eq!(result, Some(689));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_059));
    }
}
