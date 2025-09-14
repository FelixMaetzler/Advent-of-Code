all_aoc::solution!(13, 2022);
#[derive(PartialEq, Eq, Clone)]
enum Packet {
    Int(u32),
    List(Vec<Self>),
}
impl Packet {
    fn parse(input: &str) -> Result<Self, String> {
        let chars: Vec<char> = input.chars().collect();
        let (packet, pos) = Self::parse_packet(&chars, 0)?;
        if pos != chars.len() {
            return Err(format!("Unexpected characters at end (pos={pos})"));
        }
        Ok(packet)
    }

    fn parse_packet(chars: &[char], mut pos: usize) -> Result<(Self, usize), String> {
        match chars.get(pos) {
            Some('[') => {
                pos += 1;
                let mut items = Vec::new();
                loop {
                    while matches!(chars.get(pos), Some(' ')) {
                        pos += 1;
                    }
                    if matches!(chars.get(pos), Some(']')) {
                        pos += 1;
                        break;
                    }
                    let (item, new_pos) = Self::parse_packet(chars, pos)?;
                    items.push(item);
                    pos = new_pos;
                    while matches!(chars.get(pos), Some(' ')) {
                        pos += 1;
                    }
                    if matches!(chars.get(pos), Some(',')) {
                        pos += 1;
                    }
                }
                Ok((Self::List(items), pos))
            }
            Some(c) if c.is_ascii_digit() => {
                let start = pos;
                while matches!(chars.get(pos), Some(d) if d.is_ascii_digit()) {
                    pos += 1;
                }
                let num: u32 = chars[start..pos]
                    .iter()
                    .collect::<String>()
                    .parse()
                    .map_err(|e: core::num::ParseIntError| e.to_string())?;
                Ok((Self::Int(num), pos))
            }
            Some(c) => Err(format!("Unexpected character '{c}' at pos {pos}")),
            None => Err("Unexpected end of input".to_owned()),
        }
    }
}
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        match (self, other) {
            (Self::Int(x), Self::Int(y)) => x.cmp(y),
            (Self::Int(int), list @ Self::List(_)) => {
                let l = Self::List(vec![Self::Int(*int)]);
                l.cmp(list)
            }
            (list @ Self::List(_), Self::Int(int)) => {
                let l = Self::List(vec![Self::Int(*int)]);
                list.cmp(&l)
            }
            (Self::List(x), Self::List(y)) => {
                for (xx, yy) in x.iter().zip(y) {
                    let cmp = xx.cmp(yy);
                    if cmp != (core::cmp::Ordering::Equal) {
                        return cmp;
                    }
                }
                x.len().cmp(&y.len())
            }
        }
    }
}
pub fn part_one(input: &str) -> Option<usize> {
    let pairs = parse(input);
    Some(
        pairs
            .into_iter()
            .enumerate()
            .filter(|(_, (x, y))| x < y)
            .map(|(i, _)| i + 1)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let first = Packet::parse("[[2]]").unwrap();
    let second = Packet::parse("[[6]]").unwrap();
    let mut vec = parse(input)
        .flat_map(Into::<[Packet; 2]>::into)
        .chain([first.clone(), second.clone()])
        .collect::<Vec<_>>();
    vec.sort_unstable();
    let first = vec
        .iter()
        .enumerate()
        .find(|(_, p)| **p == first)
        .unwrap()
        .0
        + 1;
    let second = vec
        .iter()
        .enumerate()
        .find(|(_, p)| **p == second)
        .unwrap()
        .0
        + 1;
    Some(first * second)
}
fn parse(input: &str) -> impl Iterator<Item = (Packet, Packet)> {
    input
        .split("\n\n")
        .map(|l| l.split_once('\n').unwrap())
        .map(|(a, b)| (Packet::parse(a).unwrap(), Packet::parse(b).unwrap()))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(5_938));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(140));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(29_025));
    }
}
