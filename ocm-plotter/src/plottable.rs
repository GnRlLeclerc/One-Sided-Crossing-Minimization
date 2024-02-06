use plotters::{
    backend::DrawingBackend,
    coord::Shift,
    drawing::{DrawingArea, IntoDrawingArea},
};
use plotters_bitmap::BitMapBackend;

/// Trait that defines a struct that can be plotted using plotters-rs
/// The idea is that a wrapper struct can contain data and plotting parameters in order to alter the plot.
/// This structure would then be shared to the plotting widget, which would plot it.
///
/// This trait is generic in order to be compatible with any plotting backend,
/// such as Cairo for GTK apps or bitmap for CLI image generation.
pub trait Plottable<DB>
where
    DB: DrawingBackend,
{
    fn plot(&self, root: &mut DrawingArea<DB, Shift>);
}

/// Plot a plottable object and save it to a file
#[allow(dead_code)]
pub fn plot_to_file<T: for<'a> Plottable<BitMapBackend<'a>>>(plottable: &T, filename: &str) {
    let mut root = BitMapBackend::new(filename, (800, 600)).into_drawing_area();

    plottable.plot(&mut root);

    root.present()
        .expect("Unable to write the plot to the file");
}
