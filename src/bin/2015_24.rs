use all_aoc::helper::permutations::IteratorCombinator as _;

all_aoc::solution!(24, 2015);

fn can_partition(available: u128, groups_left: usize, w: &[u64], target: u64) -> bool {
    if groups_left == 0 {
        return available == 0;
    }
    search_group(available, 0, 0, 0, groups_left, w, target)
}

fn search_group(
    available: u128,
    start_idx: usize,
    sum: u64,
    chosen: u128,
    groups_left: usize,
    w: &[u64],
    target: u64,
) -> bool {
    if sum == target {
        let remaining = available & !chosen;
        return can_partition(remaining, groups_left - 1, w, target);
    }
    let n = w.len();
    let mut i = start_idx;
    while i < n {
        if ((available >> i) & 1) == 1 {
            let wi = w[i];
            if sum + wi <= target
                && search_group(
                    available,
                    i + 1,
                    sum + wi,
                    chosen | (1_u128 << i),
                    groups_left,
                    w,
                    target,
                )
            {
                return true;
            }
        }
        i += 1;
    }
    false
}
fn execute(input: &str, count: usize) -> Option<u64> {
    let mut w = parse(input);
    w.sort_unstable_by(|a, b| b.cmp(a));

    let total: u64 = w.iter().sum();
    if total % (count as u64) != 0 {
        return None;
    }
    let target = total / (count as u64);
    let n = w.len();

    if n > 128 {
        return None;
    }

    let full_mask: u128 = if n == 128 { !0 } else { (1_u128 << n) - 1 };

    for k in 1..=n {
        let mut best_qe: Option<u128> = None;

        for idxs in (0..n).combinations(k) {
            let sum: u64 = idxs.iter().map(|&i| w[i]).sum();
            if sum != target {
                continue;
            }

            let mut first_mask = 0_u128;
            for &i in &idxs {
                first_mask |= 1_u128 << i;
            }
            let rest = full_mask & !first_mask;

            if can_partition(rest, count - 1, &w, target) {
                let qe: u128 = idxs.iter().map(|&i| u128::from(w[i])).product();
                match best_qe {
                    None => best_qe = Some(qe),
                    Some(cur) => {
                        if qe < cur {
                            best_qe = Some(qe);
                        }
                    }
                }
            }
        }

        if let Some(qe) = best_qe {
            return Some(qe.try_into().unwrap());
        }
    }

    None
}
pub fn part_one(input: &str) -> Option<u64> {
    execute(input, 3)
}
pub fn part_two(input: &str) -> Option<u64> {
    execute(input, 4)
}

fn parse(input: &str) -> Vec<u64> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(99));
    }
    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(10_439_961_859));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(44));
    }
    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(72_050_269));
    }
}
