use std::time::Instant;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

use clap::Parser;
use ocm_parser::parse_file;

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

    let application = gtk::Application::new(Some(APP_ID), Default::default());

    // Connect to "activate" signal of `application`
    application.connect_activate(build_ui);

    // Run with empty args
    application.run_with_args::<&str>(&[]);
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("GUI OCM Problem Solver")
        .build();

    // Present window
    window.present();
}
