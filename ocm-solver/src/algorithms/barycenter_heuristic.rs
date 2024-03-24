use ordered_float::OrderedFloat;

use crate::{crossings::line_sweep_crossings, graphs::abscissa_graph::AbscissaGraph};

/// Do one in-place iteration of the barycenter heuristic method on a graph
/// where all vertices have an abscissa.
///
/// Algorithm
/// ---------
/// 1. For each node (top and bottom), set the new abscissa to the mean of its neighbors' abscissas.
///    * If the node has no neighbors, keep its abscissa.
///
/// Note: the abscissas must have to be rebalanced before displaying the graph again in order to have a pretty display.
///
/// Complexity
/// ----------
/// * Time: `O(V + E)`
/// * Space: `O(V + E)`
pub fn barycenter_heuristic_solve(graph: &mut AbscissaGraph) {
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

    // For each top node, compute the mean of its neighbors' abscissas
    // Time complexity: O(V)
    top_neighbors
        .iter_mut()
        .enumerate()
        .for_each(|(index, neighbors)| {
            if !neighbors.is_empty() {
                top_x[index] =
                    neighbors.iter().map(|x| x.into_inner()).sum::<f64>() / neighbors.len() as f64;
            }
        });

    // For each bottom node, compute the mean of its neighbors' abscissas
    // Time complexity: O(V)
    bottom_neighbors
        .iter_mut()
        .enumerate()
        .for_each(|(index, neighbors)| {
            if !neighbors.is_empty() {
                bottom_x[index] =
                    neighbors.iter().map(|x| x.into_inner()).sum::<f64>() / neighbors.len() as f64;
            }
        });

    // Swap vectors
    graph.top_nodes_abscissas = top_x;
    graph.bottom_nodes_abscissas = bottom_x;
}

/// Do multiple in-place iterations of the barycenter heuristic method on a graph
/// where all vertices have an abscissa. Count the crossings before each iteration,
/// and stop when the crossing count stops decreasing.
///
/// Algorithm
/// ---------
/// 1. Count the crossings in the graph.
///
/// 2. For each node (top and bottom), set the new abscissa to the mean of its neighbors' abscissas.
///    * If the node has no neighbors, keep its abscissa.
///
/// 3. Rebalance the graph node positions, and start again.
///
/// Note: the abscissas must have to be rebalanced before displaying the graph again in order to have a pretty display.
///
/// Complexity
/// ----------
/// * Time: Depends on the number of iterations.
/// * Space: `O(V + E)`
pub fn iterated_barycenter_heuristic_solve(graph: &mut AbscissaGraph, verbose: bool) {
    let mut new_crossings = line_sweep_crossings(graph);
    let mut crossings = new_crossings + 1;
    let mut iteration = 0;
    let mut previous_graph = graph.clone();

    while new_crossings < crossings {
        previous_graph = graph.clone(); // Save the previous graph (if the last iteration is not fruitful). This may be expensive

        barycenter_heuristic_solve(graph);
        graph.rebalance_abscissas(); // Rebalance the node positions, because we use means

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
