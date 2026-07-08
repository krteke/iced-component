use spectrum_theme::{ThemeBuildError, config::TomlThemeSourceError};
use thiserror::Error;

/// Error returned while loading an Adwaita theme pack.
#[derive(Debug, Error)]
pub enum ThemeLoadError {
    /// TOML parsing failed.
    #[error("failed to parse theme TOML: {0}")]
    ParseToml(#[from] TomlThemeSourceError),
    /// Typed token construction failed.
    #[error("failed to build typed theme: {0}")]
    Build(#[from] ThemeBuildError),
}
