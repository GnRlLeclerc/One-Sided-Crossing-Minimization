//! Some utility functions for algorithms

use std::ops::Add;

/// Given a sorted array, returns its median if it exists.
///
/// Returns
/// -------
/// - `None` if the array is empty
/// - `Some(median)` if the array has an odd number of elements
/// - `Some((mid1 + mid2) / 2)` if the array has an even number of elements
pub fn sorted_array_median<T: Add<Output = T> + std::ops::Div<Output = T> + From<i32> + Copy>(
    array: &[T],
) -> Option<T> {
    let len = array.len();

    match len {
        0 => None,
        _ if len % 2 == 1 => Some(array[len / 2]),
        _ => {
            let mid1 = array[len / 2 - 1];
            let mid2 = array[len / 2];
            Some((mid1 + mid2) / T::from(2))
        }
    }
}

/// Given an array of values implementing the `PartialOrd` trait, returns an array of indices where
/// the 1st index points to the smallest value, the 2nd index points to the 2nd smallest value, etc.
pub fn sorted_index_array<T: PartialOrd>(array: &[T]) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..array.len()).collect();
    indices.sort_by(|&i, &j| array[i].partial_cmp(&array[j]).unwrap());
    indices
}

/// Given an array of values implementing the `PartialOrd` trait, returns an array of ranks where
/// ranks[i] is the rank of the i-th element in the array. The rank 0 corresponds to the smallest element
pub fn rank_index_array<T: PartialOrd>(array: &[T]) -> Vec<usize> {
    let indices = sorted_index_array(array);

    let mut ranks = vec![0; array.len()];
    for (rank, index) in indices.iter().enumerate() {
        ranks[*index] = rank;
    }
    ranks
}

/// Sort a slice of edges in place in lexicographic order, i.e. first by the first node index, then by the second node index.
pub fn edges_lexicographic_sort(edges: &mut [(u64, u64)]) {
    edges.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
}

/// Sort a slice of edges in place in order of their minimum index present.
///
/// This sort is useful when dealing with a line sweep algorithm that needs to progressively add
/// active edges when iterating over a vertical line that goes through the top and bottom u64-indexed nodes
pub fn edges_min_index_sort(edges: &mut [(u64, u64)]) {
    edges.sort_by(|a, b| a.0.min(a.1).cmp(&b.0.min(b.1)));
}
