use plotters::{backend::DrawingBackend, coord::Shift, drawing::DrawingArea};

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
    fn plot(&self, root: &mut DrawingArea<DB, Shift>) -> Result<(), Box<dyn std::error::Error>>;
}
