//! Base definitions for graphs
//!

pub type Edge = (u64, u64);

/// Trait for graphs that can return their edges with the following conventions:
/// - Edges refer to nodes by their indices, starting from 0.
/// - The first top node has an index of 0.
/// - The first bottom node has an index of 0.
/// - The nodes are ordered by their indices.
pub trait OrderedGraph {
    fn get_ordered_edges(&self) -> Vec<(u64, u64)>;
}
