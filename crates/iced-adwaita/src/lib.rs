//! Adwaita theme implementation for `iced-component`.

/// Adwaita button component primitives.
pub mod button;
/// Shared Adwaita component context.
pub mod context;
/// Adwaita spinner rendering primitives.
pub mod spinner;
/// Adwaita theme tokens and style provider.
pub mod theme;

pub use context::Context;
pub use theme::{ADWAITA_DARK_TOML, ADWAITA_LIGHT_TOML, ThemeLoadError, ThemePack};
