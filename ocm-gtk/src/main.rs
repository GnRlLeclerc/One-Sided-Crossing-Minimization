use std::rc::Rc;
use std::time::Instant;

use crate::gtk_utils::plot_in_window;
use clap::Parser;
use ocm_parser::parse_file;

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
}

const APP_ID: &str = "gitlab.binets.fr.gui-ocm-problem-solver";

fn main() {
    let args = Args::parse();

    let start_time = Instant::now();

    println!("Reading graph from file {}", args.source);

    let graph = parse_file(&args.source);

    println!("Graph read from file: {:?}", graph);

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
    let graph_rc = Rc::new(graph);
    plot_in_window(APP_ID, graph_rc);
}
