use all_aoc::helper::md5;

all_aoc::solution!(17, 2016);
struct State {
    curr_room: isize,
    history: String,
}
impl State {
    fn next(&self, password: &str) -> Vec<Self> {
        let hash = md5::md5(&(password.to_owned() + &self.history.clone()));
        let neigbours = neigbours(self.curr_room);
        let mut ret = vec![];
        if ('b'..='f').contains(&hash.chars().nth(0).unwrap())
            && neigbours.contains(&(self.curr_room - 4))
        {
            //UP
            ret.push(Self {
                curr_room: (self.curr_room - 4),
                history: self.history.chars().chain("U".chars()).collect(),
            });
        }
        if ('b'..='f').contains(&hash.chars().nth(1).unwrap())
            && neigbours.contains(&(self.curr_room + 4))
        {
            //DOWN
            ret.push(Self {
                curr_room: (self.curr_room + 4),
                history: self.history.chars().chain("D".chars()).collect(),
            });
        }
        if ('b'..='f').contains(&hash.chars().nth(2).unwrap()) && self.curr_room % 4 != 0 {
            //LEFT
            ret.push(Self {
                curr_room: (self.curr_room - 1),
                history: self.history.chars().chain("L".chars()).collect(),
            });
        }
        if ('b'..='f').contains(&hash.chars().nth(3).unwrap()) && self.curr_room % 4 != 3 {
            //Right
            ret.push(Self {
                curr_room: (self.curr_room + 1),
                history: self.history.chars().chain("R".chars()).collect(),
            });
        }
        ret
    }
    fn shortest(&self, password: &str) -> Option<String> {
        if self.curr_room == 15 {
            return Some(self.history.clone());
        }
        let mut min: Option<String> = None;
        for state in self.next(password) {
            if let Some(x) = state.shortest(password) {
                match &min {
                    Some(y) => {
                        if x.len() < y.len() {
                            min = Some(x);
                        }
                    }
                    None => min = Some(x),
                }
            }
        }
        min
    }
    fn longest(&self, password: &str) -> Option<usize> {
        if self.curr_room == 15 {
            return Some(self.history.len());
        }
        let mut max: Option<usize> = None;
        for state in self.next(password) {
            max = max.max(state.longest(password));
        }
        max
    }
}
fn neigbours(input: isize) -> Vec<isize> {
    match input {
        0 => vec![1, 4],
        1 => vec![0, 2, 5],
        2 => vec![1, 3, 6],
        3 => vec![2, 7],
        4 => vec![0, 5, 8],
        5 => vec![1, 4, 6, 9],
        6 => vec![2, 5, 7, 10],
        7 => vec![3, 6, 11],
        8 => vec![4, 9, 12],
        9 => vec![5, 8, 10, 13],
        10 => vec![6, 9, 11, 14],
        11 => vec![7, 10, 15],
        12 => vec![8, 9, 13],
        13 => vec![9, 12, 14],
        14 => vec![10, 13, 15],
        15 => vec![10, 11, 14],
        _ => unreachable!(),
    }
}
pub fn part_one(input: &str) -> Option<String> {
    let state = State {
        curr_room: 0,
        history: String::new(),
    };
    Some(state.shortest(input).unwrap())
}

pub fn part_two(input: &str) -> Option<usize> {
    let state = State {
        curr_room: 0,
        history: String::new(),
    };
    Some(state.longest(input).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = &all_aoc::cli::read_examples_file(DAY);
        let mut it = input.lines();

        assert_eq!(part_one(it.next().unwrap()), Some("DDRRRD".to_owned()));
        assert_eq!(
            part_one(it.next().unwrap()),
            Some("DDUDRLRRUDRD".to_owned())
        );
        assert_eq!(
            part_one(it.next().unwrap()),
            Some("DRURDRUDDLLDLUURRDULRLDUUDDDRR".to_owned())
        );
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some("RDURRDDLRD".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let input = &all_aoc::cli::read_examples_file(DAY);
        let mut it = input.lines();

        assert_eq!(part_two(it.next().unwrap()), Some(370));
        assert_eq!(part_two(it.next().unwrap()), Some(492));
        assert_eq!(part_two(it.next().unwrap()), Some(830));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(526));
    }
}
