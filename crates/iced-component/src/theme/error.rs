use spectrum_resolver::ResolveError;
use spectrum_theme::ThemeBuildError;
use thiserror::Error;

/// Error returned while loading a theme pack.
#[derive(Debug, Error)]
pub enum ThemeLoadError {
    /// TOML parsing failed.
    #[error("failed to parse theme TOML: {0}")]
    ParseToml(#[from] toml::de::Error),
    /// Theme reference or Material resolution failed.
    #[error("failed to resolve theme: {0}")]
    Resolve(#[from] ResolveError),
    /// Typed token construction failed.
    #[error("failed to build typed theme: {0}")]
    Build(#[from] ThemeBuildError),
    /// Failed to load theme from file.
    #[error("failed to load theme from file: {0}")]
    Load(#[from] std::io::Error),
}
