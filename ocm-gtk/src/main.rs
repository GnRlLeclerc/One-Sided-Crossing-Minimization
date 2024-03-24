use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use crate::gtk_utils::plot_in_window;
use clap::Parser;
use ocm_parser::bipartite_graph::BipartiteGraph;
use ocm_parser::parse_file;
use ocm_solver::algorithms::{solve, Algorithm};
use ocm_solver::crossings::line_sweep_crossings;

mod gtk_utils;
mod plotter_widget;

#[derive(Parser, Debug)]
#[command(author="Thibaut de Saivre", version, about="GUI solver for the OCM problem", long_about = None)]
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
}

const APP_ID: &str = "gitlab.binets.fr.gui-ocm-problem-solver";

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

    // Display the first graph in a window
    let graph_rc = Rc::new(RefCell::new(graph));
    plot_in_window(APP_ID, graph_rc.clone());

    if args.verbose {
        println!(
            "Crossings before: {}",
            line_sweep_crossings::<BipartiteGraph>(&graph_rc.borrow())
        );
        println!("Using algorithm: {:?}", args.algorithm);
    }

    let graph = solve(&graph_rc.borrow(), &args.algorithm, args.verbose);

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

    // Display the result again
    let graph_rc = Rc::new(RefCell::new(graph));
    plot_in_window(APP_ID, graph_rc.clone());

    if args.output_file.is_some() {
        // Save the resulting graph to a file
        let filename = args.output_file.unwrap();
        graph_rc.borrow().save_to_file(&filename).unwrap();
    }
}
