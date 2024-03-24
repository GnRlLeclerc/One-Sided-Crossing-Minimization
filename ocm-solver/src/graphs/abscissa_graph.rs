use ocm_parser::{
    bipartite_graph::BipartiteGraph,
    graph_base::{Edge, OrderedGraph},
};

use crate::algo_utils::{rank_index_array, sorted_index_array};

/// Abscissa-based graph data structure, where each node has an abscissa attributed at construction.
/// By convention, we space all vertices evenly among the top and bottom, such that the extremal vertices
/// in either the top or bottom layer, depending on which one has the most vertices, fall right on -1 and +1.
#[derive(Debug, Default, Clone)]
pub struct AbscissaGraph {
    /// Abscissas for the top nodes. Their indices start from 0
    pub top_nodes_abscissas: Vec<f64>,
    /// Abscissas for the bottom nodes. Their indices start from 0
    pub bottom_nodes_abscissas: Vec<f64>,

    /// Edges between the top and bottom nodes
    pub edges: Vec<Edge>,
}

impl AbscissaGraph {
    /// Create a new empty graph
    pub fn new() -> Self {
        Self::default()
    }

    /// Rebalance and symmetrize the graph abscissas for clean display.
    /// Call this method after updating vertex abscissas using a barycentric or median heuristic method.
    pub fn rebalance_abscissas(&mut self) {
        // Compute the absissas
        let max_row_node_count = self
            .bottom_nodes_abscissas
            .len()
            .max(self.top_nodes_abscissas.len());
        // Scale the nodes into [-1, 1] by multiplying by a scaling value and adding a negative offset
        let scale = 2_f64 / max_row_node_count as f64;

        // Indices are in [0, n[, will be brought to [0, m] by scaling, and must have m/2 substracted
        let top_offset = -(self.top_nodes_abscissas.len() as f64) * scale * 0.5_f64;
        let bottom_offset = -(self.bottom_nodes_abscissas.len() as f64) * scale * 0.5_f64;

        // Compute the top node indices
        let indices = sorted_index_array(&self.top_nodes_abscissas);

        // Iterate over the top nodes and update their abscissas
        for (index, &old_index) in indices.iter().enumerate() {
            self.top_nodes_abscissas[old_index] = index as f64 * scale + top_offset;
        }

        // Compute the bottom node indices
        let indices = sorted_index_array(&self.bottom_nodes_abscissas);

        // Iterate over the bottom nodes and update their abscissas
        for (index, &old_index) in indices.iter().enumerate() {
            self.bottom_nodes_abscissas[old_index] = index as f64 * scale + bottom_offset;
        }
    }
}

/// Implement the conversion from a bipartite graph reference (does not consume the original one)
impl From<&BipartiteGraph> for AbscissaGraph {
    fn from(origin: &BipartiteGraph) -> Self {
        let mut graph = AbscissaGraph::new();

        // Clone the edges and reset their respective indices at 0
        let top_count = origin.top_node_count;
        graph.edges = origin
            .edges
            .iter()
            .map(|(top_index, bottom_index)| (top_index - 1, bottom_index - 1 - top_count))
            .collect();

        // Compute the absissas
        let max_row_node_count = origin.bottom_node_count.max(origin.top_node_count);
        // Scale the nodes into [-1, 1] by multiplying by a scaling value and adding a negative offset
        let scale = 2_f64 / max_row_node_count as f64;

        // Indices are in [0, n[, will be brought to [0, m] by scaling, and must have m/2 substracted
        let top_offset = -(origin.top_node_count as f64) * scale * 0.5_f64;
        let bottom_offset = -(origin.bottom_node_count as f64) * scale * 0.5_f64;

        // Fill the nodes. Keep in mind that the edges assume that the indices start from 1.
        for index in 0..origin.top_node_count {
            graph
                .top_nodes_abscissas
                .push(index as f64 * scale + top_offset);
        }
        for index in 0..origin.bottom_node_count {
            graph
                .bottom_nodes_abscissas
                .push(index as f64 * scale + bottom_offset);
        }

        graph
    }
}

impl From<&AbscissaGraph> for BipartiteGraph {
    fn from(origin: &AbscissaGraph) -> Self {
        let mut graph = BipartiteGraph::new();

        // The BipartiteGraph lists its node indices starting from 1, left to right.
        // The AbscissaGraph node abscissas are not in order, we need to compute their sorted indices and update the edges
        let top_indices = sorted_index_array(&origin.top_nodes_abscissas);
        let bottom_indices = sorted_index_array(&origin.bottom_nodes_abscissas);

        // Clone the edges and reset their indices back to the BipartiteGraph format
        // (top ones start at 1, bottom ones start at top_count + 1)
        let top_count = origin.top_nodes_abscissas.len();
        graph.edges = origin
            .edges
            .iter()
            .map(|(top_index, bottom_index)| {
                (
                    (top_indices[*top_index as usize] + 1) as u64,
                    (bottom_indices[*bottom_index as usize] + 1 + top_count) as u64,
                )
            })
            .collect();

        // Fill the nodes. Keep in mind that the edges assume that the indices start from 1.
        graph.top_node_count = origin.top_nodes_abscissas.len() as u64;
        graph.bottom_node_count = origin.bottom_nodes_abscissas.len() as u64;

        graph
    }
}

impl OrderedGraph for AbscissaGraph {
    fn get_ordered_edges(&self) -> Vec<Edge> {
        // The AbscissaGraph's nodes are not ordered by their indices.
        // We must rebuild the edges with the correct indices.
        let top_indices = rank_index_array(&self.top_nodes_abscissas);
        let bottom_indices = rank_index_array(&self.bottom_nodes_abscissas);

        self.edges
            .iter()
            .map(|(top_index, bottom_index)| {
                (
                    (top_indices[*top_index as usize]) as u64,
                    (bottom_indices[*bottom_index as usize]) as u64,
                )
            })
            .collect()
    }
}
