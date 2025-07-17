use std::collections::HashMap;

all_aoc::solution!(10, 2016);
#[derive(Debug, Clone, Copy)]
enum GoesTo {
    Bot(u8),
    Output(u8),
}
impl GoesTo {
    fn from_pair<'a>(v_1: &'a str, v_2: &'a str) -> Result<Self, &'a str> {
        match v_1 {
            "output" => v_2.parse().map_or(Err(v_2), |x| Ok(Self::Output(x))),
            "bot" => v_2.parse().map_or(Err(v_2), |x| Ok(Self::Bot(x))),
            x => Err(x),
        }
    }
}
#[derive(Debug, Clone, Copy)]
struct Bot {
    number: u8,
    low: Option<u8>,
    high: Option<u8>,
    low_goes_to: GoesTo,
    high_goes_to: GoesTo,
}
impl Bot {
    fn assign(&mut self, num: u8) {
        match (self.low, self.high) {
            (None, None) => self.low = Some(num),
            (None, Some(_)) => unreachable!("is not allowed"),
            (Some(y), None) => {
                if num < y {
                    self.low = Some(num);
                    self.high = Some(y);
                } else {
                    self.high = Some(num);
                }
            }
            (Some(_), Some(_)) => unreachable!("should not happen"),
        }
    }
    const fn is_executable(&self) -> bool {
        self.low.is_some() && self.high.is_some()
    }
}
fn execute(input: &str, part_1: bool) -> Option<u32> {
    let mut map = parse(input);
    let mut going_on = true;
    let mut output = HashMap::new();
    while going_on {
        going_on = false;
        if let Some(&bot) = map.values().find(|b| b.is_executable()) {
            if part_1 && (Some(17), Some(61)) == (bot.low, bot.high) {
                return Some(u32::from(bot.number));
            }
            match bot.low_goes_to {
                GoesTo::Bot(b) => map.get_mut(&b).unwrap().assign(bot.low.unwrap()),
                GoesTo::Output(b) => {
                    output.insert(b, bot.low.unwrap());
                }
            }
            match bot.high_goes_to {
                GoesTo::Bot(b) => map.get_mut(&b).unwrap().assign(bot.high.unwrap()),
                GoesTo::Output(b) => {
                    output.insert(b, bot.high.unwrap());
                }
            }
            let bot = map.get_mut(&bot.number).unwrap();
            bot.low = None;
            bot.high = None;
            going_on = true;
        }
    }
    if part_1 {
        None
    } else {
        Some(u32::from(output[&0]) * u32::from(output[&1]) * u32::from(output[&2]))
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    execute(input, true)
}

pub fn part_two(input: &str) -> Option<u32> {
    execute(input, false)
}
fn parse(input: &str) -> HashMap<u8, Bot> {
    let mut map = HashMap::new();
    for line in input.lines().filter(|s| s.starts_with('b')) {
        let v = line.split_ascii_whitespace().collect::<Vec<_>>();
        assert_eq!(v.len(), 12);
        let number = v[1].parse().unwrap();
        let low_goes_to = GoesTo::from_pair(v[5], v[6]).unwrap();
        let high_goes_to = GoesTo::from_pair(v[10], v[11]).unwrap();

        let res = map.insert(
            number,
            Bot {
                number,
                low: None,
                high: None,
                low_goes_to,
                high_goes_to,
            },
        );
        debug_assert!(res.is_none());
    }
    for line in input.lines().filter(|s| s.starts_with('v')) {
        let v = line.split_ascii_whitespace().collect::<Vec<_>>();
        assert_eq!(v.len(), 6);
        debug_assert_eq!(v[4], "bot");
        let num = v[1].parse().unwrap();
        let bot = v[5].parse().unwrap();
        let b = map
            .get_mut(&bot)
            .unwrap_or_else(|| panic!("Bot {num} should be in there"));
        b.assign(num);
    }
    map
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(116));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(23_903));
    }
}
