//! Basic bipartite graph definition, to be parsed from the dataset files

use std::io::Write;

use crate::graph_base::{Edge, OrderedGraph};
/// Bipartite Graph data structure, as parsed from the dataset files
#[derive(Debug, Default, Clone)]
pub struct BipartiteGraph {
    /// Number of top nodes. Their indices start from 1
    pub top_node_count: u64,
    /// Number of bottom nodes. Their indices start from `top_node_count + 1`
    pub bottom_node_count: u64,

    /// Edges between the top and bottom nodes
    pub edges: Vec<Edge>,
}

impl BipartiteGraph {
    /// Create a new empty graph
    pub fn new() -> Self {
        Self::default()
    }

    pub fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let mut file = std::fs::File::create(filename)?;
        let mut writer = std::io::BufWriter::new(&mut file);

        // Write the header
        writeln!(
            writer,
            "p ocr {} {} {}",
            self.top_node_count,
            self.bottom_node_count,
            self.edges.len()
        )?;

        // Write the edges
        for (top, bottom) in &self.edges {
            writeln!(writer, "{} {}", top, bottom)?;
        }

        println!("Graph saved to file {}", filename);

        Ok(())
    }
}

impl OrderedGraph for BipartiteGraph {
    fn get_ordered_edges(&self) -> Vec<(u64, u64)> {
        // A BipartiteGraph already has its nodes ordered by their indices.
        // However, top nodes are indexed from 1, and bottom nodes are indexed from top_node_count + 1.
        // This is absolutely not optimal when using the line sweep algorithm,
        // so we need to reset the indices to start from 0 for both top and bottom nodes.
        self.edges
            .iter()
            .map(|(top, bottom)| (top - 1, bottom - 1 - self.top_node_count))
            .collect()
    }
}
