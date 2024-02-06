use ocm_parser::bipartite_graph::BipartiteGraph;
use plottable::Plottable;
use plotters::prelude::*;
use plotters::{backend::DrawingBackend, coord::Shift, drawing::DrawingArea};

pub mod plottable;

/// Example implementation for Bipartite Graph.
impl<'a, DB> Plottable<DB> for BipartiteGraph
where
    DB: DrawingBackend + 'a,
{
    fn plot(&self, root: &mut DrawingArea<DB, Shift>) {
        root.fill(&WHITE).unwrap();

        let top_center_offset = -self.top_node_count as f64 * 0.5;
        let bottom_center_offset = -self.bottom_node_count as f64 * 0.5;

        // Y position for the nodes
        let top_height = 1.0;
        let bottom_height = -1.0;
        // X normalization
        let horizontal_normalization =
            self.bottom_node_count.max(self.top_node_count) as f64 * 0.5f64;

        // Try dusplaying them, now (see if size scales well.)
        // Create the scatter plot context
        let mut scatter_ctx = ChartBuilder::on(root)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(-2f64..2f64, -2f64..2f64)
            .expect("Unable to build the scatter plot context");

        scatter_ctx
            .draw_series((0..self.top_node_count).map(|i| {
                Circle::new(
                    (
                        (i as f64 + top_center_offset) / horizontal_normalization,
                        top_height,
                    ),
                    8,
                    BLUE.filled(),
                )
            }))
            .expect("Unable to draw the top nodes");

        // Plot the bottom nodes
        scatter_ctx
            .draw_series((0..self.bottom_node_count).map(|i| {
                Circle::new(
                    (
                        (i as f64 + bottom_center_offset) / horizontal_normalization,
                        bottom_height,
                    ),
                    8,
                    RED.filled(),
                )
            }))
            .expect("Unable to draw the bottom nodes");

        // Draw the lines between the nodes
        scatter_ctx
            .draw_series(self.edges.iter().map(|(i1, i2)| {
                PathElement::new(
                    vec![
                        (
                            (*i1 as f64 - 1f64 + top_center_offset) / horizontal_normalization,
                            top_height,
                        ),
                        (
                            ((*i2 - self.top_node_count) as f64 - 1f64 + bottom_center_offset)
                                / horizontal_normalization,
                            bottom_height,
                        ),
                    ],
                    BLACK,
                )
            }))
            .expect("Unable to draw the edges");
    }
}
