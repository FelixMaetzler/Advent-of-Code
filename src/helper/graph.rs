use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    ops::Add,
};

use super::grid::{grid_index::GridIndex, Grid};

/// `curr_to_neighbor_comparison` is a function that
///  has as arguments the current node and the neighbor node of the grid
/// and has to determine if these nodes have a edge in the graph
pub fn build_graph4<T>(
    grid: &impl Grid<T>,
    curr_to_neighbor_comparison: impl Fn(&T, &T) -> bool,
) -> Vec<Vec<NodeIndex>>
where
    T: Clone,
{
    let mut vec = Vec::with_capacity(grid.len());
    for i in 0..grid.len() {
        let x = grid
            .get_neigbors4(i)
            .filter(|(_, val)| {
                curr_to_neighbor_comparison(
                    grid.get(i)
                        .expect("cant fail. Is checked to exist by get_neigbors4"),
                    val,
                )
            })
            .map(|(idx, _)| idx.to_flat_index(grid))
            .collect();
        vec.push(x);
    }
    vec
}
/// `curr_to_neighbor_comparison` is a function that
///  has as arguments the current node and the neighbor node of the grid
/// and has to determine if these nodes have a edge in the graph
pub fn build_graph8<T>(
    grid: &impl Grid<T>,
    curr_to_neighbor_comparison: impl Fn(&T, &T) -> bool,
) -> Vec<Vec<NodeIndex>>
where
    T: Clone,
{
    let mut vec = Vec::with_capacity(grid.len());
    for i in 0..grid.len() {
        let x = grid
            .get_neigbors8(i)
            .filter(|(_, val)| {
                curr_to_neighbor_comparison(
                    grid.get(i)
                        .expect("cant fail. Is checked to exist by get_neigbors4"),
                    val,
                )
            })
            .map(|(idx, _)| idx.to_flat_index(grid))
            .collect();
        vec.push(x);
    }
    vec
}
type NodeIndex = usize;
pub trait GraphWithoutWeights
where
    Self: Graph + Sized,
{
    fn add_edge(&mut self, from: NodeIndex, to: NodeIndex);
    fn from_edges(it: impl Iterator<Item = (NodeIndex, NodeIndex)>) -> Self {
        let mut g = Self::new();
        it.for_each(|(from, to)| g.add_edge(from, to));
        g
    }
}
pub trait GraphWithWeights<T>
where
    Self: Graph + Sized,
{
    fn weight(&self, from: NodeIndex, to: NodeIndex) -> Option<&T>;
    fn add_edge(&mut self, from: NodeIndex, to: NodeIndex, weight: T);
    fn from_edges(it: impl Iterator<Item = (NodeIndex, NodeIndex, T)>) -> Self {
        let mut g = Self::new();
        it.for_each(|(from, to, weight)| g.add_edge(from, to, weight));
        g
    }
    fn dijkstra(&self, start: NodeIndex) -> HashMap<NodeIndex, T>
    where
        T: Add<Output = T> + Default + Ord + Copy,
    {
        let mut dist = HashMap::new();
        let mut heap = BinaryHeap::new();
        let mut visited = HashSet::new();
        dist.insert(start, T::default());
        heap.push(State {
            cost: T::default(),
            position: start,
        });
        while let Some(State { cost, position }) = heap.pop() {
            if !visited.insert(position) {
                continue;
            }
            for &v in self.outgoing(position) {
                let u = position;
                let weight = *self.weight(u, v).unwrap();
                let next = State {
                    cost: cost + weight,
                    position: v,
                };
                match dist.get(&v) {
                    Some(&current_cost) => {
                        if next.cost < current_cost {
                            dist.insert(v, next.cost);
                            heap.push(next);
                        }
                    }
                    None => {
                        dist.insert(v, next.cost);
                        heap.push(next);
                    }
                }
            }
        }

        dist
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State<T> {
    cost: T,
    position: NodeIndex,
}
impl<T> Ord for State<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl<T> PartialOrd for State<T>
where
    T: PartialOrd + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
pub trait Graph {
    fn new() -> Self;
    fn outgoing(&self, index: NodeIndex) -> impl Iterator<Item = &NodeIndex>;
    fn all_paths(&self, start: NodeIndex, end: NodeIndex) -> Vec<Vec<NodeIndex>>
    where
        Self: Sized,
    {
        let mut result = Vec::new();
        let mut path = Vec::new();

        dfs(self, start, end, &mut path, &mut result);
        result
    }
}
fn dfs(
    graph: &impl Graph,
    current: NodeIndex,
    end: NodeIndex,
    path: &mut Vec<NodeIndex>,
    result: &mut Vec<Vec<NodeIndex>>,
) {
    path.push(current);

    if current == end {
        result.push(path.clone());
    } else {
        for &neighbor in graph.outgoing(current) {
            if !path.contains(&neighbor) {
                dfs(graph, neighbor, end, path, result);
            }
        }
    }

    path.pop();
}

impl GraphWithoutWeights for Vec<Vec<NodeIndex>> {
    fn add_edge(&mut self, from: NodeIndex, to: NodeIndex) {
        debug_assert!(!self[from].contains(&to));
        self[from].push(to);
    }
}
impl Graph for Vec<Vec<NodeIndex>> {
    fn new() -> Self {
        vec![vec![]]
    }

    fn outgoing(&self, index: NodeIndex) -> impl Iterator<Item = &NodeIndex> {
        self[index].iter()
    }
}

pub struct SpecialGraph<T> {
    adj_matrix: HashMap<NodeIndex, HashSet<NodeIndex>>,
    edges: HashMap<(NodeIndex, NodeIndex), T>,
}
impl<T> Graph for SpecialGraph<T> {
    fn new() -> Self {
        Self {
            adj_matrix: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    fn outgoing(&self, index: NodeIndex) -> impl Iterator<Item = &NodeIndex> {
        if let Some(x) = self.adj_matrix.get(&index) {
            x.iter()
        } else {
            panic!("This Node doesn't exist")
        }
    }
}
impl<T> GraphWithWeights<T> for SpecialGraph<T> {
    fn weight(&self, from: NodeIndex, to: NodeIndex) -> Option<&T> {
        self.edges.get(&(from, to))
    }

    fn add_edge(&mut self, from: NodeIndex, to: NodeIndex, weight: T) {
        self.adj_matrix
            .entry(from)
            .and_modify(|s| {
                s.insert(to);
            })
            .or_insert(HashSet::from_iter([to]));
        self.edges.insert((from, to), weight);
    }
}
