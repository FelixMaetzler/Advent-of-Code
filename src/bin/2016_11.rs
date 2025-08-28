use core::hash::Hash;
use std::collections::HashMap;

use all_aoc::helper::{
    graph::{Special, WithWeights as _},
    permutations::IteratorCombinator as _,
};

all_aoc::solution!(11, 2016);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct State<const T: usize> {
    elevator: u64,
    chips: [u64; T],
    gens: [u64; T],
}

impl State<5> {
    fn part_1() -> Self {
        let mut s = Self {
            elevator: 1,
            chips: [1, 3, 3, 3, 3],
            gens: [1, 2, 2, 2, 2],
        };
        s.normalize();
        s
    }
}
impl State<7> {
    fn part_2() -> Self {
        let mut s = Self {
            elevator: 1,
            chips: [1, 3, 3, 3, 3, 1, 1],
            gens: [1, 2, 2, 2, 2, 1, 1],
        };
        s.normalize();
        s
    }
}
impl<const T: usize> State<T> {
    fn end_stae() -> Self {
        let mut s = Self {
            elevator: 4,
            chips: [4; T],
            gens: [4; T],
        };
        s.normalize();
        s
    }
    fn normalize(&mut self) {
        let save_chips = self.chips;
        let save_gens = self.gens;
        let mut v = save_chips
            .iter()
            .copied()
            .zip(save_gens)
            .collect::<Vec<_>>();
        v.sort_by_key(|k| k.1);
        for (new_index, (nc, ng)) in v.iter().enumerate() {
            self.chips[new_index] = *nc;
            self.gens[new_index] = *ng;
        }
    }
    fn next_state(&self) -> Vec<Self> {
        let mut ret = vec![];
        let it = self
            .chips
            .iter()
            .chain(self.gens.iter())
            .enumerate()
            .filter(|(_, x)| **x == self.elevator)
            .map(|(x, _)| x)
            .combinations_until(2);
        for x in it {
            if !(1..=2).contains(&x.len()) {
                continue;
            }
            if self.elevator != 4 {
                let mut clone = *self;
                clone.elevator += 1;
                for xx in x.clone() {
                    if xx >= T {
                        clone.gens[xx - T] += 1;
                    } else {
                        clone.chips[xx] += 1;
                    }
                }
                clone.normalize();
                if clone.is_possible() && clone.is_valid() {
                    ret.push(clone);
                }
            }
            if self.elevator != 1 {
                let mut clone = *self;
                clone.elevator -= 1;
                for xx in x {
                    if xx >= T {
                        clone.gens[xx - T] -= 1;
                    } else {
                        clone.chips[xx] -= 1;
                    }
                }
                clone.normalize();
                if clone.is_possible() && clone.is_valid() {
                    ret.push(clone);
                }
            }
        }
        ret
    }
    fn is_valid(&self) -> bool {
        for (chip, chip_floor) in self.chips.iter().enumerate() {
            if self
                .gens
                .iter()
                .enumerate()
                .filter(|(genn, _)| *genn != chip)
                .any(|(_, gen_floor)| gen_floor == chip_floor)
                && *chip_floor != self.gens[chip]
            {
                return false;
            }
        }

        true
    }
    fn is_possible(&self) -> bool {
        self.chips
            .iter()
            .chain(self.gens.iter())
            .any(|f| *f == self.elevator)
    }
}
pub fn part_one(_: &str) -> Option<usize> {
    solve(State::part_1())
}

pub fn part_two(_: &str) -> Option<usize> {
    solve(State::part_2())
}
fn solve<const T: usize>(mut start_state: State<T>) -> Option<usize> {
    start_state.normalize();
    let mut map = HashMap::new();
    let mut edges = Vec::new();
    let mut queue = vec![start_state];

    while let Some(curr) = queue.pop() {
        let src_idx = if let Some(&idx) = map.get(&curr) {
            idx
        } else {
            let idx = map.len();
            map.insert(curr, idx);
            idx
        };

        for next in curr.next_state() {
            let dst_idx = if let Some(&idx) = map.get(&next) {
                idx
            } else {
                let idx = map.len();
                map.insert(next, idx);
                queue.push(next);
                idx
            };
            edges.push((src_idx, dst_idx, 1));
        }
    }

    let graph = Special::from_edges(edges.into_iter());
    let start = map[&start_state];
    let end = map[&State::end_stae()];
    let map = graph.dijkstra_distances(start, Some(end));
    Some(map[&end])
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn valid() {
        let mut states = [
            State {
                elevator: 1,
                chips: [1, 1],
                gens: [2, 3],
            },
            State {
                elevator: 2,
                chips: [2, 1],
                gens: [2, 3],
            },
            State {
                elevator: 3,
                chips: [3, 1],
                gens: [3, 3],
            },
            State {
                elevator: 2,
                chips: [2, 1],
                gens: [3, 3],
            },
            State {
                elevator: 1,
                chips: [1, 1],
                gens: [3, 3],
            },
            State {
                elevator: 2,
                chips: [2, 2],
                gens: [3, 3],
            },
            State {
                elevator: 3,
                chips: [3, 3],
                gens: [3, 3],
            },
            State {
                elevator: 4,
                chips: [4, 4],
                gens: [3, 3],
            },
            State {
                elevator: 3,
                chips: [3, 4],
                gens: [3, 3],
            },
            State {
                elevator: 4,
                chips: [3, 4],
                gens: [4, 4],
            },
            State {
                elevator: 3,
                chips: [3, 3],
                gens: [4, 4],
            },
            State {
                elevator: 4,
                chips: [4, 4],
                gens: [4, 4],
            },
        ];
        for n in &mut states {
            n.normalize();
        }
        for state in &states {
            assert!(state.is_valid(), "state {state:?} isnt valid");
        }
        for state in &states {
            assert!(state.is_possible(), "state {state:?} isnt valid");
        }
        states.windows(2).for_each(|w| {
            assert!(
                w[0].next_state().contains(&w[1]),
                "Transition from {:?} to {:?} failed",
                w[0],
                w[1]
            );
        });
        states.windows(2).for_each(|w| {
            assert!(
                w[1].next_state().contains(&w[0]),
                "Transition from {:?} to {:?} failed",
                w[1],
                w[0]
            );
        });
    }

    impl State<2> {
        fn test() -> Self {
            let mut s = Self {
                elevator: 1,
                chips: [1, 1],
                gens: [2, 3],
            };
            s.normalize();
            s
        }
    }
    #[test]
    fn test_part_one() {
        let result = solve(State::test());
        assert!(State::test().is_valid());
        assert!(State::test().is_possible());
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(33));
    }
    #[cfg(feature = "expensive")]
    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(57));
    }
}
