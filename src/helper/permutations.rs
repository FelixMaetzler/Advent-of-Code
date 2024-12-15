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
    first: bool,
}
impl Counter {
    pub fn new(vector_len: usize, k: usize) -> Self {
        let state = (0..k).collect::<Vec<_>>();

        Self {
            vector_len,
            k,
            state,
            first: true,
        }
    }
}
impl Iterator for Counter {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.k > self.vector_len {
            return None;
        }
        if self.first {
            self.first = false;
            return if self.k == 0 {
                Some(vec![])
            } else {
                Some(self.state.clone())
            };
        }
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
} // Define the extension trait
pub trait IteratorCombinator: Iterator {
    fn combinations(self, k: usize) -> Combinator<<Self as Iterator>::Item>
    where
        Self: Sized,
        <Self as std::iter::Iterator>::Item: Clone,
    {
        let collect = self.collect::<Vec<_>>();
        let c = Combinator::new(collect, k);
        c.into_iter()
    }
    fn combinations_until(self, k: usize) -> impl Iterator<Item = Vec<<Self as Iterator>::Item>>
    where
        Self: Sized,
        <Self as std::iter::Iterator>::Item: Clone,
    {
        let collect = self.collect::<Vec<_>>();
        let combinators = (0..=k)
            .map(|k| Combinator::new(collect.clone(), k))
            .collect::<Vec<_>>();
        combinators.into_iter().flat_map(|c| c.into_iter())
    }
    fn powerset(self) -> impl Iterator<Item = Vec<<Self as Iterator>::Item>>
    where
        Self: Sized,
        <Self as std::iter::Iterator>::Item: Clone,
    {
        let collect = self.collect::<Vec<_>>();
        let combinators = (0..=collect.len())
            .map(|k| Combinator::new(collect.clone(), k))
            .collect::<Vec<_>>();
        combinators.into_iter().flat_map(|c| c.into_iter())
    }
}

// Implement the trait for all iterators
impl<T> IteratorCombinator for T where T: Iterator {}
#[cfg(test)]
mod tests {
    use crate::helper::permutations::powerset;

    use super::{Counter, IteratorCombinator};

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
    fn test_counter_normal() {
        for n in 1..20 {
            for k in 0..n {
                let c = Counter::new(n, k);
                let vec = c.into_iter().collect::<Vec<_>>();
                assert_eq!(vec.len(), n_choose_k(n, k));
                for v in &vec {
                    assert!(v.windows(2).all(|w| w[0] < w[1]))
                }
            }
        }
    }
    #[test]
    fn test_counter_k_0() {
        let k = 0;
        for n in 3..20 {
            let c = Counter::new(n, k);
            let vec = c.into_iter().collect::<Vec<_>>();
            assert_eq!(vec.len(), 1);
            assert_eq!(vec[0], vec![]);
        }
    }
    #[test]
    fn test_counter_k_0_n_x() {
        for n in 0..100 {
            let c = Counter::new(n, 0);
            let vec = c.into_iter().collect::<Vec<_>>();
            assert_eq!(vec.len(), 1);
            assert_eq!(vec[0], vec![]);
        }
    }
    #[test]
    fn test_counter_k_bigger_n() {
        for k in 1..100 {
            for n in 0..k {
                let c = Counter::new(n, k);
                let vec = c.into_iter().collect::<Vec<_>>();
                assert_eq!(vec.len(), 0);
            }
        }
    }
    #[test]
    fn test_counter_all() {
        for n in 0..20 {
            for k in 0..20 {
                let c = Counter::new(n, k);
                let vec = c.into_iter().collect::<Vec<_>>();
                assert_eq!(vec.len(), n_choose_k(n, k));
                for v in &vec {
                    assert!(v.windows(2).all(|w| w[0] < w[1]))
                }
            }
        }
    }
    #[test]
    fn test_counter_all_iterator() {
        for n in 0..20 {
            for k in 0..20 {
                let vec = (0..n).combinations(k).collect::<Vec<_>>();
                assert_eq!(vec.len(), n_choose_k(n, k));
                for v in &vec {
                    assert!(v.windows(2).all(|w| w[0] < w[1]))
                }
            }
        }
    }
    #[test]
    fn test_counter_all_until_iterator() {
        for n in 0..15 {
            for k in 0..15 {
                let vec = (0..n).combinations_until(k).collect::<Vec<_>>();
                assert_eq!(vec.len(), (0..=k).map(|i| n_choose_k(n, i)).sum());
                for v in &vec {
                    assert!(v.windows(2).all(|w| w[0] < w[1]))
                }
            }
        }
    }
    #[test]
    fn test_counter_powerset() {
        for n in 0..20 {
            let vec = (0..n).powerset().collect::<Vec<_>>();
            assert_eq!(vec.len(), 2_usize.pow(n));
            for v in &vec {
                assert!(v.windows(2).all(|w| w[0] < w[1]))
            }
        }
    }
    fn n_choose_k(n: usize, k: usize) -> usize {
        if k > n {
            0
        } else {
            (k + 1..=n).product::<usize>() / fakultaet(n - k)
        }
    }
    fn fakultaet(i: usize) -> usize {
        (1..=i).product()
    }
}
