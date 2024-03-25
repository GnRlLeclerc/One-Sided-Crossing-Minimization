# INF562 - OCM Problem

Author: Thibaut de Saivre

A solver for the One-sided crossing minimization problem.  
See [the PDF report here](./report/ocm-report.pdf).  
See some [python benchmarks here](./analytics.ipynb).

## Quickstart

### CLI

Run the CLI solver with the following command:

```bash
cargo run  --bin ocm-cli -- -a median -v datasets/tiny/complete_4_5.gr
```

### GTK

A GTK GUI is available. However, it requires GTK4. Installation instructions are available [here](https://gtk-rs.org/gtk4-rs/stable/latest/book/installation.html).
The [Rust book of GTK4](https://gtk-rs.org/gtk4-rs/stable/latest/book).

You can then check the gtk version on your machine:

```bash
pkg-config --modversion gtk4
```

Then run the GUI with:

```bash
cargo run --release --bin ocm-gtk -- -a median datasets/tiny/complete_4_5.gr
```

Run the CLI solver for large graphs and time it with:

```bash
cargo run --release --bin ocm-cli -- -a median -v datasets/large/25.gr
```

## Project structure

This project uses `cargo workspaces`.

```bash
├── datasets     # Graph datasets for testing
│   ├── large
│   ├── medium
│   └── tiny
├── ocm-cli      # CLI program for solving the OCM problem
├── ocm-gtk      # GTK GUI for solving the OCM problem
├── ocm-parser   # Graph dataset parser
├── ocm-plotter  # Plotting functions
├── ocm-solver   # Implementation logic
└── report       # LateX report
```

## Benchmarks

Various benchmarks are available using tests.

- `line_sweep_crossings` benchmarked over all datasets, in the [`crossings.rs`](ocm-solver/src/crossings.rs) file.

ISSUE: the benchmarks run very long for 2 of the large dataset files. You can interrupt the benchmark early, as the results are progressively written to the [`crossings_benchmark.csv`](./crossings_benchmark.csv) file.

Run them with the following command:

```bash
cargo test -- --nocapture # Do not capture stdout so that we can see progress indicators in stdout
```
