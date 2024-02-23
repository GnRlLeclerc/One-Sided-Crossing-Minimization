//! Crossings computations for different graph structures in order to evaluate performance
//! All these functions assume that the node indices are in order, i.e. the first node has index 0, the second has index 1, etc.
//!
//! All HashMaps and HashSets use the [`ahash`](crate::ahash) for better performance (cryptographic security is not needed).

use crate::algo_utils::edges_min_index_sort;

use ahash::AHashSet;
use ocm_parser::graph_base::OrderedGraph;

/// Count the crossings in a graph using the line sweep algorithm.
///
/// Algorithm
/// ---------
/// 1. Sort edges by their minimum index = appearance order when sweeping horizontally.
/// 2. Iterate through the edges in appearance order:
///     1. Update the current sweep line position to the appearance index of the new edge (= min index).
///     2. Compare the new edge with all active edges to find crossings.
///     3. Add the new edge to the active edges.
///     4. Remove dead edges from the active edges (ie: edges with their maximum index <= sweep line position).
///
/// Complexity
/// ----------
/// * Time: `O((E + V) * E)` ? Not sure...
/// * Space: `O(E)`
///
pub fn line_sweep_crossings<T: OrderedGraph>(graph: &T) -> u64 {
    // Sort the edges using their minimum index in order to swipe through them in appearance order.
    // Time: O(E * log(E))
    // Space: O(E)
    let mut edges = graph.get_ordered_edges();
    edges_min_index_sort(&mut edges);

    // Store currently active edges
    // Space: O(E)
    let mut active_edges: AHashSet<(u64, u64)> = AHashSet::new();
    let mut line_position = 0_u64; // Current index swept by the line
    let mut crossings = 0_u64; // Total number of crossings found
    let mut line_position_changed; // Boolean flag

    // Iterate through the edges in appearance order
    // Time: O((E + V) * E) ? Not sure...
    for edge in &edges {
        let new_line_position = edge.0.min(edge.1); // Update the line position to the appearance index of the new edge
        line_position_changed = false;
        if new_line_position != line_position {
            line_position = new_line_position;
            line_position_changed = true;
        }

        // 1. Compare with active edges
        crossings += scan_edges_for_crossings(&active_edges, edge);

        // 2. Add to active edges
        active_edges.insert(*edge);

        // 3. Remove dead edges (ie: their max index is less or equal to the current line position)
        // Only do this if the line position has changed. We forget cleaning (n,n) vertical edges,
        // but we gain a lot more by avoiding to scan every older edge for every new edge on the same sweep line position.
        if line_position_changed {
            remove_dead_edges(&mut active_edges, line_position);
        }
    }

    crossings
}

// Helper functions

/// Given a vertical line position, remove all dead edges from a set of active edges.
/// A dead edge is an edge whose maximum index is less or equal to the line position.
fn remove_dead_edges(active_edges: &mut AHashSet<(u64, u64)>, line_position: u64) {
    active_edges.retain(|&(start, end)| start > line_position || end > line_position);
}

/// Given a set of active edges and a new edge, scan the active edges for crossings with the new edge.
fn scan_edges_for_crossings(active_edges: &AHashSet<(u64, u64)>, edge: &(u64, u64)) -> u64 {
    let mut crossings = 0_u64;

    for (start, end) in active_edges {
        // There is a crossing if there is an inversion in the product of the direction differences.
        if (*start as i64 - edge.0 as i64) * (*end as i64 - edge.1 as i64) < 0 {
            crossings += 1;
        }
    }

    crossings
}
