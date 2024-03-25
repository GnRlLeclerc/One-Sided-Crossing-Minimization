use std::time::Instant;

use clap::Parser;
use ocm_parser::{parse_file, run_output::RunOutput};
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

    /// Save analytics to a file
    #[arg(short, long)]
    analytics: bool,

    /// Plot the result to a file
    #[arg(short, long)]
    plot: bool,
}

fn main() {
    let args = Args::parse();

    if args.debug {
        println!("Reading graph from file {}", args.source);
    }

    let graph = parse_file(&args.source);

    if args.debug {
        println!("Graph read from file: {:?}", graph);
    }

    let mut initial_crossings = 0_u64;

    // Lazy evaluation
    if args.verbose || args.analytics {
        initial_crossings = line_sweep_crossings(&graph);
    }

    if args.verbose {
        println!("Crossings before: {}", initial_crossings);
        println!("Using algorithm: {:?}", args.algorithm);
    }

    let start_time = Instant::now();
    let graph = solve(&graph, &args.algorithm, args.verbose);
    let elapsed_time = start_time.elapsed();

    let mut final_crossings = 0_u64;

    // Lazy evaluation
    if args.verbose || args.analytics {
        final_crossings = line_sweep_crossings(&graph);
    }
    if args.verbose {
        println!("Crossings after: {}", final_crossings);
    }

    // Print elapsed time if the flag is set
    if args.verbose {
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

    if args.analytics {
        // Save the analytics to a file
        let parts: Vec<&str> = args.source.split('/').collect();

        let run_output = RunOutput::new(
            &args.source,
            &args.algorithm.to_string(),
            parts[parts.len() - 2],
            initial_crossings,
            final_crossings,
            elapsed_time.as_nanos() as u64,
        );
        run_output.save_to_file();
    }

    // Save the output to a file if the flag is set
    if let Some(output_file) = args.output_file {
        // Save the graph to a file
        graph.save_to_file(&output_file).unwrap();
    }
}
