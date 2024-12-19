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
    for i in grid.all_indices() {
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
/// and has to determine if these nodes have a edge in the graph and the weight
pub fn build_graph4_special<T, W>(
    grid: &impl Grid<T>,
    curr_to_neighbor_comparison: impl Fn(&T, &T) -> Option<W> + Copy,
) -> SpecialGraph<W>
where
    T: Clone,
    W: Copy,
{
    SpecialGraph::from_edges(grid.all_indices().flat_map(|from| {
        grid.get_neigbors4(from).flat_map(move |(to_ind, to_val)| {
            curr_to_neighbor_comparison(
                grid.get(from).expect("all_indices already checked that"),
                to_val,
            )
            .map(|w| (from.to_flat_index(grid), to_ind.to_flat_index(grid), w))
        })
    }))
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
    for i in grid.all_indices() {
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
    T: Copy,
{
    fn weight(&self, from: NodeIndex, to: NodeIndex) -> Option<T>;
    fn add_edge(&mut self, from: NodeIndex, to: NodeIndex, weight: T);
    fn from_edges(it: impl Iterator<Item = (NodeIndex, NodeIndex, T)>) -> Self {
        let mut g = Self::new();
        it.for_each(|(from, to, weight)| g.add_edge(from, to, weight));
        g
    }
    fn dijkstra_distances(
        &self,
        start: NodeIndex,
        end: Option<NodeIndex>, // Optional early stopping criterion
    ) -> HashMap<NodeIndex, T>
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

            // Stop early if the target node is reached
            if let Some(target) = end {
                if position == target {
                    break;
                }
            }

            for v in self.outgoing(position) {
                let u = position;
                let weight = self.weight(u, v).unwrap();
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

    fn dijkstra_shortest_path(
        &self,
        start: NodeIndex,
        end: NodeIndex, // Required ending node
    ) -> (HashMap<NodeIndex, T>, Vec<NodeIndex>)
    where
        T: Add<Output = T> + Default + Ord + Copy,
    {
        let mut dist = HashMap::new();
        let mut paths = HashMap::new();
        let mut heap = BinaryHeap::new();
        let mut visited = HashSet::new();

        dist.insert(start, T::default());
        paths.insert(start, vec![start]); // Initialize the path for the start node
        heap.push(State {
            cost: T::default(),
            position: start,
        });

        while let Some(State { cost, position }) = heap.pop() {
            if !visited.insert(position) {
                continue;
            }

            // Stop when the target node is reached
            if position == end {
                break;
            }

            for v in self.outgoing(position) {
                let u = position;
                let weight = self.weight(u, v).unwrap();
                let next = State {
                    cost: cost + weight,
                    position: v,
                };

                match dist.get(&v) {
                    Some(&current_cost) => {
                        if next.cost < current_cost {
                            dist.insert(v, next.cost);
                            heap.push(next);

                            // Update the path to this node
                            let mut new_path = paths[&u].clone();
                            new_path.push(v);
                            paths.insert(v, new_path);
                        }
                    }
                    None => {
                        dist.insert(v, next.cost);
                        heap.push(next);

                        // Create the path to this node
                        let mut new_path = paths[&u].clone();
                        new_path.push(v);
                        paths.insert(v, new_path);
                    }
                }
            }
        }

        // Return the distance map and the shortest path to the target node
        (
            dist,
            paths.get(&end).cloned().unwrap_or_else(Vec::new), // Return an empty path if unreachable
        )
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
    fn incoming(&self, index: NodeIndex) -> impl Iterator<Item = NodeIndex>;
    fn outgoing(&self, index: NodeIndex) -> impl Iterator<Item = NodeIndex>;
    fn nodes(&self) -> usize;
    fn edges(&self) -> usize;
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
        for neighbor in graph.outgoing(current) {
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

    fn outgoing(&self, index: NodeIndex) -> impl Iterator<Item = NodeIndex> {
        self[index].iter().copied()
    }
    fn incoming(&self, index: NodeIndex) -> impl Iterator<Item = NodeIndex> {
        self.iter()
            .enumerate()
            .filter(move |(_, v)| v.contains(&index))
            .map(|(i, _)| i)
    }

    fn nodes(&self) -> usize {
        let set = self.iter().flat_map(|v| v.iter()).collect::<HashSet<_>>();
        set.len()
    }

    fn edges(&self) -> usize {
        self.iter().map(|v| v.len()).sum()
    }
}

pub struct SpecialGraph<T> {
    adj_matrix: HashMap<NodeIndex, HashSet<NodeIndex>>,
    edges: HashMap<(NodeIndex, NodeIndex), T>,
}
impl<T> SpecialGraph<T> {
    pub fn remove_node(&mut self, index: NodeIndex) -> bool {
        let incoming = self.incoming(index).collect::<Vec<_>>();
        let outgoing = self.outgoing(index).collect::<Vec<_>>();
        for incom in incoming {
            let x = self.edges.remove(&(incom, index));
            debug_assert!(x.is_some());
            let x = self.adj_matrix.get_mut(&incom).unwrap().remove(&index);
            debug_assert!(x);
        }
        for out in outgoing {
            let x = self.edges.remove(&(index, out));
            debug_assert!(x.is_some());
        }
        self.adj_matrix.remove(&index).is_some()
    }
    pub fn remove_edge(&mut self, from: NodeIndex, to: NodeIndex) -> bool {
        if self.edges.remove(&(from, to)).is_some() {
            let x = self.adj_matrix.get_mut(&from).unwrap().remove(&to);
            debug_assert!(x);
            true
        } else {
            debug_assert!(if let Some(x) = self.adj_matrix.get(&from) {
                !x.contains(&to)
            } else {
                true
            });
            false
        }
    }
}
impl<T> Graph for SpecialGraph<T> {
    fn new() -> Self {
        Self {
            adj_matrix: HashMap::new(),
            edges: HashMap::new(),
        }
    }
    fn outgoing(&self, index: NodeIndex) -> impl Iterator<Item = NodeIndex> {
        self.adj_matrix
            .get(&index)
            .map(|x| x.iter().copied())
            .into_iter()
            .flatten()
    }

    fn incoming(&self, index: NodeIndex) -> impl Iterator<Item = NodeIndex> {
        self.adj_matrix
            .iter()
            .filter(move |(_, v)| v.contains(&index))
            .map(|(i, _)| i)
            .cloned()
    }

    fn nodes(&self) -> usize {
        let set = self
            .adj_matrix
            .iter()
            .flat_map(|(_, v)| v.iter())
            .collect::<HashSet<_>>();
        set.len()
    }

    fn edges(&self) -> usize {
        self.edges.len()
    }
}
impl<T> GraphWithWeights<T> for SpecialGraph<T>
where
    T: Copy,
{
    fn weight(&self, from: NodeIndex, to: NodeIndex) -> Option<T> {
        self.edges.get(&(from, to)).copied()
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
