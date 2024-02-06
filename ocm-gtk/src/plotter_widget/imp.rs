use std::cell::RefCell;
use std::rc::Rc;

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use ocm_plotter::plottable::Plottable;

use plotters::prelude::*;
use plotters_cairo::CairoBackend;

/// A wrapper struct for plottable structs
///
/// Let's dive into the details of this struct:
/// - Option: because the Widget might have nothing to plot
/// - Rc: double usage. It allows polymorphism for the trait Plottable, and it allows sharing a data struct without copy.
/// - dyn Plottable<CairoBackend>: a struct that implements the Plottable trait, in particular for the GTK Cairo backend
#[derive(Default, Clone)]
struct PlottableWrapper {
    plottable: Option<Rc<dyn for<'a> Plottable<CairoBackend<'a>>>>,
}

/// The Plotter Widget.
/// Note that because the external implementation of this widget in the imp module anly allows access to the implementation
/// via immutable references, we are forced to use RefCells in order to change the plottable data from outside the widget.
///
/// RefCells are a runtime borrow checker.
#[derive(Default)]
pub struct PlotterWidget {
    wrapper: RefCell<PlottableWrapper>,
}

// Base definition for GTK object subclassing
#[glib::object_subclass]
impl ObjectSubclass for PlotterWidget {
    const NAME: &'static str = "PlotterWidget";
    type Type = super::PlotterWidget;
    type ParentType = gtk::Widget;
}

// Trait shared by all GObjects
impl ObjectImpl for PlotterWidget {}

// Trait shared by all widgets
impl WidgetImpl for PlotterWidget {
    /// This function is called every time the widget needs to be redrawn. This is typically triggered after a call to `queue_draw()`.
    fn snapshot(&self, snapshot: &gtk::Snapshot) {
        // Get the current widget size to determine the canvas size to draw on
        let width = self.obj().width() as u32;
        let height = self.obj().height() as u32;
        if width == 0 || height == 0 {
            return;
        }

        let bounds = gtk::graphene::Rect::new(0.0, 0.0, width as f32, height as f32);
        let cr = snapshot.append_cairo(&bounds);
        let backend = CairoBackend::new(&cr, (width, height)).unwrap();
        let mut root = backend.into_drawing_area();

        if self.wrapper.borrow().plottable.is_some() {
            // If the wrapper contains some plottable data, do plot it
            self.wrapper
                .borrow()
                .plottable
                .as_ref()
                .unwrap()
                .plot(&mut root);
        } else {
            // Else, fill the canvas with white
            root.fill(&WHITE).unwrap();
        }
        root.present().unwrap();
    }
}

impl PlotterWidget {
    /// Set the plottable struct to be plotted
    pub fn set_plottable(&self, plottable: Rc<dyn for<'a> Plottable<CairoBackend<'a>>>) {
        self.wrapper.borrow_mut().plottable = Some(plottable);
    }

    /// Rerender the widget canvas
    pub fn rerender(&self) {
        self.obj().queue_draw();
    }
}
