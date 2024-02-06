use ocm_parser::bipartite_graph::BipartiteGraph;
use plottable::Plottable;
use plotters::prelude::*;
use plotters::{backend::DrawingBackend, coord::Shift, drawing::DrawingArea};

pub mod plottable;

/// Example implementation for Bipartite Graph.
/// TODO: provide different implementations, maybe different wrappr structs with custom parameters.
impl<DB> Plottable<DB> for BipartiteGraph
where
    DB: DrawingBackend,
{
    fn plot(&self, root: &mut DrawingArea<DB, Shift>) -> Result<(), Box<dyn std::error::Error>> {
        println!("TODO: implement this !");
        root.fill(&RED).unwrap();

        Ok(())
    }
}
