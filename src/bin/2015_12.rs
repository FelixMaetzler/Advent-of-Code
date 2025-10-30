use std::collections::HashMap;

all_aoc::solution!(12, 2015);
#[derive(Debug, PartialEq, Eq, Clone)]
enum JsonObject {
    Number(i32),
    String(String),
    Array(Vec<Self>),
    Dict(HashMap<String, Self>),
}
impl JsonObject {
    fn sum(&self) -> i32 {
        match self {
            Self::Number(x) => *x,
            Self::String(_) => 0,
            Self::Array(vec) => vec.iter().map(Self::sum).sum(),
            Self::Dict(hash_map) => hash_map.values().map(Self::sum).sum(),
        }
    }
    fn sum_without_red(&self) -> i32 {
        match self {
            Self::Number(x) => *x,
            Self::String(_) => 0,
            Self::Array(vec) => vec.iter().map(Self::sum_without_red).sum(),
            Self::Dict(hash_map) => {
                if hash_map.contains_key("red")
                    || hash_map
                        .values()
                        .any(|v| matches!(v,Self::String(x)if x=="red"))
                {
                    0
                } else {
                    hash_map.values().map(Self::sum_without_red).sum()
                }
            }
        }
    }
}

fn parse_number(input: &[char], start_index: usize) -> (i32, usize) {
    let mut index = start_index;
    while input[index].is_numeric() || input[index] == '-' {
        index += 1;
    }
    let ret = input[start_index..index]
        .iter()
        .collect::<String>()
        .parse()
        .unwrap();
    (ret, index)
}

fn parse_string(input: &[char], start_index: usize) -> (String, usize) {
    assert_eq!(input[start_index], '"');
    let mut index = start_index + 1;
    while input[index] != '"' {
        index += 1;
    }

    let ret = input[start_index + 1..index].iter().collect::<String>();
    index += 1;
    (ret, index)
}
fn parse_dict(input: &[char], start_index: usize) -> (HashMap<String, JsonObject>, usize) {
    assert_eq!(input[start_index], '{');
    let mut i = start_index + 1;
    let mut dict = HashMap::new();
    loop {
        let (key, y) = parse_string(input, i);
        i = y;
        assert_eq!(input[i], ':');
        i += 1;
        let (value, y) = parse_recursive(input, i);
        i = y;
        dict.insert(key, value);
        match input[i] {
            ',' => {
                i += 1;
            }
            '}' => {
                i += 1;
                return (dict, i);
            }
            x => unreachable!("uncorrect char {x} at index {i}"),
        }
    }
}
fn parse_array(input: &[char], start_index: usize) -> (Vec<JsonObject>, usize) {
    assert_eq!(input[start_index], '[');
    let mut i = start_index + 1;
    let mut vec = Vec::new();
    loop {
        let (obj, y) = parse_recursive(input, i);
        i = y;

        vec.push(obj);
        match input[i] {
            ',' => {
                i += 1;
            }
            ']' => {
                i += 1;
                return (vec, i);
            }
            x => unreachable!("uncorrect char {x} at index {i}"),
        }
    }
}
fn parse_recursive(vec: &[char], i: usize) -> (JsonObject, usize) {
    let mut i = i;
    match vec[i] {
        '0'..='9' | '-' => {
            let (ret, new) = parse_number(vec, i);
            i = new;
            (JsonObject::Number(ret), i)
        }
        '"' => {
            let (ret, new) = parse_string(vec, i);
            i = new;
            (JsonObject::String(ret), i)
        }
        '[' => {
            let (ret, new) = parse_array(vec, i);
            i = new;
            (JsonObject::Array(ret), i)
        }
        '{' => {
            let (ret, new) = parse_dict(vec, i);
            i = new;
            (JsonObject::Dict(ret), i)
        }
        x => unreachable!("uncorrect char {x} at index {i}"),
    }
}
fn parse(input: &str) -> JsonObject {
    let vec = input.chars().collect::<Vec<_>>();
    let (obj, i) = parse_recursive(&vec, 0);
    assert_eq!(input.len(), i);
    obj
}
pub fn part_one(input: &str) -> Option<i32> {
    let obj = parse(input);
    Some(obj.sum())
}

pub fn part_two(input: &str) -> Option<i32> {
    let obj = parse(input);
    Some(obj.sum_without_red())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_string_test() {
        let (x, y) = parse_string(&"\"abc\"".chars().collect::<Vec<_>>(), 0);
        assert_eq!(x, "abc");
        assert_eq!(y, 5);
    }
    #[test]
    fn parse_number_test() {
        let (x, y) = parse_number(&"12345 ".chars().collect::<Vec<_>>(), 0);
        assert_eq!(x, 12345);
        assert_eq!(y, 5);
    }
    #[test]
    fn parse_dict_test() {
        let (x, y) = parse_recursive(&"{\"a\":5}".chars().collect::<Vec<_>>(), 0);
        let mut dict = HashMap::new();
        dict.insert("a".to_owned(), JsonObject::Number(5));
        assert_eq!(x, JsonObject::Dict(dict.clone()));
        assert_eq!(y, 7);

        let (x, y) = parse_recursive(&"{\"a\":5,\"b\":7}".chars().collect::<Vec<_>>(), 0);
        dict.insert("b".to_owned(), JsonObject::Number(7));
        assert_eq!(x, JsonObject::Dict(dict));
        assert_eq!(y, 13);
    }

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(191_164));
    }

    #[test]
    fn test_part_two_actual() {
        assert_eq!(Some(0), part_two("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}"));
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(87_842));
    }
    #[test]
    fn test_part_two() {
        assert_eq!(Some(0), part_two("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}"));
    }
}
