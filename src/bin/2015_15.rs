use core::str::FromStr;

all_aoc::solution!(15, 2015);
struct Ingridient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}
impl FromStr for Ingridient {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec = s.split(' ').collect::<Vec<_>>();
        let capacity = vec[2].trim_end_matches(',').parse().unwrap();
        let durability = vec[4].trim_end_matches(',').parse().unwrap();
        let flavor = vec[6].trim_end_matches(',').parse().unwrap();
        let texture = vec[8].trim_end_matches(',').parse().unwrap();
        let calories = vec[10].trim_end_matches(',').parse().unwrap();
        Ok(Self {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        })
    }
}
pub fn part_one(input: &str) -> Option<i32> {
    let vec = parse(input);
    let comb = generate_combinations(100, vec.len().try_into().unwrap());

    comb.into_iter()
        .filter(|v| v.iter().sum::<i32>() == 100)
        .map(|comb| calc(&vec, &comb))
        .max()
}

pub fn part_two(input: &str) -> Option<i32> {
    let vec = parse(input);
    let comb = generate_combinations(100, vec.len().try_into().unwrap());
    comb.into_iter()
        .filter(|v| v.iter().sum::<i32>() == 100 && calories(&vec, v) == 500)
        .map(|comb| calc(&vec, &comb))
        .max()
}
fn calc(ingridients: &[Ingridient], comb: &[i32]) -> i32 {
    let mut capacity = 0;
    let mut durability = 0;
    let mut flavor = 0;
    let mut texture = 0;
    for i in 0..ingridients.len() {
        capacity += comb[i] * ingridients[i].capacity;
        durability += comb[i] * ingridients[i].durability;
        flavor += comb[i] * ingridients[i].flavor;
        texture += comb[i] * ingridients[i].texture;
    }
    capacity = capacity.max(0);
    durability = durability.max(0);
    flavor = flavor.max(0);
    texture = texture.max(0);
    capacity * durability * flavor * texture
}
fn calories(ingridients: &[Ingridient], comb: &[i32]) -> i32 {
    let mut calories = 0;
    for i in 0..ingridients.len() {
        calories += comb[i] * ingridients[i].calories;
    }
    calories
}
fn parse(input: &str) -> Vec<Ingridient> {
    input
        .lines()
        .map(|l| Ingridient::from_str(l).unwrap())
        .collect()
}
fn generate_combinations(base: i32, n: u32) -> Vec<Vec<i32>> {
    let i = (base + 1).pow(n);
    (0..i).map(|i| split_and_pad(i, base, n)).collect()
}
fn split_and_pad(mut num: i32, base: i32, n: u32) -> Vec<i32> {
    let mut digits = Vec::with_capacity(n as usize);

    while num > 0 {
        digits.push(num % base);
        num /= base;
    }
    digits.reverse();
    while digits.len() < n as usize {
        digits.insert(0, 0);
    }

    digits
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(62_842_880));
    }
    #[cfg(feature = "expensive")]
    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(222_870));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&all_aoc::cli::read_examples_file(DAY));
        assert_eq!(result, Some(57_600_000));
    }
    #[cfg(feature = "expensive")]
    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(117_936));
    }
}
