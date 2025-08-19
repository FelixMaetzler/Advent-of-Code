use std::{
    collections::HashSet,
    iter,
    ops::{Add, AddAssign},
};

use all_aoc::helper::permutations::IteratorCombinator as _;

enum Win {
    BossWin(u32),
    PlayerWin(u32),
}
all_aoc::solution!(21, 2015);
#[rustfmt::skip]
const WEAPONS: [Stats; 5] = [
    Stats { cost: 8, damage: 4, armor: 0 },
    Stats { cost: 10, damage: 5, armor: 0 },
    Stats { cost: 25, damage: 6, armor: 0 },
    Stats { cost: 40, damage: 7, armor: 0 },
    Stats { cost: 74, damage: 8, armor: 0 },
];
#[rustfmt::skip]
const ARMORS: [Stats; 5] = [
    Stats { cost: 13, armor: 1, damage: 0 },
    Stats { cost: 31, armor: 2, damage: 0 },
    Stats { cost: 53, armor: 3, damage: 0 },
    Stats { cost: 75, armor: 4, damage: 0 },
    Stats { cost: 102, armor: 5, damage: 0 },
];
#[rustfmt::skip]
const RINGS: [Stats; 6] = [
    Stats { cost: 25, damage: 1, armor: 0 },
    Stats { cost: 50, damage: 2, armor: 0 },
    Stats { cost: 100, damage: 3, armor: 0 },
    Stats { cost: 20, damage: 0, armor: 1 },
    Stats { cost: 40, damage: 0, armor: 2 },
    Stats { cost: 80, damage: 0, armor: 3 },
];

#[derive(Debug, Default, Clone, Copy, Hash, Eq, PartialEq)]
struct Stats {
    cost: u32,
    damage: u32,
    armor: u32,
}
impl Add for Stats {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            cost: self.cost + rhs.cost,
            damage: self.damage + rhs.damage,
            armor: self.armor + rhs.armor,
        }
    }
}
impl AddAssign for Stats {
    fn add_assign(&mut self, rhs: Self) {
        self.cost += rhs.cost;
        self.damage += rhs.damage;
        self.armor += rhs.armor;
    }
}
impl iter::Sum for Stats {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |acc, s| acc + s)
    }
}
#[derive(Debug, Default, Clone, Copy, Hash, Eq, PartialEq)]
struct Entitiy {
    stats: Stats,
    hit_points: u32,
}
impl Entitiy {}
fn setup_all_player_combs() -> HashSet<Entitiy> {
    let all_weapon_combs = WEAPONS;
    let all_armor_combs = iter::once(None)
        .chain(ARMORS.into_iter().map(Some))
        .collect::<Vec<_>>();
    let all_ring_combs = RINGS.into_iter().combinations_until(2).collect::<Vec<_>>();
    let len = all_armor_combs.len() * all_weapon_combs.len() * all_ring_combs.len();
    let mut players = HashSet::with_capacity(len);
    for rings in all_ring_combs {
        for weapon in &all_weapon_combs {
            for armor in all_armor_combs.iter().flatten() {
                let stats: Stats = rings.iter().copied().sum::<Stats>() + *weapon + *armor;

                players.insert(Entitiy {
                    stats,
                    hit_points: 100,
                });
            }
        }
    }

    players
}

fn fight(player: Entitiy, boss: Entitiy) -> Win {
    let mut player = player;
    let mut boss = boss;
    while !round(&mut player, &mut boss) {}
    if player.hit_points == 0 {
        Win::BossWin(player.stats.cost)
    } else if boss.hit_points == 0 {
        Win::PlayerWin(player.stats.cost)
    } else {
        unreachable!()
    }
}
fn round(player: &mut Entitiy, boss: &mut Entitiy) -> bool {
    debug_assert_ne!(boss.hit_points, 0);
    debug_assert_ne!(player.hit_points, 0);
    let dmg_to_player = boss.stats.damage.saturating_sub(player.stats.armor).max(1);
    debug_assert!(dmg_to_player >= 1);
    let dmg_to_boss = player.stats.damage.saturating_sub(boss.stats.armor).max(1);
    debug_assert!(dmg_to_boss >= 1);
    boss.hit_points = boss.hit_points.saturating_sub(dmg_to_boss);
    if boss.hit_points == 0 {
        return true;
    }
    player.hit_points = player.hit_points.saturating_sub(dmg_to_player);
    debug_assert_ne!(boss.hit_points, 0);
    player.hit_points == 0
}
pub fn part_one(input: &str) -> Option<u32> {
    let players = setup_all_player_combs();
    let boss = parse_boss(input);
    players
        .into_iter()
        .filter_map(|p| match fight(p, boss) {
            Win::BossWin(_) => None,
            Win::PlayerWin(c) => Some(c),
        })
        .min()
}
pub fn part_two(input: &str) -> Option<u32> {
    let players = setup_all_player_combs();
    let boss = parse_boss(input);
    players
        .into_iter()
        .filter_map(|p| match fight(p, boss) {
            Win::BossWin(c) => Some(c),
            Win::PlayerWin(_) => None,
        })
        .max()
}
fn parse_boss(input: &str) -> Entitiy {
    let vec = input
        .lines()
        .map(|l| l.split_once(": ").unwrap().1)
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();
    Entitiy {
        stats: Stats {
            cost: 0,
            damage: vec[1],
            armor: vec[2],
        },
        hit_points: vec[0],
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(121));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(201));
    }
}
