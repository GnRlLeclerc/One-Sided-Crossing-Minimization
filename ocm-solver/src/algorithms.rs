use std::fmt::{Display, Formatter};

use ocm_parser::bipartite_graph::BipartiteGraph;

use crate::graphs::abscissa_graph::AbscissaGraph;

pub mod barycenter_heuristic;
pub mod median_heuristic;

/// Algorithm to use for the generic solver
#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Algorithm {
    Median,
    IterMedian,
    Barycenter,
    IterBarycenter,
}

/// Generic solve function for the generic bipartite graph input
/// Returns the solution bipartite graph.
pub fn solve(graph: &BipartiteGraph, algorithm: &Algorithm, verbose: bool) -> BipartiteGraph {
    match algorithm {
        Algorithm::Median => {
            let mut graph: AbscissaGraph = graph.into();
            median_heuristic::median_heuristic_solve(&mut graph);
            (&graph).into()
        }
        Algorithm::IterMedian => {
            let mut graph = graph.into();
            median_heuristic::iterated_median_heuristic_solve(&mut graph, verbose);
            (&graph).into()
        }
        Algorithm::Barycenter => {
            let mut graph = graph.into();
            barycenter_heuristic::barycenter_heuristic_solve(&mut graph);
            (&graph).into()
        }
        Algorithm::IterBarycenter => {
            let mut graph = graph.into();
            barycenter_heuristic::iterated_barycenter_heuristic_solve(&mut graph, verbose);
            (&graph).into()
        }
    }
}

/// Directory names for the analytics output for each algorithm
impl Display for Algorithm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Algorithm::Median => write!(f, "median"),
            Algorithm::IterMedian => write!(f, "iterated_median"),
            Algorithm::Barycenter => write!(f, "barycenter"),
            Algorithm::IterBarycenter => write!(f, "iterated_barycenter"),
        }
    }
}
