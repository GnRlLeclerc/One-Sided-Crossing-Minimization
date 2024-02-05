//! Basic bipartite graph definition, to be parsed from the dataset files

/// Bipartite Graph data structure, as parsedd from the dataset files
#[derive(Debug, Default, Clone)]
pub struct BipartiteGraph {
    /// Number of top nodes. Their indices start from 1
    pub top_node_count: i64,
    /// Number of bottom nodes. Their indices start from `top_node_count`
    pub bottom_node_count: i64,

    /// Edges between the top and bottom nodes
    pub edges: Vec<(i64, i64)>,
}
