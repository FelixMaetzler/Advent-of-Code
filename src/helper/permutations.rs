fn generate_permutations_recursive<T: Clone>(vec: &[T], start: usize, result: &mut Vec<Vec<T>>) {
    let mut vec = vec.to_vec();

    if start == vec.len() {
        result.push(vec);
        return;
    }

    for i in start..vec.len() {
        vec.swap(start, i);
        generate_permutations_recursive(&vec, start + 1, result);
    }
}
/// Given a Slice, this function returns all possible permutations of it
pub fn generate_permutations<T: Clone>(vec: &[T]) -> Vec<Vec<T>> {
    let mut ret = Vec::new();
    generate_permutations_recursive(vec, 0, &mut ret);
    ret
}
/// generates all Combinations from the given slice
/// [1, 2] gives you [[], [1], [2], [1, 2]]
pub fn generate_combinations<T: Clone>(input: &[T]) -> Vec<Vec<T>> {
    let mut result = vec![vec![]];

    for item in input {
        let mut new_combinations = Vec::new();
        for combination in &result {
            let mut new_combination = combination.clone();
            new_combination.push(item.clone());
            new_combinations.push(new_combination);
        }
        result.extend(new_combinations);
    }

    result
}
#[cfg(test)]
mod tests {
    use crate::helper::permutations::generate_combinations;

    #[test]
    fn generate_combinations_test() {
        let gen = generate_combinations(&[1, 2]);
        assert_eq!(gen.len(), 4);
        assert!(gen.contains(&vec![]));
        assert!(gen.contains(&vec![1]));
        assert!(gen.contains(&vec![2]));
        assert!(gen.contains(&vec![1, 2]));
    }
}
