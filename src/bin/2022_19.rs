use core::str::FromStr;

all_aoc::solution!(19, 2022);
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Blueprint {
    id: u16,
    ore_cost: u16,
    clay_cost: u16,
    obsidian_cost: (u16, u16),
    geode_cost: (u16, u16),
    max_ore: u16,
}
impl FromStr for Blueprint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec = s.split_ascii_whitespace().collect::<Vec<_>>();
        let id = vec[1].trim_end_matches(':').parse().unwrap();
        let ore_cost = vec[6].parse().unwrap();
        let clay_cost = vec[12].parse().unwrap();
        let obsidian_cost = (vec[18].parse().unwrap(), vec[21].parse().unwrap());
        let geode_cost = (vec[27].parse().unwrap(), vec[30].parse().unwrap());
        let max_ore = [ore_cost, clay_cost, obsidian_cost.0, geode_cost.0]
            .into_iter()
            .max()
            .unwrap();
        Ok(Self {
            id,
            ore_cost,
            clay_cost,
            obsidian_cost,
            geode_cost,
            max_ore,
        })
    }
}
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    time_remaining: u16,
    blueprint: Blueprint,
    ressources: [u16; 4],
    robots: [u16; 4],
}
impl State {
    const fn new(blueprint: Blueprint, time_remaining: u16) -> Self {
        Self {
            time_remaining,
            blueprint,
            ressources: [0; 4],
            robots: [1, 0, 0, 0],
        }
    }
    const fn robots_collect(&mut self) {
        self.time_remaining -= 1;
        self.ressources[0] += self.robots[0];
        self.ressources[1] += self.robots[1];
        self.ressources[2] += self.robots[2];
        self.ressources[3] += self.robots[3];
    }

    fn build_ore(mut self) -> Option<Self> {
        (self.ressources[0] >= self.blueprint.ore_cost).then(|| {
            self.ressources[0] -= self.blueprint.ore_cost;
            self.robots[0] += 1;
            self
        })
    }
    fn build_clay(mut self) -> Option<Self> {
        (self.ressources[0] >= self.blueprint.clay_cost).then(|| {
            self.ressources[0] -= self.blueprint.clay_cost;
            self.robots[1] += 1;
            self
        })
    }
    fn build_obsidian(mut self) -> Option<Self> {
        (self.ressources[0] >= self.blueprint.obsidian_cost.0
            && self.ressources[1] >= self.blueprint.obsidian_cost.1)
            .then(|| {
                self.ressources[0] -= self.blueprint.obsidian_cost.0;
                self.ressources[1] -= self.blueprint.obsidian_cost.1;
                self.robots[2] += 1;
                self
            })
    }
    fn build_geode(mut self) -> Option<Self> {
        (self.ressources[0] >= self.blueprint.geode_cost.0
            && self.ressources[2] >= self.blueprint.geode_cost.1)
            .then(|| {
                self.ressources[0] -= self.blueprint.geode_cost.0;
                self.ressources[2] -= self.blueprint.geode_cost.1;
                self.robots[3] += 1;
                self
            })
    }
    const fn calc_geodes_at_the_end(self) -> u16 {
        self.ressources[3] + self.time_remaining * self.robots[3]
    }
    fn next_states(self) -> [Option<Self>; 4] {
        let mut ret = [None; 4];
        if self.robots[2] > 0 {
            let mut copy = self;
            while copy.time_remaining > 1 {
                if let Some(mut x) = copy.build_geode() {
                    x.robots_collect();
                    x.ressources[3] -= 1;
                    ret[0] = Some(x);
                    break;
                }
                copy.robots_collect();
            }
        }
        if self.blueprint.max_ore >= self.robots[0] {
            let mut copy = self;
            while copy.time_remaining > 2 {
                if let Some(mut x) = copy.build_ore() {
                    x.robots_collect();
                    x.ressources[0] -= 1;
                    ret[1] = Some(x);
                    break;
                }
                copy.robots_collect();
            }
        }
        if self.blueprint.obsidian_cost.1 >= self.robots[1] {
            let mut copy = self;
            while copy.time_remaining > 2 {
                if let Some(mut x) = copy.build_clay() {
                    x.robots_collect();
                    x.ressources[1] -= 1;
                    ret[2] = Some(x);
                    break;
                }
                copy.robots_collect();
            }
        }
        if self.blueprint.geode_cost.1 >= self.robots[2] && self.robots[1] > 0 {
            let mut copy = self;
            while copy.time_remaining > 2 {
                if let Some(mut x) = copy.build_obsidian() {
                    x.robots_collect();
                    x.ressources[2] -= 1;
                    ret[3] = Some(x);
                    break;
                }
                copy.robots_collect();
            }
        }

        ret
    }
}
fn max_geodes(state: State) -> u16 {
    let mut queue = Vec::with_capacity(1_000_000);
    queue.push(state);
    let mut max = 0;
    while let Some(v) = queue.pop() {
        if (state.ressources[3]
            + state.robots[3] * state.time_remaining
            + (state.time_remaining * (state.time_remaining + 1)) / 2)
            <= max
        {
            continue;
        }
        max = max.max(v.calc_geodes_at_the_end());
        for next in v.next_states().into_iter().flatten() {
            queue.push(next);
        }
    }
    max
}
pub fn part_one(input: &str) -> Option<u16> {
    Some(
        parse(input)
            .map(|b| (b, State::new(b, 24)))
            .map(|(b, v)| (b.id) * max_geodes(v))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u16> {
    Some(
        parse(input)
            .take(3)
            .map(|b| State::new(b, 32))
            .map(max_geodes)
            .product(),
    )
}
fn parse(input: &str) -> impl Iterator<Item = Blueprint> {
    input.lines().map(|l| l.parse().unwrap())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let blueprint = Blueprint {
            id: 1,
            ore_cost: 4,
            clay_cost: 2,
            obsidian_cost: (3, 14),
            geode_cost: (2, 7),
            max_ore: 4,
        };
        let s = State {
            time_remaining: 4,
            blueprint,
            ressources: [4, 25, 7, 2],
            robots: [1, 4, 2, 1],
        };
        let ss = State {
            time_remaining: 3,
            blueprint,
            ressources: [3, 29, 2, 3],
            robots: [1, 4, 2, 2],
        };
        let sss = State {
            time_remaining: 2,
            blueprint,
            ressources: [4, 33, 4, 5],
            robots: [1, 4, 2, 2],
        };
        assert_eq!(max_geodes(sss), 9);
        assert_eq!(max_geodes(ss), 9);
        assert_eq!(max_geodes(s), 9);
    }
    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(33));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(790));
    }
    #[cfg(feature = "expensive")]
    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(56 * 62));
    }
    #[cfg(feature = "expensive")]
    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(7_350));
    }
}
