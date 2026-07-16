//! Independent adwaita-like theme implementation for `iced-component`.
//!
//! This crate is not produced by, affiliated with, or endorsed by GNOME or
//! libadwaita. It provides an independently implemented Iced theme whose
//! default visual direction targets broad compatibility with Adwaita-like apps.

/// Adwaita-like button component primitives.
pub mod button;
/// Shared adwaita-like component context.
pub mod context;
/// Adwaita-like spinner rendering primitives.
pub mod spinner;
/// Adwaita-like theme tokens and style provider.
pub mod theme;

pub use context::Context;
pub use theme::{ADWAITA_DARK_TOML, ADWAITA_LIGHT_TOML, ThemeLoadError, ThemePack};
