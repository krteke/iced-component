use std::sync::OnceLock;

use spectrum_theme::{Color, Length, Radius, config::TomlThemeSource, define_theme_tokens};

use crate::{
    context::ThemeMode,
    theme::{MATERIAL_DARK_TOML, MATERIAL_LIGHT_TOML, Opacity, ThemeLoadError},
};

define_theme_tokens! {
    #[derive(Clone)]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    pub struct ThemePack {
        #[derive(Copy, Debug, PartialEq)]
        component ColorRole {
            color: Color,
            on_color: Color,
            container: Color,
            on_container: Color,
        }
        #[derive(Copy, Debug, PartialEq)]
        component ButtonStyleTokens {
            background: Color,
            background_opacity: Opacity,
            foreground: Color,
            foreground_opacity: Opacity,
            border: Color,
            border_opacity: Opacity,
            border_width: Length,
            radius: Radius,
            shadow: Color,
            shadow_opacity: Opacity,
            shadow_y: Length,
            shadow_blur: Length,
            state_layer: Color,
            state_layer_opacity: Opacity,
        }
        #[derive(Copy, Debug, PartialEq)]
        component LoadingIndicatorTokens {
            size: Length,
            active: Color,
            container: Color,
            contained_active: Color,
        }
        palette {
            background {
                color: Color,
                on_color: Color,
            }
            primary: ColorRole,
            secondary: ColorRole,
            tertiary: ColorRole,
            error: ColorRole,
            surface {
                color: Color,
                on_color: Color,
                on_color_variant: Color,
                dim: Color,
                bright: Color,
                variant: Color,
                container {
                    lowest: Color,
                    low: Color,
                    base: Color,
                    high: Color,
                    highest: Color,
                }
            }
            inverse {
                surface: Color,
                on_surface: Color,
                primary: Color,
            }
            outline {
                color: Color,
                variant: Color,
            }
            shadow: Color,
            scrim: Color,
            fixed {
                primary {
                    color: Color,
                    dim: Color,
                    on_color: Color,
                    on_color_variant: Color,
                }
                secondary {
                    color: Color,
                    dim: Color,
                    on_color: Color,
                    on_color_variant: Color,
                }
                tertiary {
                    color: Color,
                    dim: Color,
                    on_color: Color,
                    on_color_variant: Color,
                }
            }
        }
        button {
            container_height: Length,
            corner_radius: Radius,
            label_size: Length,
            label_line_height: Length,
            icon_size: Length,
            leading_padding: Length,
            trailing_padding: Length,
            leading_icon_padding: Length,
            trailing_icon_padding: Length,
            outlined_border_width: Length,
            states elevated: ButtonStyleTokens {
                idle,
                hover extends idle,
                pressed extends idle,
                disabled extends idle,
                focus extends hover,
            }
            states filled: ButtonStyleTokens {
                idle,
                hover extends idle,
                pressed extends idle,
                disabled extends idle,
                focus extends hover,
            }
            states filled_tonal: ButtonStyleTokens {
                idle,
                hover extends idle,
                pressed extends idle,
                disabled extends idle,
                focus extends hover,
            }
            states outlined: ButtonStyleTokens {
                idle,
                hover extends idle,
                pressed extends idle,
                disabled extends idle,
                focus extends hover,
            }
            states text: ButtonStyleTokens {
                idle,
                hover extends idle,
                pressed extends idle,
                disabled extends idle,
                focus extends hover,
            }
        }
        loading_indicator: LoadingIndicatorTokens,
    }
}

/// Material elevated button states generated for [`ThemePack`].
pub type ButtonElevatedTokens = ThemePackElevatedStates;
/// Material elevated button state enum generated for [`ThemePack`].
pub type ButtonElevatedState = ThemePackElevatedState;
/// Material filled button states generated for [`ThemePack`].
pub type ButtonFilledTokens = ThemePackFilledStates;
/// Material filled button state enum generated for [`ThemePack`].
pub type ButtonFilledState = ThemePackFilledState;
/// Material filled tonal button states generated for [`ThemePack`].
pub type ButtonFilledTonalTokens = ThemePackFilledTonalStates;
/// Material filled tonal button state enum generated for [`ThemePack`].
pub type ButtonFilledTonalState = ThemePackFilledTonalState;
/// Material outlined button states generated for [`ThemePack`].
pub type ButtonOutlinedTokens = ThemePackOutlinedStates;
/// Material outlined button state enum generated for [`ThemePack`].
pub type ButtonOutlinedState = ThemePackOutlinedState;
/// Material text button states generated for [`ThemePack`].
pub type ButtonTextTokens = ThemePackTextStates;
/// Material text button state enum generated for [`ThemePack`].
pub type ButtonTextState = ThemePackTextState;

impl ThemePack {
    /// Returns a theme pack based on the given mode.
    #[must_use]
    pub fn from_mode(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::Light => Self::light(),
            ThemeMode::Dark => Self::dark(),
        }
    }

    /// Returns the embedded Material 3 light baseline.
    #[must_use]
    pub fn light() -> Self {
        static MATERIAL_LIGHT: OnceLock<ThemePack> = OnceLock::new();

        MATERIAL_LIGHT
            .get_or_init(|| Self::try_light().expect("embedded Material light theme is valid"))
            .clone()
    }

    /// Returns the embedded Material 3 dark baseline.
    #[must_use]
    pub fn dark() -> Self {
        static MATERIAL_DARK: OnceLock<ThemePack> = OnceLock::new();

        MATERIAL_DARK
            .get_or_init(|| Self::try_dark().expect("embedded Material dark theme is valid"))
            .clone()
    }

    /// Loads the embedded Material 3 light baseline.
    pub fn try_light() -> Result<Self, ThemeLoadError> {
        Self::try_from_toml(MATERIAL_LIGHT_TOML)
    }

    /// Loads the embedded Material 3 light baseline with a different seed.
    pub fn try_light_with_seed(seed: Color) -> Result<Self, ThemeLoadError> {
        Self::try_from_toml_with_seed(MATERIAL_LIGHT_TOML, seed)
    }

    /// Loads the embedded Material 3 dark baseline.
    pub fn try_dark() -> Result<Self, ThemeLoadError> {
        Self::try_from_toml(MATERIAL_DARK_TOML)
    }

    /// Loads the embedded Material 3 dark baseline with a different seed.
    pub fn try_dark_with_seed(seed: Color) -> Result<Self, ThemeLoadError> {
        Self::try_from_toml_with_seed(MATERIAL_DARK_TOML, seed)
    }

    /// Loads a typed theme from TOML.
    pub fn try_from_toml(input: &str) -> Result<Self, ThemeLoadError> {
        let source = TomlThemeSource::parse(input)?;

        Self::try_from_toml_source(&source)
    }

    /// Loads a typed theme from TOML with an overriding Material seed.
    pub fn try_from_toml_with_seed(input: &str, seed: Color) -> Result<Self, ThemeLoadError> {
        let source = TomlThemeSource::parse(input)?.with_seed(seed);

        Self::try_from_toml_source(&source)
    }

    pub(crate) fn try_from_toml_source(source: &TomlThemeSource) -> Result<Self, ThemeLoadError> {
        Ok(Self::try_from_source(source)?)
    }
}
