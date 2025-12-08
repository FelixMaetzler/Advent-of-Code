use core::cmp::Ordering;

pub struct DisjointUnionSets {
    parent: Vec<usize>,
    rank: Vec<u8>,
    components: usize,
}
impl DisjointUnionSets {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            components: n,
        }
    }
    /// Finds the root of the set with i.
    /// Because of pathcompression, this method takes a mut reference.
    pub fn find(&mut self, i: usize) -> usize {
        assert!(i < self.parent.len(), "index out of bounds");
        if self.parent[i] != i {
            let root = self.find(self.parent[i]);
            self.parent[i] = root;
        }
        self.parent[i]
    }
    /// unites the sets with x and y in there.
    /// returns wether actual merging was done.
    pub fn union_sets(&mut self, x: usize, y: usize) -> bool {
        assert!(x < self.parent.len(), "index out of bounds");
        assert!(y < self.parent.len(), "index out of bounds");

        let x_root = self.find(x);
        let y_root = self.find(y);
        if x_root == y_root {
            return false; // already in the same set
        }

        // union by rank
        let rx = self.rank[x_root];
        let ry = self.rank[y_root];
        match rx.cmp(&ry) {
            Ordering::Less => self.parent[x_root] = y_root,
            Ordering::Equal => {
                self.parent[y_root] = x_root;
                self.rank[x_root] = rx + 1;
            }
            Ordering::Greater => self.parent[y_root] = x_root,
        }
        self.components -= 1;
        true
    }
    /// returns wether all sets are unified in O(1).
    pub const fn is_all_unified(&self) -> bool {
        self.num_components() <= 1
    }
    /// Number of disjoint sets in O(1).
    pub const fn num_components(&self) -> usize {
        self.components
    }
    /// checks if two indexes are in the same set.
    ///Performing pathcompression requires mutable access.
    pub fn same_set(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let n = self.parent.len();
        let mut buckets = vec![Vec::new(); n];
        for i in 0..n {
            let r = self.find(i);
            buckets[r].push(i);
        }
        buckets.retain(|v| !v.is_empty());
        buckets
    }
}
