//! Basic bipartite graph definition, to be parsed from the dataset files

use std::io::Write;
/// Bipartite Graph data structure, as parsed from the dataset files
#[derive(Debug, Default, Clone)]
pub struct BipartiteGraph {
    /// Number of top nodes. Their indices start from 1
    pub top_node_count: i64,
    /// Number of bottom nodes. Their indices start from `top_node_count + 1`
    pub bottom_node_count: i64,

    /// Edges between the top and bottom nodes
    pub edges: Vec<(i64, i64)>,
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

        Ok(())
    }
}
