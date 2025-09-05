all_aoc::solution!(5, 2022);

pub fn part_one(input: &str) -> Option<String> {
    let (mut stacks, ins) = parse(input);
    for (count, from, to) in ins {
        for _ in 0..count {
            let x = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(x);
        }
    }
    Some(stacks.into_iter().map(|v| *v.last().unwrap()).collect())
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut stacks, ins) = parse(input);
    for (count, from, to) in ins {
        let mut s = vec![];
        for _ in 0..count {
            let x = stacks[from - 1].pop().unwrap();
            s.push(x);
        }
        s.reverse();
        stacks[to - 1].append(&mut s);
    }
    Some(stacks.into_iter().map(|v| *v.last().unwrap()).collect())
}
fn parse(input: &str) -> (Vec<Vec<char>>, impl Iterator<Item = (usize, usize, usize)>) {
    let (drawing, instructions) = input.split_once("\n\n").unwrap();
    let mut it = drawing.lines().rev();
    let n = it.next().unwrap().split_ascii_whitespace().count();
    let mut stacks = vec![vec![]; n];
    for line in it {
        let mut chars = line.chars().rev();
        let mut i = n;
        while let Some(c) = chars.next() {
            i -= 1;
            if c == ']' {
                let c = chars.next().unwrap();
                stacks[i].push(c);
            } else {
                chars.next();
            }
            chars.next();
            chars.next();
        }
    }
    let ins = instructions
        .lines()
        .map(|l| l.split_ascii_whitespace().collect::<Vec<_>>())
        .map(|v| {
            (
                v[1].parse().unwrap(),
                v[3].parse().unwrap(),
                v[5].parse().unwrap(),
            )
        });
    (stacks, ins)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some("CMZ".to_owned()));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some("FJSRQCFTN".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some("MCD".to_owned()));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some("CJVLJQPHS".to_owned()));
    }
}
