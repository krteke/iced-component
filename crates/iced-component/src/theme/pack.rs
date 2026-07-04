use spectrum_theme::{
    Color, Length, Radius, ShadowLayer, config::TomlThemeSource, define_theme_tokens,
};
use std::sync::OnceLock;

use crate::theme::ThemeLoadError;

/// Embedded default Adwaita-like light theme.
pub const ADWAITA_LIGHT_TOML: &str = include_str!("../../themes/adwaita-light.toml");

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

/// App token group generated for [`ThemePack`].
pub type AppTokens = ThemePackApp;
/// Control metrics generated for [`ThemePack`].
pub type ControlTokens = ThemePackControl;
/// Background surface token group.
pub type SurfaceBackgroundTokens = ThemePackBackgroundStates;
/// Background surface state enum.
pub type SurfaceBackgroundState = ThemePackBackgroundState;
/// Regular surface token group.
pub type SurfaceRegularTokens = ThemePackRegularStates;
/// Regular surface state enum.
pub type SurfaceRegularState = ThemePackRegularState;
/// Raised surface token group.
pub type SurfaceRaisedTokens = ThemePackRaisedStates;
/// Raised surface state enum.
pub type SurfaceRaisedState = ThemePackRaisedState;
/// Panel layout tokens.
pub type PanelComponentTokens = PanelTokens;
/// Button token component generated for [`ThemePack`].
pub type ButtonComponentTokens = ButtonTokens;
/// Standard filled button token group.
pub type ButtonStandardFilledTokens = ThemePackStandardFilledStates;
/// Standard filled button state enum.
pub type ButtonStandardFilledState = ThemePackStandardFilledState;
/// Standard flat button token group.
pub type ButtonStandardFlatTokens = ThemePackStandardFlatStates;
/// Standard flat button state enum.
pub type ButtonStandardFlatState = ThemePackStandardFlatState;
/// Standard raised button token group.
pub type ButtonStandardRaisedTokens = ThemePackStandardRaisedStates;
/// Standard raised button state enum.
pub type ButtonStandardRaisedState = ThemePackStandardRaisedState;
/// Suggested filled button token group.
pub type ButtonSuggestedFilledTokens = ThemePackSuggestedFilledStates;
/// Suggested filled button state enum.
pub type ButtonSuggestedFilledState = ThemePackSuggestedFilledState;
/// Suggested flat button token group.
pub type ButtonSuggestedFlatTokens = ThemePackSuggestedFlatStates;
/// Suggested flat button state enum.
pub type ButtonSuggestedFlatState = ThemePackSuggestedFlatState;
/// Suggested raised button token group.
pub type ButtonSuggestedRaisedTokens = ThemePackSuggestedRaisedStates;
/// Suggested raised button state enum.
pub type ButtonSuggestedRaisedState = ThemePackSuggestedRaisedState;
/// Destructive filled button token group.
pub type ButtonDestructiveFilledTokens = ThemePackDestructiveFilledStates;
/// Destructive filled button state enum.
pub type ButtonDestructiveFilledState = ThemePackDestructiveFilledState;
/// Destructive flat button token group.
pub type ButtonDestructiveFlatTokens = ThemePackDestructiveFlatStates;
/// Destructive flat button state enum.
pub type ButtonDestructiveFlatState = ThemePackDestructiveFlatState;
/// Destructive raised button token group.
pub type ButtonDestructiveRaisedTokens = ThemePackDestructiveRaisedStates;
/// Destructive raised button state enum.
pub type ButtonDestructiveRaisedState = ThemePackDestructiveRaisedState;

impl ThemePack {
    /// Returns the default muted Adwaita-like baseline.
    #[must_use]
    pub fn adwaita() -> Self {
        static ADWAITA: OnceLock<ThemePack> = OnceLock::new();

        ADWAITA
            .get_or_init(|| Self::try_adwaita().expect("embedded Adwaita theme is valid"))
            .clone()
    }

    /// Loads the default muted Adwaita-like baseline.
    pub fn try_adwaita() -> Result<Self, ThemeLoadError> {
        Self::try_from_toml(ADWAITA_LIGHT_TOML)
    }

    /// Loads a typed theme from a TOML theme specification.
    pub fn try_from_toml(input: &str) -> Result<Self, ThemeLoadError> {
        let source = TomlThemeSource::parse(input)?;

        Ok(Self::try_from_source(&source)?)
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;
    use spectrum_theme::{Color, ShadowLayer};

    use super::{ADWAITA_LIGHT_TOML, ThemePack};

    #[test]
    fn adwaita_baseline_uses_muted_blue_accent() {
        let theme = ThemePack::adwaita();
        let accent = theme.button.suggested_filled.idle.bg;

        assert!(accent.blue() > accent.red());
        assert!(accent.red() < 96);
        assert_eq!(accent.alpha(), 255);
    }

    #[test]
    fn default_elevation_is_subtle() {
        let shadow = ThemePack::adwaita().surface.raised.idle.shadow;

        assert!(shadow.color().alpha() <= 48);
        assert!(shadow.blur().value() <= 12.0);
    }

    #[test]
    fn adwaita_buttons_do_not_use_shadows() {
        let theme = ThemePack::adwaita();

        for shadow in [
            theme.button.standard_filled.idle.shadow,
            theme.button.standard_flat.idle.shadow,
            theme.button.standard_raised.idle.shadow,
            theme.button.suggested_filled.idle.shadow,
            theme.button.suggested_flat.idle.shadow,
            theme.button.suggested_raised.idle.shadow,
            theme.button.destructive_filled.idle.shadow,
            theme.button.destructive_flat.idle.shadow,
            theme.button.destructive_raised.idle.shadow,
        ] {
            assert_no_shadow(shadow);
        }
    }

    #[test]
    fn adwaita_theme_is_loaded_from_embedded_toml() {
        let theme = ThemePack::try_from_toml(ADWAITA_LIGHT_TOML).unwrap();

        assert_eq!(theme.app.bg, "#f6f5f4".parse::<Color>().unwrap());
        assert_eq!(
            theme.surface.raised.idle.border,
            theme.surface.regular.idle.border
        );
        assert_eq!(
            theme.button.standard_filled.idle.bg,
            theme.surface.raised.idle.bg
        );
        assert_eq!(
            theme.button.suggested_filled.idle.border,
            theme.button.suggested_filled.idle.bg
        );
        assert_eq!(
            theme.button.destructive_filled.idle.border,
            theme.button.destructive_filled.idle.bg
        );
        assert_eq!(
            theme.button.suggested_filled.disabled.fg,
            theme.button.standard_filled.disabled.fg
        );
        assert_approx_eq!(f32, theme.control.icon_button.size.value(), 40.0);
        assert_approx_eq!(f32, theme.panel.regular.padding.value(), 18.0);
        assert_approx_eq!(f32, theme.panel.regular.spacing.value(), 12.0);
        assert_approx_eq!(f32, theme.panel.regular.title_size.value(), 17.0);
    }

    fn assert_no_shadow(shadow: ShadowLayer) {
        assert_eq!(shadow.color().alpha(), 0);
        assert_approx_eq!(f32, shadow.offset_x().value(), 0.0);
        assert_approx_eq!(f32, shadow.offset_y().value(), 0.0);
        assert_approx_eq!(f32, shadow.blur().value(), 0.0);
        assert_approx_eq!(f32, shadow.spread().value(), 0.0);
    }
}
