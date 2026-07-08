use spectrum_theme::{
    Color, Length, Radius, ShadowLayer, config::TomlThemeSource, define_theme_tokens,
};
use std::sync::OnceLock;

use super::{ADWAITA_LIGHT_TOML, ThemeLoadError};

define_theme_tokens! {
    #[derive(Clone)]
    pub struct ThemePack {
        app {
            bg: Color,
            fg: Color,
            fg_muted: Color,
        }
        control {
            border { width: Length }
            surface {
                padding: Length,
            }
            button {
                padding_x: Length,
                padding_y: Length,
            }
            icon_button {
                size: Length,
                icon_size: Length,
            }
        }
        #[derive(Copy, Debug, PartialEq)]
        component ButtonTokens {
            bg: Color,
            fg: Color,
            border: Color,
            border_width: Length,
            radius: Radius,
            focus_ring: Color,
            shadow: ShadowLayer,
        }
        #[derive(Copy, Debug, PartialEq)]
        component SurfaceTokens {
            bg: Color,
            fg: Color,
            border: Color,
            border_width: Length,
            radius: Radius,
            shadow: ShadowLayer,
        }
        #[derive(Copy, Debug, PartialEq)]
        component PanelTokens {
            padding: Length,
            spacing: Length,
            title_size: Length,
        }
        #[derive(Copy, Debug, PartialEq)]
        component SpinnerTokens {
            color: Color,
            size: Length,
        }
        surface {
            states background: SurfaceTokens {
                idle,
                hover extends idle,
            }
            states regular: SurfaceTokens {
                idle,
                hover extends idle,
            }
            states raised: SurfaceTokens {
                idle,
                hover extends idle,
            }
        }
        panel {
            regular: PanelTokens,
        }
        spinner: SpinnerTokens
        button {
            shape {
                rounded { radius: Radius }
                pill { radius: Radius }
                circular { radius: Radius }
            }
            states standard_filled: ButtonTokens {
                idle,
                hover extends idle,
                pressed extends hover,
                disabled extends idle,
            }
            states standard_flat: ButtonTokens {
                idle,
                hover extends idle,
                pressed extends hover,
                disabled extends idle,
            }
            states standard_raised: ButtonTokens {
                idle,
                hover extends idle,
                pressed extends hover,
                disabled extends idle,
            }
            states suggested_filled inherit standard_filled,
            states suggested_flat inherit standard_filled,
            states suggested_raised inherit standard_filled,
            states destructive_filled inherit standard_filled,
            states destructive_flat inherit standard_filled,
            states destructive_raised inherit standard_filled,
        }
    }
}

impl ThemePack {
    /// Returns the embedded Adwaita light baseline.
    #[must_use]
    pub fn light() -> Self {
        static ADWAITA_LIGHT: OnceLock<ThemePack> = OnceLock::new();

        ADWAITA_LIGHT
            .get_or_init(|| Self::try_light().expect("embedded Adwaita theme is valid"))
            .clone()
    }

    /// Loads the embedded Adwaita light baseline.
    pub fn try_light() -> Result<Self, ThemeLoadError> {
        Self::try_from_toml(ADWAITA_LIGHT_TOML)
    }

    /// Loads a typed theme from TOML.
    pub fn try_from_toml(input: &str) -> Result<Self, ThemeLoadError> {
        let source = TomlThemeSource::parse(input)?;

        Ok(Self::try_from_source(&source)?)
    }
}
