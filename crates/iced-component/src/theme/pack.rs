use spectrum_resolver::resolve_theme;
use spectrum_schema::ThemeSpec;
use spectrum_theme::{Color, Length, Radius, ShadowLayer, define_theme_tokens};
use std::cell::RefCell;
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
        surface {
            base {
                bg: Color,
                fg: Color,
                border: Color,
                radius: Radius,
            }
            raised {
                bg: Color,
                fg: Color,
                border: Color,
                radius: Radius,
                shadow: ShadowLayer,
            }
        }
        control {
            border { width: Length }
            button {
                padding_x: Length,
                padding_y: Length,
            }
            icon_button {
                size: Length,
                icon_size: Length,
            }
        }
        button {
            shape {
                rounded { radius: Radius }
                pill { radius: Radius }
                circular { radius: Radius }
            }
            standard {
                filled {
                    idle { bg: Color, fg: Color, border: Color }
                    hover { bg: Color, fg: Color, border: Color }
                    pressed { bg: Color, fg: Color, border: Color }
                    disabled { bg: Color, fg: Color, border: Color }
                    focus { ring: Color }
                    shadow: ShadowLayer
                }
                flat {
                    idle { bg: Color, fg: Color, border: Color }
                    hover { bg: Color, fg: Color, border: Color }
                    pressed { bg: Color, fg: Color, border: Color }
                    disabled { bg: Color, fg: Color, border: Color }
                    focus { ring: Color }
                    shadow: ShadowLayer
                }
                raised {
                    idle { bg: Color, fg: Color, border: Color }
                    hover { bg: Color, fg: Color, border: Color }
                    pressed { bg: Color, fg: Color, border: Color }
                    disabled { bg: Color, fg: Color, border: Color }
                    focus { ring: Color }
                    shadow: ShadowLayer
                }
            }
            suggested {
                filled {
                    idle { bg: Color, fg: Color, border: Color }
                    hover { bg: Color, fg: Color, border: Color }
                    pressed { bg: Color, fg: Color, border: Color }
                    disabled { bg: Color, fg: Color, border: Color }
                    focus { ring: Color }
                    shadow: ShadowLayer
                }
                flat {
                    idle { bg: Color, fg: Color, border: Color }
                    hover { bg: Color, fg: Color, border: Color }
                    pressed { bg: Color, fg: Color, border: Color }
                    disabled { bg: Color, fg: Color, border: Color }
                    focus { ring: Color }
                    shadow: ShadowLayer
                }
                raised {
                    idle { bg: Color, fg: Color, border: Color }
                    hover { bg: Color, fg: Color, border: Color }
                    pressed { bg: Color, fg: Color, border: Color }
                    disabled { bg: Color, fg: Color, border: Color }
                    focus { ring: Color }
                    shadow: ShadowLayer
                }
            }
            destructive {
                filled {
                    idle { bg: Color, fg: Color, border: Color }
                    hover { bg: Color, fg: Color, border: Color }
                    pressed { bg: Color, fg: Color, border: Color }
                    disabled { bg: Color, fg: Color, border: Color }
                    focus { ring: Color }
                    shadow: ShadowLayer
                }
                flat {
                    idle { bg: Color, fg: Color, border: Color }
                    hover { bg: Color, fg: Color, border: Color }
                    pressed { bg: Color, fg: Color, border: Color }
                    disabled { bg: Color, fg: Color, border: Color }
                    focus { ring: Color }
                    shadow: ShadowLayer
                }
                raised {
                    idle { bg: Color, fg: Color, border: Color }
                    hover { bg: Color, fg: Color, border: Color }
                    pressed { bg: Color, fg: Color, border: Color }
                    disabled { bg: Color, fg: Color, border: Color }
                    focus { ring: Color }
                    shadow: ShadowLayer
                }
            }
        }
    }
}

