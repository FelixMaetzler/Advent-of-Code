use std::collections::HashSet;

use all_aoc::helper::position::Position;

all_aoc::solution!(15, 2022);
#[derive(Debug, Clone, Copy)]
struct Sensor {
    x: i64,
    y: i64,
    range: i64,
}

pub fn part_one(input: &str) -> Option<usize> {
    solve_part_1(input, 2_000_000)
}
fn solve_part_1(input: &str, row: i64) -> Option<usize> {
    let sensor_list = parse(input)
        .map(|(s, b)| {
            let radius = (s.x - b.x).abs() + (s.y - b.y).abs();
            Sensor {
                x: s.x,
                y: s.y,
                range: radius,
            }
        })
        .collect::<Vec<_>>();
    let beacon_list = parse(input).map(|(_, b)| b).collect::<Vec<_>>();

    let mut ranges = vec![];
    for sensor in &sensor_list {
        let distance = (sensor.y - row).abs();
        if distance > sensor.range {
            continue;
        }
        let left = sensor.x - (sensor.range - distance);
        let right = sensor.x + (sensor.range - distance);
        ranges.push(left..=right);
    }
    let mut set = HashSet::new();

    for range in ranges {
        for i in range {
            set.insert(i);
        }
    }
    let pos_sensor = sensor_list
        .iter()
        .filter(|&&sensor| sensor.y == row)
        .map(|&sensor| sensor.x);
    set.extend(pos_sensor);

    let pos_beacon = beacon_list
        .iter()
        .filter(|&&beacon| beacon.y == row)
        .map(|&beacon| beacon.x)
        .collect::<HashSet<_>>();
    set.retain(|i| !pos_beacon.contains(i));
    Some(set.len())
}

pub fn part_two(input: &str) -> Option<i64> {
    solve_part_2(input, 4_000_000)
}
fn solve_part_2(input: &str, row_max: i64) -> Option<i64> {
    let sensor_list = parse(input)
        .map(|(s, b)| {
            let radius = (s.x - b.x).abs() + (s.y - b.y).abs();
            Sensor {
                x: s.x,
                y: s.y,
                range: radius,
            }
        })
        .collect::<Vec<_>>();
    for n in 0..row_max {
        let mut covered_row = vec![0..=row_max];
        for sensor in &sensor_list {
            let distance = (sensor.y - n).abs();
            if distance > sensor.range {
                continue;
            }
            let start = sensor.x - (sensor.range - distance);
            let end = sensor.x + (sensor.range - distance);
            let mut new_covered_row = vec![];
            for range in covered_row {
                if start <= *range.start() && end >= *range.end() {
                } else if end < *range.start() || start > *range.end() {
                    new_covered_row.push(range);
                } else if start >= *range.start() && end <= *range.end() {
                    if *range.start() != start {
                        new_covered_row.push(*range.start()..=(start - 1));
                    }
                    if *range.end() != end {
                        new_covered_row.push((end + 1)..=*range.end());
                    }
                } else if end >= *range.start() && start <= *range.start() {
                    new_covered_row.push((end + 1)..=*range.end());
                } else if start <= *range.end() && end >= *range.end() {
                    new_covered_row.push(*range.start()..=(start - 1));
                }
            }
            covered_row = new_covered_row;
        }
        if !covered_row.is_empty() {
            return Some(4_000_000 * covered_row[0].end() + n);
        }
    }
    unreachable!()
}
fn parse(input: &str) -> impl Iterator<Item = (Position<i64>, Position<i64>)> {
    input.lines().map(|l| {
        let v = l.split_ascii_whitespace().collect::<Vec<_>>();
        let s_x = v[2]
            .trim_end_matches(',')
            .trim_start_matches("x=")
            .parse()
            .unwrap();
        let s_y = v[3]
            .trim_end_matches(':')
            .trim_start_matches("y=")
            .parse()
            .unwrap();
        let b_x = v[8]
            .trim_end_matches(',')
            .trim_start_matches("x=")
            .parse()
            .unwrap();
        let b_y = v[9].trim_start_matches("y=").parse().unwrap();
        (Position { x: s_x, y: s_y }, Position { x: b_x, y: b_y })
    })
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_1(&all_aoc::cli::read_examples_file(DAY), 10);
        assert_eq!(result, Some(26));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(4_811_413));
    }

    #[test]
    fn test_part_two() {
        let result = solve_part_2(&all_aoc::cli::read_examples_file(DAY), 20);
        assert_eq!(result, Some(56_000_011));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(13_171_855_019_123));
    }
}
