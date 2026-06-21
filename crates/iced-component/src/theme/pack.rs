use spectrum_resolver::resolve_theme;
use spectrum_schema::ThemeSpec;
use spectrum_theme::{Color, Radius, ShadowLayer, define_theme_tokens};
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
        button {
            standard {
                bg: Color,
                fg: Color,
                border: Color,
                hover { bg: Color }
                pressed { bg: Color }
                disabled {
                    bg: Color,
                    fg: Color,
                }
                focus { ring: Color }
                radius: Radius,
                shadow: ShadowLayer,
            }
            primary {
                bg: Color,
                fg: Color,
                border: Color,
                hover { bg: Color }
                pressed { bg: Color }
                disabled {
                    bg: Color,
                    fg: Color,
                }
                focus { ring: Color }
                radius: Radius,
                shadow: ShadowLayer,
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
/// Standard button token group generated for [`ThemePack`].
pub type ButtonStandardTokens = ThemePackButtonStandard;
/// Primary button token group generated for [`ThemePack`].
pub type ButtonPrimaryTokens = ThemePackButtonPrimary;

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
    use spectrum_theme::Color;

    use super::{ADWAITA_LIGHT_TOML, ThemePack, set_theme_pack, with_theme_pack};

    #[test]
    fn adwaita_baseline_uses_muted_blue_accent() {
        let theme = ThemePack::adwaita();
        let accent = theme.button.primary.bg;

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
        theme.button.primary.bg = accent;

        set_theme_pack(theme);

        with_theme_pack(|current| assert_eq!(current.button.primary.bg, accent));
        set_theme_pack(ThemePack::adwaita());
    }

    #[test]
    fn adwaita_theme_is_loaded_from_embedded_toml() {
        let theme = ThemePack::try_from_toml(ADWAITA_LIGHT_TOML).unwrap();

        assert_eq!(theme.app.bg, "#f6f5f4".parse::<Color>().unwrap());
        assert_eq!(theme.surface.raised.border, theme.surface.base.border);
        assert_eq!(theme.button.standard.bg, theme.surface.raised.bg);
        assert_eq!(theme.button.primary.border, theme.button.primary.bg);
        assert_eq!(
            theme.button.primary.disabled.fg,
            theme.button.standard.disabled.fg
        );
    }
}
