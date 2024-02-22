//! Other graph representations that may be better suited for problem solving.

use ocm_parser::bipartite_graph::BipartiteGraph;

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
    pub edges: Vec<(i64, i64)>,
}

impl AbscissaGraph {
    /// Create a new empty graph
    pub fn new() -> Self {
        Self::default()
    }

    /// Rebalance and symmetrize the graph abscissas for clean display.
    /// Call this method after updating vertex abscissas using a barycentric or median heuristic method.
    pub fn rebalance_abscissas(&mut self) {
        todo!()
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
        let top_offset = -origin.top_node_count as f64 * scale * 0.5_f64;
        let bottom_offset = -origin.bottom_node_count as f64 * scale * 0.5_f64;

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
