use spectrum_theme::{Color, Length, config::TomlThemeSource, define_theme_tokens};
use std::sync::OnceLock;

use crate::context::ThemeMode;

use super::{ADWAITA_DARK_TOML, ADWAITA_LIGHT_TOML, ThemeLoadError};

define_theme_tokens! {
    #[derive(Clone)]
    pub struct ThemePack {
        app {
            window {
                bg: Color,
                fg: Color,
            }
            view {
                bg: Color,
                fg: Color,
            }
        }
        #[derive(Copy, Debug, PartialEq)]
        component SpinnerTokens {
            color: Color,
            size: Length,
        }
        spinner: SpinnerTokens
    }
}

impl ThemePack {
    /// Returns a theme pack based on the given mode.
    #[must_use]
    pub fn from_mode(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::Light => Self::light(),
            ThemeMode::Dark => Self::dark(),
        }
    }

    /// Returns the embedded Adwaita light baseline.
    #[must_use]
    pub fn light() -> Self {
        static ADWAITA_LIGHT: OnceLock<ThemePack> = OnceLock::new();

        ADWAITA_LIGHT
            .get_or_init(|| Self::try_light().expect("embedded Adwaita light theme is valid"))
            .clone()
    }

    /// Returns the embedded Adwaita dark baseline.
    #[must_use]
    pub fn dark() -> Self {
        static ADWAITA_DARK: OnceLock<ThemePack> = OnceLock::new();

        ADWAITA_DARK
            .get_or_init(|| Self::try_dark().expect("embedded Adwaita dark theme is valid"))
            .clone()
    }

    /// Loads the embedded Adwaita light baseline.
    pub fn try_light() -> Result<Self, ThemeLoadError> {
        Self::try_from_toml(ADWAITA_LIGHT_TOML)
    }

    /// Loads the embedded Adwaita dark baseline.
    pub fn try_dark() -> Result<Self, ThemeLoadError> {
        Self::try_from_toml(ADWAITA_DARK_TOML)
    }

    /// Loads a typed theme from TOML.
    pub fn try_from_toml(input: &str) -> Result<Self, ThemeLoadError> {
        let source = TomlThemeSource::parse(input)?;

        Ok(Self::try_from_source(&source)?)
    }
}
