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
pub fn generate_combinations_until<T: Clone>(input: &[T], max_len: usize) -> Vec<Vec<T>> {
    generate_combinations(input)
        .into_iter()
        .filter(|v| v.len() <= max_len)
        .collect()
}
pub struct Combinator<T>
where
    T: Clone,
{
    vec: Vec<T>,
    counter: Counter,
}
impl<T> Combinator<T>
where
    T: Clone,
{
    pub fn new(vec: Vec<T>, k: usize) -> Self {
        let counter = Counter::new(vec.len(), k);
        Self { vec, counter }
    }
}
impl<T> Iterator for Combinator<T>
where
    T: Clone,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.counter
            .next()
            .map(|state| state.into_iter().map(|i| self.vec[i].clone()).collect())
    }
}
pub struct Counter {
    vector_len: usize,
    k: usize,
    state: Vec<usize>,
}
impl Counter {
    pub fn new(vector_len: usize, k: usize) -> Self {
        let mut state = (0..k).collect::<Vec<_>>();
        state[k - 1] -= 1;
        Self {
            vector_len,
            k,
            state,
        }
    }
}
impl Iterator for Counter {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut index = None;
        for i in (0..self.k).rev() {
            if self.state[i] < self.vector_len + i - self.k {
                index = Some(i);
                break;
            }
        }
        let index = index?;
        self.state[index] += 1;
        for i in index + 1..self.k {
            self.state[i] = self.state[i - 1] + 1;
        }
        Some(self.state.clone())
    }
}
pub fn powerset<T>(input: &[T]) -> impl Iterator<Item = Vec<T>> + use<'_, T>
where
    T: Clone,
{
    (0..1 << input.len()).map(move |bitmask| {
        input
            .iter()
            .enumerate()
            .filter_map(move |(i, item)| {
                if bitmask & (1 << i) != 0 {
                    Some(item)
                } else {
                    None
                }
            })
            .cloned()
            .collect()
    })
}
#[cfg(test)]
mod tests {
    use crate::helper::permutations::{generate_combinations, powerset};

    use super::Counter;

    #[test]
    fn generate_combinations_test() {
        let gen = generate_combinations(&[1, 2]);
        assert_eq!(gen.len(), 4);
        assert!(gen.contains(&vec![]));
        assert!(gen.contains(&vec![1]));
        assert!(gen.contains(&vec![2]));
        assert!(gen.contains(&vec![1, 2]));
    }
    #[test]
    fn powerset_test() {
        let gen = powerset(&[1, 2]).collect::<Vec<_>>();
        assert_eq!(gen.len(), 4);
        assert!(gen.contains(&vec![]));
        assert!(gen.contains(&vec![1]));
        assert!(gen.contains(&vec![2]));
        assert!(gen.contains(&vec![1, 2]));
    }
    #[test]
    fn test_counter() {
        for n in 3..20 {
            for k in 2..n {
                let c = Counter::new(n, k);
                let vec = c.into_iter().collect::<Vec<_>>();
                assert_eq!(vec.len(), n_choose_k(n, k));
                for v in &vec {
                    assert!(v.windows(2).all(|w| w[0] < w[1]))
                }
            }
        }
    }
    fn n_choose_k(n: usize, k: usize) -> usize {
        (k + 1..=n).product::<usize>() / fakultaet(n - k)
    }
    fn fakultaet(i: usize) -> usize {
        (1..=i).product()
    }
}
