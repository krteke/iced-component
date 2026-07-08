use spectrum_theme::{Color, config::TomlThemeSource, define_theme_tokens};

use crate::theme::ThemeLoadError;

define_theme_tokens! {
    #[derive(Clone)]
    pub struct ThemePack {
        app {
            bg: Color,
            fg: Color,
            fg_muted: Color,
        }
    }
}

impl ThemePack {
    /// Loads a typed theme from TOML.
    pub fn try_from_toml(input: &str) -> Result<Self, ThemeLoadError> {
        let source = TomlThemeSource::parse(input)?;

        Ok(Self::try_from_source(&source)?)
    }
}
