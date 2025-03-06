use all_aoc::helper::intcode::{InputMode, IntInteger, Intcode};
all_aoc::solution!(19, 2019);

pub fn part_one(input: &str) -> Option<u32> {
    let computer = parse(input);
    let it = (0..50).flat_map(|i| (0..50).map(move |j| (i, j)));
    let mut cnt = 0;
    for (i, j) in it {
        if is_attracted(&computer, i, j) {
            cnt += 1;
        }
    }

    Some(cnt)
}

pub fn part_two(input: &str) -> Option<IntInteger> {
    let computer = parse(input);
    let mut x = 0;
    let mut y = 100;
    loop {
        for xx in x.. {
            if is_attracted(&computer, xx, y) {
                x = xx;
                break;
            }
        }

        if square(&computer, x, y) {
            return Some(x * 10_000 + y - 99);
        }
        y += 1;
    }
}

fn square(computer: &Intcode, x: IntInteger, y: IntInteger) -> bool {
    debug_assert_eq!((y - 99..=y).count(), 100);
    for xx in (x..x + 100).rev() {
        for yy in (y - 99..=y).rev() {
            if !is_attracted(computer, xx, yy) {
                return false;
            }
        }
    }
    true
}
fn is_attracted(computer: &Intcode, x: IntInteger, y: IntInteger) -> bool {
    let mut computer = computer.clone();
    computer.set_inputs([x, y].into_iter(), InputMode::Replace);
    computer.execute();
    let output = computer.get_outputs();
    debug_assert_eq!(output.len(), 1);
    output.first() == Some(&1)
}

fn parse(input: &str) -> Intcode {
    Intcode::new(input.split(',').map(|n| n.parse().unwrap()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(229));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(6_950_903));
    }
}
