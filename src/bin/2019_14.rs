use std::{collections::HashMap, str::FromStr};

all_aoc::solution!(14, 2019);
#[derive(Debug)]
struct Chemical {
    amount: usize,
    name: String,
}
impl FromStr for Chemical {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(' ').unwrap();
        Ok(Self {
            amount: left.parse().unwrap(),
            name: right.to_owned(),
        })
    }
}
#[derive(Debug)]
struct Reaction {
    output: Chemical,
    ingredients: Vec<Chemical>,
}
impl FromStr for Reaction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ingredients, output) = s.split_once(" => ").unwrap();
        let output = Chemical::from_str(output).unwrap();
        let ingredients = ingredients
            .split(", ")
            .map(|i| Chemical::from_str(i).unwrap())
            .collect();
        Ok(Self {
            output,
            ingredients,
        })
    }
}
pub fn part_one(input: &str) -> Option<usize> {
    let vec = parse(input);

    let receips: HashMap<String, Reaction> = vec
        .into_iter()
        .map(|r| (r.output.name.clone(), r))
        .collect();
    Some(solve_part_one(1, "FUEL".to_owned(), &receips))
}
pub fn part_two(input: &str) -> Option<usize> {
    let vec = parse(input);

    let receips: HashMap<String, Reaction> = vec
        .into_iter()
        .map(|r| (r.output.name.clone(), r))
        .collect();
    solve_part_2(&receips, 1_000_000_000_000)
}
fn solve_part_one(amount: usize, name: String, receips: &HashMap<String, Reaction>) -> usize {
    let mut queue = vec![Chemical { amount, name }];
    let mut ore = 0;
    let mut map = HashMap::new();
    while let Some(ingredient) = queue.pop() {
        if ingredient.name == "ORE" {
            ore += ingredient.amount;
        } else {
            let to_be_produced = use_leftovers(&ingredient, &mut map);
            if to_be_produced == 0 {
                continue;
            }
            let reaction = receips.get(&ingredient.name).unwrap();
            let times = to_be_produced.div_ceil(reaction.output.amount);
            if let Some(result) = (reaction.output.amount * times).checked_sub(to_be_produced)
                && result > 0
            {
                *map.entry(ingredient.name).or_insert(0) += result;
            }
            for req in &reaction.ingredients {
                queue.push(Chemical {
                    amount: req.amount * times,
                    name: req.name.clone(),
                });
            }
        }
    }
    ore
}
fn use_leftovers(ingredient: &Chemical, map: &mut HashMap<String, usize>) -> usize {
    #[expect(
        clippy::option_if_let_else,
        reason = "cant refactor becuase of mut borrow"
    )]
    if let Some(amount) = map.get_mut(&ingredient.name) {
        if ingredient.amount <= *amount {
            *amount -= ingredient.amount;
            0
        } else {
            let remain = ingredient.amount - *amount;
            map.remove(&ingredient.name);
            remain
        }
    } else {
        ingredient.amount
    }
}

fn solve_part_2(receips: &HashMap<String, Reaction>, goal: usize) -> Option<usize> {
    let cost_one = solve_part_one(1, "FUEL".to_owned(), receips);
    let mut fuel_l = goal.div_euclid(cost_one);
    let mut fuel_r = fuel_l * 3;
    let mut modified_l = false;
    let mut modified_r = false;
    while fuel_r - fuel_l != 1 {
        let fuel = fuel_l.midpoint(fuel_r);
        let cost = solve_part_one(fuel, "FUEL".to_owned(), receips);

        if cost < goal {
            fuel_l = fuel;
            modified_l = true;
        } else {
            fuel_r = fuel;
            modified_r = true;
        }
    }
    (modified_l && modified_r).then_some(fuel_l)
}
fn parse(input: &str) -> Vec<Reaction> {
    input
        .lines()
        .map(|l| Reaction::from_str(l).unwrap())
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = &all_aoc::cli::read_examples_file(DAY);
        let mut iter = input.split("\n\n");

        assert_eq!(part_one(iter.next().unwrap()), Some(31));
        assert_eq!(part_one(iter.next().unwrap()), Some(165));
        assert_eq!(part_one(iter.next().unwrap()), Some(13312));
        assert_eq!(part_one(iter.next().unwrap()), Some(180_697));
        assert_eq!(part_one(iter.next().unwrap()), Some(2_210_736));
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(2_556_890));
    }

    #[test]
    fn test_part_two() {
        let input = &all_aoc::cli::read_examples_file(DAY);
        let mut iter = input.split("\n\n").skip(2);
        assert_eq!(part_two(iter.next().unwrap()), Some(82_892_753));
        assert_eq!(part_two(iter.next().unwrap()), Some(5_586_022));
        assert_eq!(part_two(iter.next().unwrap()), Some(460_664));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(1_120_408));
    }
}
