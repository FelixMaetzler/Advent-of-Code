use all_aoc::helper::position::Direction8;

all_aoc::solution!(11, 2017);

pub fn part_one(input: &str) -> Option<u32> {
    let vec = parse(input);
    let mut r: i32 = 0;
    let mut s: i32 = 0;
    let mut q: i32 = 0;
    for dir in vec {
        match dir {
            Direction8::North => {
                s += 1;
                r += -1;
            }
            Direction8::NorthEast => {
                q += 1;
                r += -1;
            }
            Direction8::SouthEast => {
                q += 1;
                s += -1;
            }
            Direction8::South => {
                r += 1;
                s += -1;
            }
            Direction8::SouthWest => {
                r += 1;
                q += -1;
            }
            Direction8::NorthWest => {
                s += 1;
                q += -1;
            }
            Direction8::East | Direction8::West => unreachable!("in a hex, there are only 6 dirs"),
        }
    }
    Some(r.abs().max(s.abs().max(q.abs())).try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let vec = parse(input);
    let mut r: i32 = 0;
    let mut s: i32 = 0;
    let mut q: i32 = 0;
    let mut max = 0;
    for dir in vec {
        match dir {
            Direction8::North => {
                s += 1;
                r += -1;
            }
            Direction8::NorthEast => {
                q += 1;
                r += -1;
            }
            Direction8::SouthEast => {
                q += 1;
                s += -1;
            }
            Direction8::South => {
                r += 1;
                s += -1;
            }
            Direction8::SouthWest => {
                r += 1;
                q += -1;
            }
            Direction8::NorthWest => {
                s += 1;
                q += -1;
            }
            Direction8::East | Direction8::West => unreachable!("in a hex, there are only 6 dirs"),
        }
        let dist = r.abs().max(s.abs().max(q.abs()));
        if dist > max {
            max = dist;
        }
    }
    Some(max.try_into().unwrap())
}
fn parse(input: &str) -> Vec<Direction8> {
    input
        .split(',')
        .map(|s| Direction8::from_short_name(s).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(834));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_569));
    }
}
