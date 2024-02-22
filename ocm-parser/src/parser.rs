//! Parsing utils for the graph files

use winnow::ascii::dec_uint;
use winnow::PResult;
use winnow::Parser;

/// Parse the header of a graph file
pub fn parse_graph_header(input: &mut &str) -> PResult<(u64, u64, u64)> {
    let (_, top_vertices_count, _, bottom_vertices_count, _, edge_count) =
        ("p ocr ", dec_uint, ' ', dec_uint, ' ', dec_uint).parse_next(input)?;
    Ok((top_vertices_count, bottom_vertices_count, edge_count))
}

/// Parse an edge from a line of the graph file
pub fn parse_graph_edges(input: &mut &str) -> PResult<(u64, u64)> {
    let (top_vertex_index, _, bottom_vertex_index) = (dec_uint, ' ', dec_uint).parse_next(input)?;

    Ok((top_vertex_index, bottom_vertex_index))
}
