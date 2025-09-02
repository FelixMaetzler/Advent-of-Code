use core::fmt::Write as _;
all_aoc::solution!(10, 2017);

pub fn part_one(input: &str) -> Option<usize> {
    let mut vec = parse1(input);
    Some(execute1(&mut vec, 256))
}

pub fn part_two(input: &str) -> Option<String> {
    let vec = parse2(input);
    let mut list: Vec<_> = (0..256).collect();
    let mut curr_pos = 0;
    let mut curr_skip_size = 0;
    for _ in 0..64 {
        execute2(&vec, &mut list, &mut curr_pos, &mut curr_skip_size);
    }
    let dense_hash: Vec<_> = list
        .chunks(16)
        .map(|c| c.iter().copied().reduce(core::ops::BitXor::bitxor).unwrap())
        .collect();
    let string = dense_hash.iter().fold(String::new(), |mut output, b| {
        let _ = write!(output, "{b:02X}");
        output
    });

    Some(string.to_ascii_lowercase())
}

fn execute1(vec: &mut [usize], length: usize) -> usize {
    let mut list: Vec<_> = (0..length).collect();
    let mut curr_pos = 0;

    for (curr_skip_size, length) in vec.iter_mut().enumerate() {
        rev(&mut list, curr_pos, *length);
        curr_pos += *length + curr_skip_size;
    }
    list[0] * list[1]
}
fn execute2(
    vec: &Vec<usize>,
    list: &mut [usize],
    curr_pos: &mut usize,
    curr_skip_size: &mut usize,
) {
    for length in vec {
        rev(list, *curr_pos, *length);
        *curr_pos += length + *curr_skip_size;
        *curr_pos %= list.len();
        *curr_skip_size += 1;
    }
}
fn rev(list: &mut [usize], pos: usize, length: usize) {
    let vgl: Vec<_> = (pos..pos + length)
        .map(|i| list[i % list.len()])
        .rev()
        .collect();
    (0..length).for_each(|i| list[(i + pos) % list.len()] = vgl[i]);
}

fn parse1(input: &str) -> Vec<usize> {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}
fn parse2(input: &str) -> Vec<usize> {
    let mut v: Vec<_> = input.trim().chars().map(|c| c as usize).collect();
    v.append(&mut vec![17, 31, 73, 47, 23]);
    v
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_actual() {
        let result = part_one(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some(62_238));
    }

    #[test]
    fn test_part_two_actual() {
        let result = part_two(&all_aoc::cli::read_inputs_file(DAY));
        assert_eq!(result, Some("2b0c9cc0449507a0db3babd57ad9e8d8".to_owned()));
    }
}
