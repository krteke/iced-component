//! Adwaita theme implementation for `iced-component`.

/// Adwaita spinner rendering primitives.
pub mod spinner;
/// Adwaita theme tokens and style provider.
pub mod theme;

pub use theme::{ADWAITA_LIGHT_TOML, ThemeLoadError, ThemePack};
