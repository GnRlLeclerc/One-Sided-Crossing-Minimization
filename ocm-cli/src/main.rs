use std::time::Instant;

use clap::Parser;
use ocm_parser::{bipartite_graph::BipartiteGraph, parse_file};
use ocm_plotter::plottable::plot_to_file;
use ocm_solver::{algorithms::median_heuristic_solve, graphs::AbscissaGraph};

#[derive(Parser, Debug)]
#[command(author="Thibaut de Saivre, Thomas Fourier", version, about="Solver for the OCM problem", long_about = None)]
struct Args {
    /// Graph source file
    #[arg()]
    source: String,

    /// Measure execution time
    #[arg(short, long)]
    time: bool,

    /// Display debug information
    #[arg(short, long)]
    debug: bool,

    /// Save the output to a file
    #[arg(short, long)]
    output_file: Option<String>,

    /// Plot the result to a file
    #[arg(short, long)]
    plot: bool,
}

fn main() {
    let args = Args::parse();

    let start_time = Instant::now();

    if args.debug {
        println!("Reading graph from file {}", args.source);
    }

    let graph = parse_file(&args.source);

    if args.debug {
        println!("Graph read from file: {:#?}", graph);
    }

    // Do the median computation
    let mut graph: AbscissaGraph = (&graph).into(); // Convert the input graph into a graph with abscissas
    median_heuristic_solve(&mut graph);
    graph.rebalance_abscissas();

    // Measure the elapsed time
    let elapsed_time = start_time.elapsed();

    // Print elapsed time if the flag is set
    if args.time {
        // Print the elapsed time in seconds and milliseconds
        println!(
            "Elapsed time: {}.{} seconds",
            elapsed_time.as_secs(),
            elapsed_time.subsec_nanos()
        );
    }

    if args.plot {
        // Save the resulting image to a file
        plot_to_file(&graph, "graph.png");
    }

    // Save the output to a file if the flag is set
    if let Some(output_file) = args.output_file {
        // Save the graph to a file
        let graph: BipartiteGraph = (&graph).into();
        graph.save_to_file(&output_file).unwrap();
    }
}