/// App token group generated for [`ThemePack`].
pub type AppTokens = ThemePackApp;
/// Regular surface token group generated for [`ThemePack`].
pub type SurfaceTokens = ThemePackSurfaceBase;
/// Raised surface token group generated for [`ThemePack`].
pub type SurfaceRaisedTokens = ThemePackSurfaceRaised;
/// Control metrics generated for [`ThemePack`].
pub type ControlTokens = ThemePackControl;
/// Standard button token group generated for [`ThemePack`].
pub type ButtonStandardTokens = ThemePackButtonStandard;
/// Standard filled button token group.
pub type ButtonStandardFilledTokens = ThemePackButtonStandardFilled;
/// Standard flat button token group.
pub type ButtonStandardFlatTokens = ThemePackButtonStandardFlat;
/// Standard raised button token group.
pub type ButtonStandardRaisedTokens = ThemePackButtonStandardRaised;
/// Suggested-action button token group generated for [`ThemePack`].
pub type ButtonSuggestedTokens = ThemePackButtonSuggested;
/// Suggested filled button token group.
pub type ButtonSuggestedFilledTokens = ThemePackButtonSuggestedFilled;
/// Suggested flat button token group.
pub type ButtonSuggestedFlatTokens = ThemePackButtonSuggestedFlat;
/// Suggested raised button token group.
pub type ButtonSuggestedRaisedTokens = ThemePackButtonSuggestedRaised;
/// Destructive-action button token group generated for [`ThemePack`].
pub type ButtonDestructiveTokens = ThemePackButtonDestructive;
/// Destructive filled button token group.
pub type ButtonDestructiveFilledTokens = ThemePackButtonDestructiveFilled;
/// Destructive flat button token group.
pub type ButtonDestructiveFlatTokens = ThemePackButtonDestructiveFlat;
/// Destructive raised button token group.
pub type ButtonDestructiveRaisedTokens = ThemePackButtonDestructiveRaised;
/// Backward-compatible alias for suggested-action button tokens.
pub type ButtonPrimaryTokens = ButtonSuggestedTokens;

thread_local! {
    static CURRENT_THEME: RefCell<ThemePack> = RefCell::new(ThemePack::adwaita());
}

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
        let spec = toml::from_str::<ThemeSpec>(input)?;
        let resolved = resolve_theme(&spec)?;

        Ok(Self::try_from_source(&resolved)?)
    }
}

/// Reads the current thread-local theme pack.
pub fn with_theme_pack<R>(read: impl FnOnce(&ThemePack) -> R) -> R {
    CURRENT_THEME.with(|theme| read(&theme.borrow()))
}

/// Replaces the current thread-local theme pack.
pub fn set_theme_pack(theme: ThemePack) {
    CURRENT_THEME.with(|current| *current.borrow_mut() = theme);
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;
    use spectrum_theme::Color;

    use super::{ADWAITA_LIGHT_TOML, ThemePack, set_theme_pack, with_theme_pack};

    #[test]
    fn adwaita_baseline_uses_muted_blue_accent() {
        let theme = ThemePack::adwaita();
        let accent = theme.button.suggested.filled.idle.bg;

        assert!(accent.blue() > accent.red());
        assert!(accent.red() < 96);
        assert_eq!(accent.alpha(), 255);
    }

    #[test]
    fn default_elevation_is_subtle() {
        let shadow = ThemePack::adwaita().surface.raised.shadow;

        assert!(shadow.color().alpha() <= 48);
        assert!(shadow.blur().value() <= 12.0);
    }

    #[test]
    fn thread_local_theme_can_be_replaced() {
        let accent = Color::new(26, 95, 180);
        let mut theme = ThemePack::adwaita();
        theme.button.suggested.filled.idle.bg = accent;

        set_theme_pack(theme);

        with_theme_pack(|current| assert_eq!(current.button.suggested.filled.idle.bg, accent));
        set_theme_pack(ThemePack::adwaita());
    }

    #[test]
    fn adwaita_theme_is_loaded_from_embedded_toml() {
        let theme = ThemePack::try_from_toml(ADWAITA_LIGHT_TOML).unwrap();

        assert_eq!(theme.app.bg, "#f6f5f4".parse::<Color>().unwrap());
        assert_eq!(theme.surface.raised.border, theme.surface.base.border);
        assert_eq!(
            theme.button.standard.filled.idle.bg,
            theme.surface.raised.bg
        );
        assert_eq!(
            theme.button.suggested.filled.idle.border,
            theme.button.suggested.filled.idle.bg
        );
        assert_eq!(
            theme.button.destructive.filled.idle.border,
            theme.button.destructive.filled.idle.bg
        );
        assert_eq!(
            theme.button.suggested.filled.disabled.fg,
            theme.button.standard.filled.disabled.fg
        );
        assert_approx_eq!(f32, theme.control.icon_button.size.value(), 40.0);
    }
}
