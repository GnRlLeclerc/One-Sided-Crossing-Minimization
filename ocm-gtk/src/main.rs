use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use crate::gtk_utils::plot_in_window;
use clap::Parser;
use ocm_parser::bipartite_graph::BipartiteGraph;
use ocm_parser::parse_file;
use ocm_solver::algorithms::median_heuristic::median_heuristic_solve;
use ocm_solver::graphs::abscissa_graph::AbscissaGraph;

mod gtk_utils;
mod plotter_widget;

#[derive(Parser, Debug)]
#[command(author="Thibaut de Saivre, Thomas Fourier", version, about="GUI solver for the OCM problem", long_about = None)]
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
}

const APP_ID: &str = "gitlab.binets.fr.gui-ocm-problem-solver";

fn main() {
    let args = Args::parse();

    let start_time = Instant::now();

    if args.debug {
        println!("Reading graph from file {}", args.source);
    }

    let graph = parse_file(&args.source);
    let graph: AbscissaGraph = (&graph).into(); // Convert the input graph into a graph with abscissas

    if args.debug {
        println!("Graph read from file: {:#?}", graph);
    }

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

    // Display the result in a new window
    let graph_rc = Rc::new(RefCell::new(graph));
    plot_in_window(APP_ID, graph_rc.clone());

    // Do one iteration of the median heuristic (without rebalancing)
    median_heuristic_solve(&mut graph_rc.borrow_mut());

    // Rebalance the graph abscissas before display
    graph_rc.borrow_mut().rebalance_abscissas();

    // Display the result again
    plot_in_window(APP_ID, graph_rc.clone());

    if args.output_file.is_some() {
        // Save the resulting graph to a file
        let filename = args.output_file.unwrap();
        let new_graph: BipartiteGraph = (&*graph_rc.borrow()).into();
        new_graph.save_to_file(&filename).unwrap();
    }
}
