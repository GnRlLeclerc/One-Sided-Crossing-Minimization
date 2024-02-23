# INF562 - OCM Problem

A solver for the One-sided crossing minimization problem.

## Quickstart

### CLI

Run the CLI solver with the following command:

```bash
cargo run  --bin ocm-cli -- datasets/tiny/complete_4_5.gr -t
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
cargo run  --bin ocm-cli -- datasets/tiny/complete_4_5.gr
```

Run the CLI solver for large graphs with:

```bash
cargo run --release  --bin ocm-cli --  -t datasets/large/25.gr
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
└── ocm-solver   # Implementation logic
```
