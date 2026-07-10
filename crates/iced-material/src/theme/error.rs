use spectrum_theme::{ThemeBuildError, config::TomlThemeSourceError};
use thiserror::Error;

use crate::context::ThemeMode;

/// Error returned while loading a Material theme pack.
#[derive(Debug, Error)]
pub enum ThemeLoadError {
    /// TOML parsing failed.
    #[error("failed to parse theme TOML: {0}")]
    ParseToml(#[from] TomlThemeSourceError),
    /// Typed token construction failed.
    #[error("failed to build typed theme: {0}")]
    Build(#[from] ThemeBuildError),
    /// The TOML source mode does not match the destination theme slot.
    #[error("theme source mode {actual} cannot be installed into the {expected} slot")]
    ModeMismatch {
        /// The mode selected by the API caller.
        expected: ThemeMode,
        /// The mode declared in TOML metadata.
        actual: ThemeMode,
    },
}

/// Error returned when reseeding a Material theme is not possible.
#[derive(Debug, Error)]
pub enum ThemeSeedError {
    /// One mode no longer has source data after a direct pack replacement or patch.
    #[error("cannot apply a Material seed because the {mode} pack is not source-backed")]
    SourceDetached {
        /// The mode that needs a TOML source before it can be reseeded.
        mode: ThemeMode,
    },
    /// Rebuilding a source-backed pack failed.
    #[error(transparent)]
    Load(#[from] ThemeLoadError),
}
