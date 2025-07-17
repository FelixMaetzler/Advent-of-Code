pub trait IteratorPermutator: Iterator {
    fn permutations(self, k: usize) -> Permutator<<Self as Iterator>::Item>
    where
        Self: Sized,
        <Self as std::iter::Iterator>::Item: Clone,
    {
        let collect = self.collect::<Vec<_>>();
        Permutator::new(collect, k)
    }

    fn permutations_until(self, k: usize) -> impl Iterator<Item = Vec<<Self as Iterator>::Item>>
    where
        Self: Sized,
        <Self as std::iter::Iterator>::Item: Clone,
    {
        let collect = self.collect::<Vec<_>>();
        (0..=k).flat_map(move |size| Permutator::new(collect.clone(), size))
    }
    fn permutation(self) -> Permutator<<Self as Iterator>::Item>
    where
        Self: Sized,
        <Self as std::iter::Iterator>::Item: Clone,
    {
        let collect = self.collect::<Vec<_>>();
        let len = collect.len();
        Permutator::new(collect, len)
    }
}

// Implement the trait for all iterators
impl<T> IteratorPermutator for T where T: Iterator {}

/// Permutator struct to handle permutations
#[derive(Clone)]
pub struct Permutator<T> {
    items: Vec<T>,
    indices: Vec<usize>,
    cycles: Vec<usize>,
    started: bool,
}

impl<T> Permutator<T> {
    pub fn new(items: Vec<T>, k: usize) -> Self {
        let n = items.len();
        let cycles = if k > n {
            (0..k).collect()
        } else {
            (0..k).map(|i| n - i).collect()
        };
        Self {
            items,
            indices: (0..n).collect(),
            cycles,
            started: false,
        }
    }
}

impl<T: Clone> Iterator for Permutator<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let k = self.cycles.len();

        if k > self.items.len() {
            return None;
        }

        if !self.started {
            self.started = true;
            return Some(
                self.indices[..k]
                    .iter()
                    .map(|&i| self.items[i].clone())
                    .collect(),
            );
        }

        for i in (0..k).rev() {
            self.cycles[i] -= 1;

            if self.cycles[i] == 0 {
                self.indices[i..].rotate_left(1);
                self.cycles[i] = self.items.len() - i;
            } else {
                self.indices.swap(i, self.items.len() - self.cycles[i]);
                return Some(
                    self.indices[..k]
                        .iter()
                        .map(|&i| self.items[i].clone())
                        .collect(),
                );
            }
        }

        None
    }
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
    fn combinations_until<'a>(self, k: usize) -> impl Iterator<Item = Vec<<Self as Iterator>::Item>>
    where
        Self: Sized,
        <Self as std::iter::Iterator>::Item: Clone + 'a,
    {
        let collect = self.collect::<Vec<_>>();

        (0..=k)
            .map(move |k| Combinator::new(collect.clone(), k))
            .flat_map(std::iter::IntoIterator::into_iter)
    }
    fn powerset<'a>(self) -> impl Iterator<Item = Vec<<Self as Iterator>::Item>>
    where
        Self: Sized,
        <Self as std::iter::Iterator>::Item: Clone + 'a,
    {
        let collect = self.collect::<Vec<_>>();

        (0..=collect.len())
            .map(move |k| Combinator::new(collect.clone(), k))
            .flat_map(std::iter::IntoIterator::into_iter)
    }
}

impl<T> IteratorCombinator for T where T: Iterator {}
#[cfg(test)]
mod tests {

    use crate::helper::permutations::IteratorPermutator;

    use super::{Counter, IteratorCombinator};

    #[test]
    fn powerset_test() {
        let g = [1, 2].into_iter().powerset().collect::<Vec<_>>();
        assert_eq!(g.len(), 4);
        assert!(g.contains(&vec![]));
        assert!(g.contains(&vec![1]));
        assert!(g.contains(&vec![2]));
        assert!(g.contains(&vec![1, 2]));
    }
    #[test]
    fn test_counter_normal() {
        for n in 1..20 {
            for k in 0..n {
                let c = Counter::new(n, k);
                let vec = c.into_iter().collect::<Vec<_>>();
                assert_eq!(vec.len(), n_choose_k(n, k));
                for v in &vec {
                    assert!(v.windows(2).all(|w| w[0] < w[1]));
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
                assert_eq!(c.into_iter().count(), 0);
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
                    assert!(v.windows(2).all(|w| w[0] < w[1]));
                }
            }
        }
    }
    #[test]
    fn test_combinator_all_iterator() {
        for n in 0..20 {
            for k in 0..20 {
                let vec = (0..n).combinations(k).collect::<Vec<_>>();
                assert_eq!(vec.len(), n_choose_k(n, k));
                for v in &vec {
                    assert!(v.windows(2).all(|w| w[0] < w[1]));
                }
            }
        }
    }
    #[test]
    fn test_permutator_all_iterator() {
        for n in 0..10 {
            for k in 0..=10 {
                assert_eq!(
                    (0..n).permutations(k).count(),
                    n_permute_k(n, k),
                    "n: {n}, k: {k}"
                );
            }
        }
    }
    #[test]
    fn test_combinator_all_until_iterator() {
        for n in 0..15 {
            for k in 0..15 {
                let vec = (0..n).combinations_until(k).collect::<Vec<_>>();
                assert_eq!(vec.len(), (0..=k).map(|i| n_choose_k(n, i)).sum());
                for v in &vec {
                    assert!(v.windows(2).all(|w| w[0] < w[1]));
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
                assert!(v.windows(2).all(|w| w[0] < w[1]));
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
    fn n_permute_k(n: usize, k: usize) -> usize {
        if k > n {
            0
        } else {
            ((n - k + 1)..=n).product()
        }
    }
    fn fakultaet(i: usize) -> usize {
        (1..=i).product()
    }
    #[test]
    fn test_n_k() {
        for n in 0..20 {
            for k in 0..=n {
                assert_eq!(
                    n_choose_k(n, k),
                    fakultaet(n) / (fakultaet(n - k) * fakultaet(k))
                );
                assert_eq!(n_permute_k(n, k), fakultaet(n) / (fakultaet(n - k)));
            }
        }
    }
}
