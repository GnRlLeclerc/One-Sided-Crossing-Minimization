use std::time::Instant;

use clap::Parser;
use ocm_parser::parse_file;
use ocm_plotter::plottable::plot_to_file;
use ocm_solver::{
    algorithms::{solve, Algorithm},
    crossings::line_sweep_crossings,
};

#[derive(Parser, Debug)]
#[command(author="Thibaut de Saivre", version, about="Solver for the OCM problem", long_about = None)]
struct Args {
    /// Graph source file
    #[arg()]
    source: String,

    /// Display debug information
    #[arg(short, long)]
    debug: bool,

    /// Display progression
    #[arg(short, long)]
    verbose: bool,

    /// Algorithm to use
    #[arg(short, long, value_enum)]
    algorithm: Algorithm,

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
        println!("Graph read from file: {:?}", graph);
    }

    if args.verbose {
        println!("Crossings before: {}", line_sweep_crossings(&graph));
        println!("Using algorithm: {:?}", args.algorithm);
    }

    let graph = solve(&graph, &args.algorithm, args.verbose);

    if args.verbose {
        println!("Crossings after: {}", line_sweep_crossings(&graph));
    }

    // Print elapsed time if the flag is set
    if args.verbose {
        // Measure the elapsed time
        let elapsed_time = start_time.elapsed();

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
        graph.save_to_file(&output_file).unwrap();
    }
}
