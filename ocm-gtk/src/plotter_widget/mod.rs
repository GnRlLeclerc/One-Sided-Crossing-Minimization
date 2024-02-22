use std::{cell::RefCell, rc::Rc};

use gtk::glib::{self, subclass::types::ObjectSubclassIsExt, Object};
use ocm_plotter::plottable::Plottable;
use plotters_cairo::CairoBackend;

mod imp;

glib::wrapper! {
    pub struct PlotterWidget(ObjectSubclass<imp::PlotterWidget>) @extends gtk::Widget;
}

/// Implementation of methods for PlotterWidget that will be accessible publicly (we are outside glib::wrapper!)
impl PlotterWidget {
    pub fn new() -> Self {
        Object::builder().build()
    }

    /// Set the plottable struct to be plotted
    pub fn set_plottable(
        &mut self,
        plottable: Rc<RefCell<dyn for<'a> Plottable<CairoBackend<'a>>>>,
    ) {
        self.imp().set_plottable(plottable);
    }

    /// Rerender the widget canvas (exposes the internal `rerender` method from the `imp` module)
    pub fn rerender(&self) {
        self.imp().rerender();
    }
}

impl Default for PlotterWidget {
    fn default() -> Self {
        Self::new()
    }
}
