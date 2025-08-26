use core::{ops::RangeInclusive, str::FromStr};
use std::collections::{HashMap, HashSet};

use all_aoc::helper::misc::count_occurrences;

all_aoc::solution!(16, 2020);
type Ticket = Vec<u64>;
type Tickets = Vec<Ticket>;
type Rules = Vec<Rule>;
struct Rule {
    name: String,
    pos: Option<usize>,
    range1: RangeInclusive<u64>,
    range2: RangeInclusive<u64>,
}
impl Rule {
    fn check(&self, x: u64) -> Option<u64> {
        if self.range1.contains(&x) || self.range2.contains(&x) {
            None
        } else {
            Some(x)
        }
    }
}
impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, range) = s.split_once(": ").unwrap();
        let (range1, range2) = range.split_once(" or ").unwrap();
        let convert = |input: &str| -> RangeInclusive<u64> {
            let (x1, x2) = input.split_once('-').unwrap();
            let x1 = x1.parse().unwrap();
            let x2 = x2.parse().unwrap();
            x1..=x2
        };

        Ok(Self {
            name: name.into(),
            pos: None,
            range1: convert(range1),
            range2: convert(range2),
        })
    }
}
fn check_ticket(rules: &Rules, ticket: &Ticket) -> Option<u64> {
    for number in ticket {
        if let Some(x) = check_number(rules, *number) {
            return Some(x);
        }
    }
    None
}
fn check_number(rules: &Rules, x: u64) -> Option<u64> {
    let mut ret = 0;
    for rule in rules {
        if let Some(e) = rule.check(x) {
            ret = e;
        } else {
            return None;
        }
    }
    Some(ret)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (rules, _, tickets) = parse(input);
    Some(
        tickets
            .into_iter()
            .filter_map(|ticket| check_ticket(&rules, &ticket))
            .sum(),
    )
}
fn transpose<T: Clone>(matrix: &[Vec<T>]) -> Vec<Vec<T>> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return vec![];
    }

    let row_count = matrix.len();
    let col_count = matrix[0].len();

    (0..col_count)
        .map(|col| (0..row_count).map(|row| matrix[row][col].clone()).collect())
        .collect()
}
pub fn part_two(input: &str) -> Option<u64> {
    let (mut rules, my_ticket, mut tickets) = parse(input);
    tickets.retain(|ticket| check_ticket(&rules, ticket).is_none());
    tickets.push(my_ticket.clone());
    let trans = transpose(&tickets);
    let mut map = HashMap::new();
    for (i, rule) in rules.iter().enumerate() {
        for (j, row) in trans.iter().enumerate() {
            if row.iter().all(|x| rule.check(*x).is_none()) {
                map.entry(i)
                    .and_modify(|e: &mut HashSet<usize>| {
                        e.insert(j);
                    })
                    .or_insert_with(|| core::iter::once(j).collect());
            }
        }
    }
    let mut changed = true;
    while changed {
        changed = false;
        {
            let mut vec = vec![];
            for (k, v) in &map {
                debug_assert!(!v.is_empty());
                if v.len() == 1 {
                    vec.push(*k);
                    debug_assert!(rules[*k].pos.is_none());
                    rules[*k].pos = Some(*v.iter().next().unwrap());
                    changed = true;
                }
            }
            for v in vec {
                map.remove(&v);
            }
        }
        {
            let c = count_occurrences(map.values().flatten());

            let mut vec = HashSet::new();
            for (k, v) in c {
                if v == 1 {
                    vec.insert(*k);
                }
            }
            let mut rem = vec![];
            for (k, v) in &map {
                let x = vec.intersection(v).collect::<HashSet<_>>();
                if x.is_empty() {
                    continue;
                }
                let x = x.into_iter().next().unwrap();
                rem.push(*k);
                debug_assert!(rules[*k].pos.is_none());
                rules[*k].pos = Some(*x);
                changed = true;
            }
            for v in rem {
                map.remove(&v);
            }
        }
    }
    rules.retain(|rule| rule.name.starts_with("departure"));
    debug_assert_eq!(rules.len(), 6);
    let erg = rules
        .into_iter()
        .map(|rule| my_ticket[rule.pos.unwrap()])
        .product();
    debug_assert!(erg < 7_251_649_195_183);
    Some(erg)
}
fn parse(input: &str) -> (Rules, Ticket, Tickets) {
    let mut it = input.split("\n\n");
    let rules = it.next().unwrap();
    let my_ticket = it.next().unwrap();
    let tickets = it.next().unwrap();
    assert!(it.next().is_none());
    let rules = rules.lines().map(|l| Rule::from_str(l).unwrap()).collect();
    let my_ticket = my_ticket
        .split_once('\n')
        .unwrap()
        .1
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    let tickets = tickets
        .lines()
        .skip(1)
        .map(|l| l.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();
    (rules, my_ticket, tickets)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(71));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(21_996));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(650_080_463_519));
    }
}
