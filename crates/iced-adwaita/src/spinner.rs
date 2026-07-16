//! Independent adwaita-like spinner components and rendering primitives.
//!
//! This module targets visual compatibility with the Adwaita design language.
//! It is independently implemented and is not produced, affiliated with, or
//! endorsed by GNOME or libadwaita.

mod appearance;
mod cadence;
mod sample;
mod shader;
mod timeline;
mod widget;

pub use appearance::SpinnerAppearance;
pub use cadence::SpinnerCadence;
pub use sample::SpinnerSample;
pub use timeline::{SpinnerPlayback, SpinnerTimeline};
pub use widget::{Spinner, SpinnerRender};
