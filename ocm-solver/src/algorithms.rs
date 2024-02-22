/*
Calcul sur l'espace dispo avec ~130 000 noeuds top & bot + ~260 000 arêtes.

stocker les voisins pour chaque node ~= 260 000 * 2 * 8 octets = 4 Mo, on est ultra larges dessus.

Il faut au moins que notre méthode avec la médiane fonctionne rapidement
260 000 fois une insertion dans un set à 260 000: 3 241 793 -> pas mal d'ops...
*/

use ordered_float::OrderedFloat;

use crate::{algo_utils::sorted_array_median, graphs::AbscissaGraph};

// TODO: watch time and space complexity for this one

/// Do one iteration of the median heuristic method on a graph
/// where all vertices have an abscissa.
pub fn median_heuristic_solve(graph: &mut AbscissaGraph) {
    // Create vectors to be updated
    let mut top_x = graph.top_nodes_abscissas.clone();
    let mut bottom_x = graph.bottom_nodes_abscissas.clone();

    // Store neighbors indices for each node in a vector.
    // Neighbors for each top node
    let mut top_neighbors: Vec<Vec<OrderedFloat<f64>>> =
        vec![vec![]; graph.top_nodes_abscissas.len()];
    // Neighbors for each bottom node
    let mut bottom_neighbors: Vec<Vec<OrderedFloat<f64>>> =
        vec![vec![]; graph.bottom_nodes_abscissas.len()];

    // For each node, store its neighbors' abscissas in a vector
    graph.edges.iter().for_each(|(top_index, bottom_index)| {
        top_neighbors[*top_index as usize]
            .push(graph.bottom_nodes_abscissas[*bottom_index as usize].into());
        bottom_neighbors[*bottom_index as usize]
            .push(graph.top_nodes_abscissas[*top_index as usize].into());
    });

    // For each top node, sort its neighbors' abscissas and get the median
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
