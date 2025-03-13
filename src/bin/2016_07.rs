use std::collections::HashSet;

all_aoc::solution!(7, 2016);

pub fn part_one(input: &str) -> Option<usize> {
    Some(input.lines().filter(|l| support_tls(l)).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(input.lines().filter(|l| support_ssl(l)).count())
}
fn in_out(input: &str) -> (Vec<String>, Vec<String>) {
    let mut stack = 0;
    let mut out = vec![String::new()];
    let mut inn = vec![];
    for c in input.chars() {
        match c {
            '[' => {
                stack += 1;
                inn.push(String::new());
            }
            ']' => {
                stack -= 1;
                if stack == 0 {
                    out.push(String::new());
                } else {
                    inn.push(String::new());
                }
            }
            x @ 'a'..='z' => {
                if stack == 0 {
                    out.last_mut().unwrap().push(x);
                } else {
                    inn.last_mut().unwrap().push(x);
                }
            }
            x => panic!("wrong char: {x}"),
        }
    }
    (inn, out)
}
fn support_tls(input: &str) -> bool {
    let (inn, out) = in_out(input);
    inn.into_iter().all(|l| !has_abba(&l)) && out.into_iter().any(|l| has_abba(&l))
}
fn has_abba(input: &str) -> bool {
    let v: Vec<_> = input.chars().collect();
    v.windows(4)
        .any(|w| w[0] == w[3] && w[1] == w[2] && w[0] != w[1])
}
fn support_ssl(input: &str) -> bool {
    let (inn, out) = in_out(input);
    let in_seq = inn
        .into_iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .flat_map(|v| {
            v.windows(3)
                .filter_map(|w| {
                    if w[0] == w[2] && w[0] != w[1] {
                        Some((w[0], w[1]))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<HashSet<_>>();
    out.into_iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .any(|v| {
            v.windows(3).any(|w| {
                if w[0] == w[2] && w[0] != w[1] {
                    in_seq.contains(&(w[1], w[0]))
                } else {
                    false
                }
            })
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert!(support_tls("abba[mnop]qrst"));
        assert!(!support_tls("abcd[bddb]xyyx"));
        assert!(!support_tls("aaaa[qwer]tyui"));
        assert!(support_tls("ioxxoj[asdfgh]zxcvbn"));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(110));
    }

    #[test]
    fn test_part_two() {
        assert!(support_ssl("aba[bab]xyz"));
        assert!(!support_ssl("xyx[xyx]xyx"));
        assert!(support_ssl("aaa[kek]eke"));
        assert!(support_ssl("zazbz[bzb]cdb"));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(242));
    }
}
