//! Adapter facade for themed Iced component crates.

/// Extensible contracts and built-in themed backend markers.
pub mod backend;
/// Theme-independent adapters for button components.
pub mod button;
/// Runtime context used to select and configure a themed backend.
pub mod context;

pub use iced_component_core as core;
