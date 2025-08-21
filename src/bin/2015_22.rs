all_aoc::solution!(22, 2015);

pub fn part_one(input: &str) -> Option<i32> {
    Some(solve(input, Difficulty::Easy))
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(solve(input, Difficulty::Hard))
}
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
    gamelog: String,
}

impl GameState {
    const fn new(player_hp: i32, boss_hp: i32, mana: i32, damage: i32) -> Self {
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
            gamelog: String::new(),
        }
    }

    fn start_turn(&self) -> Option<Self> {
        let mut player_hp = self.player_hp;
        let mut boss_hp = self.boss_hp;
        let mut mana = self.mana;
        let mut armor = 0;
        let mut effects = vec![];
        let mut gamelog = if self.turn > 0 {
            format!("{}\n", self.gamelog)
        } else {
            String::new()
        };
        if self.turn % 2 == 0 {
            player_hp -= self.difficulty as i32;
            if player_hp <= 0 {
                return None;
            }
        }

        gamelog = format!(
            "{gamelog}-- {} turn --\n- Player has {} hit points, {} armor, {} mana\n- Boss has {} hit points\n",
            if self.turn % 2 == 0 { "Player" } else { "Boss" },
            player_hp,
            self.armor,
            self.mana,
            self.boss_hp
        );

        for effect in &self.effects {
            if effect.armor > 0 {
                armor = effect.armor;
            }

            if effect.damage > 0 {
                boss_hp -= effect.damage;
                gamelog = format!(
                    "{gamelog}{} deals {} damage; its timer is now {}\n",
                    effect.name,
                    effect.damage,
                    effect.duration - 1
                );
            }

            if effect.mana > 0 {
                mana += effect.mana;
                gamelog = format!(
                    "{gamelog}{} provides {} mana; its timer is now {}\n",
                    effect.name,
                    effect.mana,
                    effect.duration - 1
                );
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
            gamelog,
        })
    }

    fn cast(&self, spell: &Spell) -> Self {
        let gamelog = format!("{}Player casts {}\n", self.gamelog, spell.name);
        assert!((self.mana >= spell.cost), "bug in move generator");

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
                gamelog,
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
                gamelog,
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
        let gamelog = format!("{}Boss attacks for {damage} damage\n", self.gamelog);
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
            gamelog,
        }
    }

    fn cheapest_win(&self, mut max: i32) -> Option<Self> {
        let mut best_so_far: Option<Self> = None;

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

            if let Some(best) = &best_so_far
                && best.spent < max
            {
                max = best.spent;
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

// --------------------------------------------------------------------------------

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

fn solve(input: &str, diff: Difficulty) -> i32 {
    let (hit_points, dmg) = parse(input);
    let mut starting_state = GameState::new(50, hit_points, 500, dmg);
    starting_state.difficulty = diff;
    let win = starting_state.cheapest_win(i32::MAX).unwrap();
    win.spent
}
fn parse(input: &str) -> (i32, i32) {
    let (hit_points, dmg) = input.split_once('\n').unwrap();
    let hit_points = hit_points
        .trim_start_matches("Hit Points: ")
        .parse()
        .unwrap();
    let dmg = dmg.trim_start_matches("Damage: ").parse().unwrap();
    (hit_points, dmg)
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
