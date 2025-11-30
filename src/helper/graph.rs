use core::{fmt::Write as _, ops::Add};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use super::grid::{Grid, index::GridIndex as _};

/// `curr_to_neighbor_comparison` is a function that
///  has as arguments the current node and the neighbor node of the grid
/// and has to determine if these nodes have a edge in the graph.
pub fn build_graph4<T, G, F>(grid: &G, curr_to_neighbor_comparison: F) -> Vec<Vec<NodeIndex>>
where
    T: Clone,
    G: Grid<T>,
    F: Fn(&T, &T) -> bool,
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
/// and has to determine if these nodes have a edge in the graph and the weight.
pub fn build_graph4_special<T, W, G, F>(grid: &G, curr_to_neighbor_comparison: F) -> Special<W>
where
    T: Clone,
    W: Copy,
    G: Grid<T>,
    F: Fn(&T, &T) -> Option<W> + Copy,
{
    Special::from_edges(grid.all_indices().flat_map(|from| {
        grid.get_neigbors4(from)
            .filter_map(move |(to_ind, to_val)| {
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
/// and has to determine if these nodes have a edge in the graph.
pub fn build_graph8<T, G, F>(grid: &G, curr_to_neighbor_comparison: F) -> Vec<Vec<NodeIndex>>
where
    T: Clone,
    G: Grid<T>,
    F: Fn(&T, &T) -> bool,
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
pub trait WithoutWeights
where
    Self: Graph + Sized,
{
    fn add_edge(&mut self, from: NodeIndex, to: NodeIndex);
    fn from_edges<I>(it: I) -> Self
    where
        I: Iterator<Item = (NodeIndex, NodeIndex)>,
    {
        let mut g = Self::new();
        it.for_each(|(from, to)| g.add_edge(from, to));
        g
    }
}
pub trait WithWeights<T>
where
    Self: Graph + Sized,
    T: Copy,
{
    fn weight(&self, from: NodeIndex, to: NodeIndex) -> Option<T>;
    fn add_edge(&mut self, from: NodeIndex, to: NodeIndex, weight: T);
    fn from_edges<I>(it: I) -> Self
    where
        I: Iterator<Item = (NodeIndex, NodeIndex, T)>,
    {
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
            if let Some(target) = end
                && position == target
            {
                break;
            }

            for v in self.outgoing(position) {
                let u = position;
                let weight = self.weight(u, v).unwrap();
                let next = State {
                    cost: cost + weight,
                    position: v,
                };

                if let Some(&current_cost) = dist.get(&v) {
                    if next.cost < current_cost {
                        dist.insert(v, next.cost);
                        heap.push(next);
                    }
                } else {
                    dist.insert(v, next.cost);
                    heap.push(next);
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

                if let Some(&current_cost) = dist.get(&v) {
                    if next.cost < current_cost {
                        dist.insert(v, next.cost);
                        heap.push(next);

                        // Update the path to this node
                        let mut new_path = paths[&u].clone();
                        new_path.push(v);
                        paths.insert(v, new_path);
                    }
                } else {
                    dist.insert(v, next.cost);
                    heap.push(next);

                    // Create the path to this node
                    let mut new_path = paths[&u].clone();
                    new_path.push(v);
                    paths.insert(v, new_path);
                }
            }
        }

        // Return the distance map and the shortest path to the target node
        (
            dist,
            paths.get(&end).cloned().unwrap_or_else(Vec::new), // Return an empty path if unreachable
        )
    }
    fn all_pairs_shortest_distances(&self) -> HashMap<(NodeIndex, NodeIndex), T>
    where
        T: Copy + Add<Output = T> + Default + Ord,
    {
        let mut result = HashMap::new();

        for u in self.nodes() {
            let dist = self.dijkstra_distances(u, None);
            for (v, d) in dist {
                if u != v {
                    result.insert((u, v), d);
                }
            }
        }

        result
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
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl<T> PartialOrd for State<T>
where
    T: PartialOrd + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
pub trait Graph {
    fn new() -> Self;
    fn incoming(&self, index: NodeIndex) -> impl Iterator<Item = NodeIndex>;
    fn outgoing(&self, index: NodeIndex) -> impl Iterator<Item = NodeIndex>;
    fn nodes_count(&self) -> usize;
    fn nodes(&self) -> impl Iterator<Item = NodeIndex>;
    fn edges_count(&self) -> usize;
    fn remove_edge(&mut self, from: NodeIndex, to: NodeIndex) -> bool;
    fn connected_components(&self) -> Vec<HashSet<NodeIndex>> {
        let mut visited = HashSet::new();
        let mut components = Vec::new();

        for start in self.nodes() {
            if visited.contains(&start) {
                continue;
            }

            let mut component = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back(start);
            visited.insert(start);

            while let Some(node) = queue.pop_front() {
                component.insert(node);
                for neigh in self.outgoing(node).chain(self.incoming(node)) {
                    if visited.insert(neigh) {
                        queue.push_back(neigh);
                    }
                }
            }

            components.push(component);
        }

        components
    }
    fn all_paths(&self, start: NodeIndex, end: NodeIndex) -> Vec<Vec<NodeIndex>>
    where
        Self: Sized,
    {
        let mut result = Vec::new();
        let mut path = Vec::new();

        dfs(self, start, end, &mut path, &mut result);
        result
    }

    fn to_dot<F, G>(
        &self,
        undirected: bool,
        node_label_fn: Option<F>,
        edge_label_fn: Option<G>,
    ) -> String
    where
        F: Fn(usize) -> String,
        G: Fn(usize, usize) -> String,
    {
        let mut dot = if undirected {
            String::from("graph G {\n")
        } else {
            String::from("digraph G {\n")
        };
        // Add nodes with labels
        for node in self.nodes() {
            let label = node_label_fn
                .as_ref()
                .map_or_else(|| node.to_string(), |f| f(node));
            writeln!(dot, "    {node} [label=\"{label}\"];").expect("cant fail");
        }

        // Add edges with labels
        let mut edges = HashSet::new();
        for node in self.nodes() {
            for neighbor in self.outgoing(node) {
                // Add edges only once (undirected graph)
                let edge = if undirected {
                    if node < neighbor {
                        (node, neighbor)
                    } else {
                        (neighbor, node)
                    }
                } else {
                    (node, neighbor)
                };
                if edges.insert(edge) {
                    let label = edge_label_fn
                        .as_ref()
                        .map_or_else(String::new, |f| f(edge.0, edge.1));
                    writeln!(
                        dot,
                        "    {} {} {} [label=\"{}\"];",
                        edge.0,
                        if undirected { "--" } else { "->" },
                        edge.1,
                        label
                    )
                    .expect("cant fail");
                }
            }
        }

        dot.push_str("}\n");
        dot
    }

    fn bron_kerbosch1(&self) -> Vec<HashSet<NodeIndex>>
    where
        Self: Sized,
    {
        // Recursive function
        fn bron_kerbosch1_recursive<G: Graph>(
            graph: &G,
            r: &HashSet<NodeIndex>,
            p: &mut HashSet<NodeIndex>,
            x: &mut HashSet<NodeIndex>,
            all_cliques: &mut Vec<HashSet<NodeIndex>>,
        ) {
            if p.is_empty() && x.is_empty() {
                // Report R as a maximal clique
                all_cliques.push(r.iter().copied().collect());
                return;
            }

            // Iterate over a copy of P to allow mutation of P during iteration
            for &v in &p.clone() {
                // R ⋃ {v}
                let mut new_r = r.clone();
                new_r.insert(v);

                // P ⋂ N(v)
                let mut new_p: HashSet<_> = graph
                    .outgoing(v)
                    .filter(|&neighbor| p.contains(&neighbor))
                    .collect();

                // X ⋂ N(v)
                let mut new_x: HashSet<_> = graph
                    .outgoing(v)
                    .filter(|&neighbor| x.contains(&neighbor))
                    .collect();

                // Recursive call
                bron_kerbosch1_recursive(graph, &new_r, &mut new_p, &mut new_x, all_cliques);

                // P := P \ {v}
                p.remove(&v);

                // X := X ⋃ {v}
                x.insert(v);
            }
        }
        let mut all_cliques = Vec::new();
        let r = HashSet::new(); // Current clique
        let mut p: HashSet<NodeIndex> = self.nodes().collect(); // Potential candidates
        let mut x = HashSet::new(); // Already processed nodes

        bron_kerbosch1_recursive(self, &r, &mut p, &mut x, &mut all_cliques);
        all_cliques
    }
    fn topologocal_order(&self) -> Vec<usize>
    where
        Self: Clone,
    {
        let mut graph = self.clone();
        let mut s = graph
            .nodes()
            .filter(|n| graph.incoming(*n).count() == 0)
            .collect::<Vec<_>>();
        let mut l = Vec::with_capacity(graph.nodes_count());
        while let Some(n) = s.pop() {
            l.push(n);
            let o = graph.outgoing(n).collect::<Vec<_>>();
            for m in o {
                graph.remove_edge(n, m);
                if graph.incoming(m).count() == 0 {
                    s.push(m);
                }
            }
        }
        assert!(graph.edges_count() == 0, "Graph has a cycle");
        l
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

impl WithoutWeights for Vec<Vec<NodeIndex>> {
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

    fn nodes_count(&self) -> usize {
        let set = self.iter().flat_map(|v| v.iter()).collect::<HashSet<_>>();
        set.len()
    }

    fn edges_count(&self) -> usize {
        self.iter().map(std::vec::Vec::len).sum()
    }

    fn nodes(&self) -> impl Iterator<Item = NodeIndex> {
        let set = self.iter().flat_map(|v| v.iter()).collect::<HashSet<_>>();
        let mut vec = set.into_iter().collect::<Vec<_>>();
        vec.sort();
        vec.into_iter().copied()
    }

    fn remove_edge(&mut self, from: NodeIndex, to: NodeIndex) -> bool {
        self.get_mut(from)
            .is_some_and(|v| match v.iter().enumerate().find(|(_, x)| **x == to) {
                Some((i, _)) => {
                    v.remove(i);
                    true
                }
                None => false,
            })
    }
}
#[derive(Debug, Clone)]
pub struct Special<T> {
    adj_matrix: HashMap<NodeIndex, HashSet<NodeIndex>>,
    edges: HashMap<(NodeIndex, NodeIndex), T>,
}
impl<T> Special<T> {
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
    pub fn delete_edge(&mut self, from: NodeIndex, to: NodeIndex) -> bool {
        if self.edges.remove(&(from, to)).is_some() {
            let x = self.adj_matrix.get_mut(&from).unwrap().remove(&to);
            debug_assert!(x);
            true
        } else {
            debug_assert!(self.adj_matrix.get(&from).is_none_or(|x| !x.contains(&to)));
            false
        }
    }
}
impl<T> Graph for Special<T> {
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
            .copied()
    }

    fn nodes_count(&self) -> usize {
        let set = self
            .adj_matrix
            .iter()
            .flat_map(|(_, v)| v.iter())
            .collect::<HashSet<_>>();
        set.len()
    }

    fn edges_count(&self) -> usize {
        self.edges.len()
    }

    fn nodes(&self) -> impl Iterator<Item = NodeIndex> {
        let set = self
            .adj_matrix
            .iter()
            .flat_map(|(_, v)| v.iter())
            .chain(self.adj_matrix.keys())
            .collect::<HashSet<_>>();
        let mut vec = set.into_iter().collect::<Vec<_>>();
        vec.sort();
        vec.into_iter().copied()
    }

    fn remove_edge(&mut self, from: NodeIndex, to: NodeIndex) -> bool {
        self.adj_matrix.entry(from).and_modify(|s| {
            s.remove(&to);
        });
        self.edges.remove(&(from, to)).is_some()
    }
}
impl<T> WithWeights<T> for Special<T>
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
            .or_insert_with(|| HashSet::from_iter([to]));
        self.edges.insert((from, to), weight);
    }
}
