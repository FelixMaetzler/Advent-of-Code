use all_aoc::helper::{
    intcode::{InputMode, IntInteger, Intcode, Return},
    permutations::IteratorPermutator as _,
};

all_aoc::solution!(7, 2019);

pub fn part_one(input: &str) -> Option<IntInteger> {
    let computer = parse(input);
    Some(
        (0..5)
            .permutation()
            .map(|comb| calc_part_1(&computer, &comb))
            .max()
            .unwrap(),
    )
}
pub fn part_two(input: &str) -> Option<IntInteger> {
    let computer = parse(input);
    Some(
        (5..10)
            .permutation()
            .map(|comb| calc_part_2(&computer, &comb))
            .max()
            .unwrap(),
    )
}
fn calc_part_1(computer: &Intcode, comb: &[isize]) -> IntInteger {
    let mut output = 0;
    for phase_setting in comb {
        let mut c = computer.clone();
        c.set_inputs([*phase_setting, output].into_iter(), InputMode::Extend);
        c.execute();
        let o = c.get_outputs();
        debug_assert_eq!(o.len(), 1);
        output = *o.first().unwrap();
    }
    output
}
fn calc_part_2(computer: &Intcode, comb: &[isize]) -> IntInteger {
    let mut output = 0;
    let mut computers = vec![computer.clone(); 5];
    let mut continu = 0;
    for i in 0..5 {
        let phase_setting = comb[i];
        computers[i].set_inputs(core::iter::once(phase_setting), InputMode::Extend);
    }
    while continu == 0 {
        for c in &mut computers {
            c.set_inputs(core::iter::once(output), InputMode::Extend);
            c.halt_at_output(true);
            match c.execute() {
                Return::Finished => {
                    continu += 1;
                }
                Return::NewOutput => {}
            }
            output = *c.get_outputs().last().unwrap();
        }
        debug_assert!(continu == 0 || continu == 5);
    }
    output
}
fn parse(input: &str) -> Intcode {
    Intcode::new(input.split(',').map(|n| n.parse().unwrap()).collect())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = &all_aoc::cli::read_examples_file(DAY)
            .split("\n\n")
            .take(3)
            .map(part_one)
            .collect::<Vec<_>>();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], Some(43_210));
        assert_eq!(result[1], Some(54_321));
        assert_eq!(result[2], Some(65_210));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(21_760));
    }

    #[test]
    fn test_part_two() {
        let result = &all_aoc::cli::read_examples_file(DAY)
            .split("\n\n")
            .skip(3)
            .map(part_two)
            .collect::<Vec<_>>();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], Some(139_629_729));
        assert_eq!(result[1], Some(18_216));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(69_816_958));
    }
}
