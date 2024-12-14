all_aoc::solution!(22, 2015);
#[derive(Debug, Clone, Copy)]
enum Difficulty {
    Easy = 0,
    Hard = 1,
}
#[derive(Debug)]
struct GameState {
    player_hp: i32,
    boss_hp: i32,
    mana: i32,
    damage: i32,
    armor: i32,
    spent: i32,
    turn: i32,
    difficulty: Difficulty,
    effects: Vec<Spell>,
}

impl GameState {
    fn new(player_hp: i32, boss_hp: i32, mana: i32, damage: i32) -> Self {
        Self {
            player_hp,
            boss_hp,
            mana,
            damage,
            armor: 0,
            spent: 0,
            turn: 0,
            difficulty: Difficulty::Easy,
            effects: vec![],
        }
    }

    fn start_turn(&self) -> Option<Self> {
        let mut player_hp = self.player_hp;
        let mut boss_hp = self.boss_hp;
        let mut mana = self.mana;
        let mut armor = 0;
        let mut effects = vec![];

        if self.turn % 2 == 0 {
            player_hp -= self.difficulty as i32;
            if player_hp <= 0 {
                return None;
            }
        }

        for effect in &self.effects {
            if effect.armor > 0 {
                armor = effect.armor;
            }

            if effect.damage > 0 {
                boss_hp -= effect.damage;
            }

            if effect.mana > 0 {
                mana += effect.mana;
            }

            if effect.duration > 1 {
                effects.push(effect.tick());
            }
        }

        Some(Self {
            player_hp,
            boss_hp,
            mana,
            damage: self.damage,
            armor,
            spent: self.spent,
            turn: self.turn + 1,
            difficulty: self.difficulty,
            effects,
        })
    }

    fn cast(&self, spell: &Spell) -> Self {
        if self.mana < spell.cost {
            panic!("bug in move generator");
        }

        if spell.duration > 0 {
            let mut effects = self.effects.clone();
            effects.push(spell.clone());
            Self {
                player_hp: self.player_hp,
                boss_hp: self.boss_hp,
                mana: self.mana - spell.cost,
                damage: self.damage,
                armor: self.armor,
                spent: self.spent + spell.cost,
                turn: self.turn,
                difficulty: self.difficulty,
                effects,
            }
        } else {
            Self {
                player_hp: self.player_hp + spell.heal,
                boss_hp: self.boss_hp - spell.damage,
                mana: self.mana + spell.mana - spell.cost,
                damage: self.damage,
                armor: self.armor,
                spent: self.spent + spell.cost,
                turn: self.turn,
                difficulty: self.difficulty,
                effects: self.effects.clone(),
            }
        }
    }

    fn moves(&self) -> Vec<Spell> {
        let mut result = vec![];
        'outer: for spell in SPELLS {
            for effect in &self.effects {
                if effect.name == spell.name {
                    continue 'outer;
                }
            }

            if spell.cost <= self.mana {
                result.push(spell.clone());
            }
        }

        result
    }

    fn boss_move(&self) -> Self {
        let damage = 1.max(self.damage - self.armor);

        Self {
            player_hp: self.player_hp - damage,
            boss_hp: self.boss_hp,
            mana: self.mana,
            damage: self.damage,
            armor: self.armor,
            spent: self.spent,
            turn: self.turn,
            difficulty: self.difficulty,
            effects: self.effects.clone(),
        }
    }

    fn cheapest_win(&self, mut max: i32) -> Option<Self> {
        let mut best_so_far: Option<GameState> = None;

        let turn = self.start_turn()?;

        if turn.player_hp <= 0 {
            return None;
        }

        if turn.boss_hp <= 0 {
            return Some(turn);
        }

        if turn.turn > 50 {
            return None;
        }

        if turn.turn % 2 == 0 {
            let boss_move = turn.boss_move();
            if boss_move.player_hp <= 0 {
                return None;
            }
            return boss_move.cheapest_win(max);
        }

        for spell in &turn.moves() {
            if spell.cost + self.spent >= max {
                continue;
            }

            let move_state = turn.cast(spell);

            if let Some(best) = &best_so_far {
                if best.spent < max {
                    max = best.spent;
                }
            }

            if move_state.boss_hp <= 0 {
                if let Some(best) = &best_so_far {
                    if best.spent > move_state.spent {
                        best_so_far = Some(move_state);
                    }
                } else {
                    best_so_far = Some(move_state);
                }
            } else if move_state.player_hp <= 0 {
                continue;
            } else if let Some(cheapest) = move_state.cheapest_win(max) {
                if let Some(best) = &best_so_far {
                    if best.spent > cheapest.spent {
                        best_so_far = Some(cheapest);
                    }
                } else {
                    best_so_far = Some(cheapest);
                }
            }
        }

        best_so_far
    }
}
#[derive(Debug, Clone)]
struct Spell {
    name: &'static str,
    cost: i32,
    damage: i32,
    heal: i32,
    armor: i32,
    mana: i32,
    duration: i32,
}

impl Spell {
    fn tick(&self) -> Self {
        let mut next = self.clone();
        next.duration -= 1;
        next
    }
}

const SPELLS: [Spell; 5] = [
    Spell {
        name: "Magic Missile",
        cost: 53,
        damage: 4,
        heal: 0,
        armor: 0,
        mana: 0,
        duration: 0,
    },
    Spell {
        name: "Drain",
        cost: 73,
        damage: 2,
        heal: 2,
        armor: 0,
        mana: 0,
        duration: 0,
    },
    Spell {
        name: "Shield",
        cost: 113,
        damage: 0,
        heal: 0,
        armor: 7,
        mana: 0,
        duration: 6,
    },
    Spell {
        name: "Poison",
        cost: 173,
        damage: 3,
        heal: 0,
        armor: 0,
        mana: 0,
        duration: 6,
    },
    Spell {
        name: "Recharge",
        cost: 229,
        damage: 0,
        heal: 0,
        armor: 0,
        mana: 101,
        duration: 5,
    },
];
pub fn part_one(input: &str) -> Option<i32> {
    let (hp, dmg) = parse(input);
    let starting_state = GameState::new(hp, 55, 500, dmg);
    let win = starting_state.cheapest_win(i32::MAX).unwrap();
    Some(win.spent)
}

pub fn part_two(input: &str) -> Option<i32> {
    let (hp, dmg) = parse(input);
    let mut starting_state = GameState::new(hp, 55, 500, dmg);
    starting_state.difficulty = Difficulty::Hard;
    let win = starting_state.cheapest_win(i32::MAX).unwrap();
    Some(win.spent)
}
fn parse(input: &str) -> (i32, i32) {
    (
        input
            .lines()
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .parse()
            .unwrap(),
        input
            .lines()
            .nth(1)
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .parse()
            .unwrap(),
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(953));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_289));
    }
}
