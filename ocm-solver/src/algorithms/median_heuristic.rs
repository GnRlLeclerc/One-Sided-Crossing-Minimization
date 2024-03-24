use ordered_float::OrderedFloat;

use crate::{
    algo_utils::sorted_array_median, crossings::line_sweep_crossings,
    graphs::abscissa_graph::AbscissaGraph,
};

/// Do one in-place iteration of the median heuristic method on a graph
/// where all vertices have an abscissa.
///
/// Algorithm
/// ---------
/// 1. For each node (top and bottom), set the new abscissa to the median of its neighbors' abscissas.
///    * If the node has no neighbors, keep its abscissa.
///    * If the node has an even number of neighbors, take the average of the two middle values.
///
/// Note: the abscissas must have to be rebalanced before displaying the graph again in order to have a pretty display.
///
/// Complexity
/// ----------
/// * Time: `O(V * log(V) + E)`
/// * Space: `O(V + E)`
pub fn median_heuristic_solve(graph: &mut AbscissaGraph) {
    // Create vectors to be updated
    // Space complexity: O(V)
    let mut top_x = graph.top_nodes_abscissas.clone();
    let mut bottom_x = graph.bottom_nodes_abscissas.clone();

    // Store neighbors indices for each node in a vector.
    // Neighbors for each top node
    // Space complexity: O(E) after filling
    let mut top_neighbors: Vec<Vec<OrderedFloat<f64>>> =
        vec![vec![]; graph.top_nodes_abscissas.len()];
    // Neighbors for each bottom node
    let mut bottom_neighbors: Vec<Vec<OrderedFloat<f64>>> =
        vec![vec![]; graph.bottom_nodes_abscissas.len()];

    // For each node, store its neighbors' abscissas in a vector
    // Time complexity: O(E)
    graph.edges.iter().for_each(|(top_index, bottom_index)| {
        top_neighbors[*top_index as usize]
            .push(graph.bottom_nodes_abscissas[*bottom_index as usize].into());
        bottom_neighbors[*bottom_index as usize]
            .push(graph.top_nodes_abscissas[*top_index as usize].into());
    });

    // For each top node, sort its neighbors' abscissas and get the median
    // Time complexity: O(V * log(V))
    top_neighbors
        .iter_mut()
        .enumerate()
        .for_each(|(index, neighbors)| {
            neighbors.sort_unstable();

            if let Some(median) = sorted_array_median(neighbors) {
                top_x[index] = median.into();
            }
        });

    // For each bottom node, sort its neighbors' abscissas and get the median
    // Time complexity: O(V * log(V))
    bottom_neighbors
        .iter_mut()
        .enumerate()
        .for_each(|(index, neighbors)| {
            neighbors.sort_unstable();

            if let Some(median) = sorted_array_median(neighbors) {
                bottom_x[index] = median.into();
            }
        });

    // Swap vectors
    graph.top_nodes_abscissas = top_x;
    graph.bottom_nodes_abscissas = bottom_x;
}

/// Do multiple in-place iterations of the median heuristic method on a graph
/// where all vertices have an abscissa. Count the crossings before each iteration,
/// and stop when the crossing count stops decreasing.
///
/// Algorithm
/// ---------
/// 1. Count the crossings in the graph.
///
/// 2. For each node (top and bottom), set the new abscissa to the median of its neighbors' abscissas.
///    * If the node has no neighbors, keep its abscissa.
///    * If the node has an even number of neighbors, take the average of the two middle values.
///
/// 3. Rebalance the graph node positions, and start again.
///
/// Note: the abscissas must have to be rebalanced before displaying the graph again in order to have a pretty display.
///
/// Complexity
/// ----------
/// * Time: Depends on the number of iterations.
/// * Space: `O(V + E)`
pub fn iterated_median_heuristic_solve(graph: &mut AbscissaGraph, verbose: bool) {
    let mut new_crossings = line_sweep_crossings(graph);
    let mut crossings = new_crossings + 1;
    let mut iteration = 0;
    let mut previous_graph = graph.clone();

    while new_crossings < crossings {
        previous_graph = graph.clone(); // Save the previous graph (if the last iteration is not fruitful). This may be expensive

        median_heuristic_solve(graph);
        graph.rebalance_abscissas(); // Rebalance the node positions, because we use medians

        // Swap and recompute crossings
        crossings = new_crossings;
        new_crossings = line_sweep_crossings(graph);

        if verbose {
            iteration += 1;
            println!("Iteration {}: {} crossings", iteration, new_crossings);
        }
    }

    // If the last iteration was not fruitful, revert to the previous graph
    if new_crossings > crossings {
        *graph = previous_graph;
    }
}
