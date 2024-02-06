//! Utility functions for GTK interfaces
//!
//! Note that because of Rust's strict ownership system,
//! we decided to use RefCell and Rc to share the plottable object's data.
//! Thus, a clonable Rc is expected for the drawing functions.

use std::rc::Rc;

use gtk::{prelude::*, ApplicationWindow};
use ocm_plotter::plottable::Plottable;
use plotters_cairo::CairoBackend;

use crate::plotter_widget::PlotterWidget;

/// Plot the given plottable object in a brand new window application
pub fn plot_in_window(app_id: &str, plottable: Rc<dyn for<'a> Plottable<CairoBackend<'a>>>) {
    let application = gtk::Application::new(Some(app_id), Default::default());

    application.connect_activate(move |app| {
        // Create the plotting widget for this application
        let mut plot_widget = PlotterWidget::new();
        plot_widget.set_plottable(plottable.clone());

        // Create a window and set the title
        let window = ApplicationWindow::builder()
            .application(app)
            .title("GUI OCM Problem Solver")
            .default_height(500)
            .default_width(800)
            .child(&plot_widget)
            .build();

        window.present();
    });

    // Run with empty args
    application.run_with_args::<&str>(&[]);
}
