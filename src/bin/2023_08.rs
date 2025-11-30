use all_aoc::helper::misc::lcm;

all_aoc::solution!(8, 2023);

#[derive(Eq, PartialEq, Hash, Clone)]
struct Data<'a> {
    data: &'a str,
    pos: Pos,
}

impl<'a> Data<'a> {
    fn from_str(s: &'a str) -> Self {
        match s.chars().last().unwrap() {
            'A' => Self {
                data: s,
                pos: Pos::Start,
            },
            'Z' => Self {
                data: s,
                pos: Pos::End,
            },
            'B'..='Y' => Self {
                data: s,
                pos: Pos::Middle,
            },
            c => unreachable!("Undefinded char detected: {c}"),
        }
    }
}
#[derive(Clone)]
struct Node<'a> {
    left: Data<'a>,
    right: Data<'a>,
}
impl<'a> Node<'a> {
    fn from_str(s: &'a str) -> Self {
        let (n1, n2) = s.split_once(", ").unwrap();
        let n1 = &n1[1..];
        let n2 = &n2[..n2.len() - 1];
        Self {
            left: Data::from_str(n1),
            right: Data::from_str(n2),
        }
    }
}
/// You can change the return type `OptimizedHashMap` to `HashMap` without any problems
/// The solution takes then longer.
/// This optimization was made to bring the time for part 2 under 1 ms.
fn parse(input: &str) -> (Vec<Dir>, OptimizedHashMap<Data<'_>, Node<'_>>) {
    let (left, right) = input.trim().split_once("\n\n").unwrap();
    let left = left.chars().map(|c| Dir::try_from(c).unwrap()).collect();
    let map = right
        .lines()
        .map(|l| {
            let (k, rem) = l.split_once(" = ").unwrap();
            let k = Data::from_str(k);
            let data = Node::from_str(rem);
            (k, data)
        })
        .collect();
    (left, map)
}
#[derive(Clone)]
enum Dir {
    Left,
    Right,
}
impl TryFrom<char> for Dir {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(()),
        }
    }
}
/// Is a replacement for a regular `HashMap`, that is optimized for exactly this szenario
/// and uses a Vec unter the hood.
struct OptimizedHashMap<K, V> {
    values: Vec<Option<V>>,
    keys: Vec<Option<K>>,
}
impl<'a> OptimizedHashMap<Data<'a>, Node<'a>> {
    fn get(&self, key: &Data<'a>) -> Option<&Node<'a>> {
        self.values[hash(key.data)].as_ref()
    }
    fn keys(&self) -> impl Iterator<Item = &Data<'_>> {
        self.keys.iter().filter_map(|e| e.as_ref())
    }
}
impl<'a> FromIterator<(Data<'a>, Node<'a>)> for OptimizedHashMap<Data<'a>, Node<'a>> {
    fn from_iter<T: IntoIterator<Item = (Data<'a>, Node<'a>)>>(iter: T) -> Self {
        let mut values = vec![None; 26 * 26 * 26];
        let mut keys = vec![None; 26 * 26 * 26];
        for (k, v) in iter {
            let hash = hash(k.data);
            values[hash] = Some(v);
            keys[hash] = Some(k);
        }
        Self { values, keys }
    }
}
fn hash(x: &str) -> usize {
    let x = x.as_bytes();
    ((x[0] - b'A') as usize) + 26 * ((x[1] - b'A') as usize) + 26 * 26 * ((x[2] - b'A') as usize)
}
#[derive(Eq, PartialEq, Hash, Clone)]
enum Pos {
    Start,
    Middle,
    End,
}

pub fn part_one(input: &str) -> Option<u64> {
    let (instructions, map) = parse(input);
    let mut it = instructions.into_iter().cycle();
    let mut curr = &Data::from_str("AAA");
    let mut c = 0;
    while curr != &Data::from_str("ZZZ") {
        let node = map.get(curr).unwrap();
        curr = match it.next().unwrap() {
            Dir::Left => &node.left,
            Dir::Right => &node.right,
        };
        c += 1;
    }
    Some(c)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (instructions, map) = parse(input);
    let mut curr: Vec<&Data<'_>> = map
        .keys()
        .filter(|s| s.pos == Pos::Start)
        .collect::<Vec<_>>();
    let mut curr_lcm = 1;

    for x in &mut curr {
        let mut it = instructions.iter().cycle();
        let mut c = 0;
        while x.pos != Pos::End {
            c += 1;
            let dir = it.next().unwrap();
            let node = map.get(x).unwrap();
            *x = match dir {
                Dir::Left => &node.left,
                Dir::Right => &node.right,
            };
        }
        curr_lcm = lcm(curr_lcm, c.try_into().unwrap());
    }
    Some(curr_lcm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(18_023));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(14_449_445_933_179));
    }
}
