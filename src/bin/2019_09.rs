use all_aoc::helper::intcode::{InputMode, IntInteger, Intcode};

all_aoc::solution!(9, 2019);
fn execute(input: &str, x: isize) -> Option<isize> {
    let vec = parse(input);
    let mut m = Intcode::new(vec);
    m.set_inputs(core::iter::once(x), InputMode::Replace);
    m.execute();
    let o = m.get_outputs();
    debug_assert_eq!(o.len(), 1);
    Some(*o.first().unwrap())
}
pub fn part_one(input: &str) -> Option<isize> {
    execute(input, 1)
}

pub fn part_two(input: &str) -> Option<isize> {
    execute(input, 2)
}
fn parse(input: &str) -> Vec<IntInteger> {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(4_288_078_517));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(69_256));
    }
}
