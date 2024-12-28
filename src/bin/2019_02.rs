use all_aoc::helper::intcode::{IntInteger, Intcode};

all_aoc::solution!(2, 2019);
fn execute(noun: IntInteger, verb: IntInteger, program: &[IntInteger]) -> IntInteger {
    let mut m = Intcode::new(program.to_vec());
    m[1] = noun;
    m[2] = verb;
    m.execute();
    m[0]
}
pub fn part_one(input: &str) -> Option<IntInteger> {
    let vec = parse(input);
    Some(execute(12, 2, &vec))
}

pub fn part_two(input: &str) -> Option<IntInteger> {
    let vec = parse(input);
    let erg = (0..100 * 100)
        .find(|i| execute(i / 100, i % 100, &vec) == 19_690_720)
        .unwrap();
    Some(100 * (erg / 100) + (erg % 100))
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
        assert_eq!(result, Some(6_627_023));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(4_019));
    }
}
