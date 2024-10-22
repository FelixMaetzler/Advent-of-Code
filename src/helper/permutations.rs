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
