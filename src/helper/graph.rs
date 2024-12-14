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
pub trait Graph {
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

impl Graph for &[Vec<NodeIndex>] {
    fn outgoing(&self, index: NodeIndex) -> impl Iterator<Item = &NodeIndex> {
        self[index].iter()
    }
}
