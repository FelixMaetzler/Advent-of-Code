use all_aoc::helper::intcode::{InputMode, IntInteger, Intcode};

all_aoc::solution!(5, 2019);

pub fn part_one(input: &str) -> Option<IntInteger> {
    let vec = parse(input);
    let mut m = Intcode::new(vec);
    m.set_inputs([1].into_iter(), InputMode::Replace);
    m.execute();
    let mut o = m.get_outputs();
    let code = o.pop().unwrap();
    debug_assert!(o.into_iter().all(|n| n == 0));
    Some(code)
}

pub fn part_two(input: &str) -> Option<IntInteger> {
    let vec = parse(input);
    let mut m = Intcode::new(vec);
    m.set_inputs([5].into_iter(), InputMode::Replace);
    m.execute();
    let mut o = m.get_outputs();
    let code = o.pop().unwrap();
    debug_assert!(o.is_empty());
    Some(code)
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
        assert_eq!(result, Some(15_386_262));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(10_376_124));
    }
}
