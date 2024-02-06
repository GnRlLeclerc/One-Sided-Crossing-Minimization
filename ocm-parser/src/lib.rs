use std::io::BufRead;

use bipartite_graph::BipartiteGraph;
use parser::{parse_graph_edges, parse_graph_header};

pub mod bipartite_graph;
pub mod parser;

/// Parse a graph from a source file
pub fn parse_file(filename: &str) -> BipartiteGraph {
    // Read the file line by line
    let file = std::fs::File::open(filename).unwrap();

    let reader = std::io::BufReader::new(file);
    let mut lines = reader.lines();

    let mut graph = BipartiteGraph::new(); // Create an empty graph

    // Parse the header line
    let header_line = lines.next().unwrap().unwrap();
    let mut header_str = header_line.as_str();
    let (top_count, bot_count, edge_count) = parse_graph_header(&mut header_str).unwrap();
    graph.top_node_count = top_count;
    graph.bottom_node_count = bot_count;

    // Parse the edges
    for _ in 0..edge_count {
        let edge_line = lines.next().unwrap().unwrap();
        let mut edge_str = edge_line.as_str();

        let (top, bot) = parse_graph_edges(&mut edge_str).unwrap();
        graph.edges.push((top, bot));
    }

    graph
}
