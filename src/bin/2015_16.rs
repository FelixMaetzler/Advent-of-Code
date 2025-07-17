use std::{collections::HashMap, str::FromStr};

all_aoc::solution!(16, 2015);
#[derive(Debug, PartialEq, Eq, Hash)]
enum Item {
    Children,
    Cats,
    Samoyeds,
    Pomeranians,
    Akitas,
    Vizslas,
    Goldfish,
    Trees,
    Cars,
    Perfumes,
}
impl FromStr for Item {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "children" => Ok(Self::Children),
            "cats" => Ok(Self::Cats),
            "samoyeds" => Ok(Self::Samoyeds),
            "pomeranians" => Ok(Self::Pomeranians),
            "akitas" => Ok(Self::Akitas),
            "vizslas" => Ok(Self::Vizslas),
            "goldfish" => Ok(Self::Goldfish),
            "trees" => Ok(Self::Trees),
            "cars" => Ok(Self::Cars),
            "perfumes" => Ok(Self::Perfumes),
            x => unreachable!("{x}"),
        }
    }
}
#[derive(Debug)]
struct Sue {
    number: u32,
    items: HashMap<Item, u32>,
}
fn initialize_goal() -> HashMap<Item, u32> {
    let mut goal = HashMap::new();
    goal.insert(Item::Children, 3);
    goal.insert(Item::Cats, 7);
    goal.insert(Item::Samoyeds, 2);
    goal.insert(Item::Pomeranians, 3);
    goal.insert(Item::Akitas, 0);
    goal.insert(Item::Vizslas, 0);
    goal.insert(Item::Goldfish, 5);
    goal.insert(Item::Trees, 3);
    goal.insert(Item::Cars, 2);
    goal.insert(Item::Perfumes, 1);
    goal
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut sues = parse(input);
    let goal = initialize_goal();
    for item in goal {
        sues.retain(|s| match s.items.get(&item.0) {
            Some(x) => *x == item.1,
            None => true,
        });
    }
    assert_eq!(sues.len(), 1);
    Some(sues.first().unwrap().number)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sues = parse(input);
    let goal = initialize_goal();
    for (search, goal_val) in goal {
        sues.retain(|s| match s.items.get(&search) {
            Some(x) if search == Item::Cats || search == Item::Trees => *x > goal_val,
            Some(x) if search == Item::Pomeranians || search == Item::Goldfish => *x < goal_val,
            Some(x) => *x == goal_val,
            None => true,
        });
    }
    assert_eq!(sues.len(), 1);
    Some(sues.first().unwrap().number)
}

fn parse(input: &str) -> Vec<Sue> {
    let mut sues = vec![];
    for line in input.lines() {
        let without = line.replace([':', ','], " ");
        let vec: Vec<_> = without.split_ascii_whitespace().collect();
        assert_eq!(vec.len(), 8);
        let sue = vec[1].parse::<u32>().unwrap();
        let mut items = HashMap::new();
        for i in [2, 4, 6] {
            let item = vec[i];
            let number = vec[i + 1].parse().unwrap();
            items.insert(item.parse().unwrap(), number);
        }
        sues.push(Sue { items, number: sue });
    }
    sues
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(213));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(323));
    }
}
