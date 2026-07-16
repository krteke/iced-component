use spectrum_theme::{Color, Length, Radius, config::TomlThemeSource, define_theme_tokens};
use std::sync::OnceLock;

use crate::context::ThemeMode;

use super::{ADWAITA_DARK_TOML, ADWAITA_LIGHT_TOML, ThemeLoadError};

define_theme_tokens! {
    #[derive(Clone)]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
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
        accent {
            bg: Color,
            fg: Color,
            color: Color,
        }
        #[derive(Copy, Debug, PartialEq)]
        component ButtonTokens {
            bg: Color,
            fg: Color,
            border: Color,
            border_width: Length,
            radius: Radius,
        }
        #[derive(Copy, Debug, PartialEq)]
        component SpinnerTokens {
            foreground: Color,
            track: Color,
            size: Length,
            minimum_size: Length,
            maximum_size: Length,
            minimum_stroke: Length,
            maximum_stroke: Length,
        }
        button {
            min_width: Length,
            min_height: Length,
            base_padding_x: Length,
            padding_x: Length,
            padding_y: Length,
            image_min_width: Length,
            image_padding_x: Length,
            image_text_padding_x: Length,
            image_text_spacing: Length,
            image_text_label_padding_x: Length,
            icon_size: Length,

            shape {
                rounded { radius: Radius }
                pill {
                    radius: Radius,
                    padding_x: Length,
                    padding_y: Length,
                }
                circular {
                    radius: Radius,
                    size: Length,
                }
            }
            states standard: ButtonTokens {
                idle,
                hover extends idle,
                pressed extends hover,
                disabled extends idle,
            }
            states flat: ButtonTokens {
                idle,
                hover extends idle,
                pressed extends hover,
                disabled extends idle,
            }
            states suggested: ButtonTokens {
                idle,
                hover extends idle,
                pressed extends hover,
                disabled extends idle,
            }
            states destructive: ButtonTokens {
                idle,
                hover extends idle,
                pressed extends hover,
                disabled extends idle,
            }
        }
        spinner: SpinnerTokens
    }
}

/// Standard button token group generated for [`ThemePack`].
pub type ButtonStandardTokens = ThemePackStandardStates;
/// Standard button state enum generated for [`ThemePack`].
pub type ButtonStandardState = ThemePackStandardState;
/// Flat button token group generated for [`ThemePack`].
pub type ButtonFlatTokens = ThemePackFlatStates;
/// Flat button state enum generated for [`ThemePack`].
pub type ButtonFlatState = ThemePackFlatState;
/// Suggested button token group generated for [`ThemePack`].
pub type ButtonSuggestedTokens = ThemePackSuggestedStates;
/// Suggested button state enum generated for [`ThemePack`].
pub type ButtonSuggestedState = ThemePackSuggestedState;
/// Destructive button token group generated for [`ThemePack`].
pub type ButtonDestructiveTokens = ThemePackDestructiveStates;
/// Destructive button state enum generated for [`ThemePack`].
pub type ButtonDestructiveState = ThemePackDestructiveState;

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
