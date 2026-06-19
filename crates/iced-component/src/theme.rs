//! Theme token value types reused from `spectrum-theme`.

use core::convert::Infallible;
use std::cell::RefCell;

use spectrum_theme::{
    __private::{ColorSource, LengthSource, RadiusSource, ShadowSource, TokenSource},
    define_theme_tokens,
};
pub use spectrum_theme::{
    Color, ColorParseError, FontStyle, FontStyleParseError, FontWeight, FontWeightParseError,
    Length, LengthParseError, LengthUnit, LineHeight, LineHeightParseError, Radius,
    RadiusParseError, Rgb, Rgba, ShadowError, ShadowLayer, ThemeBuildError,
};

define_theme_tokens! {
    pub struct ThemePack {
        palette {
            background: Color,
            surface: Color,
            surface_raised: Color,
            text: Color,
            text_muted: Color,
            accent: Color,
            accent_text: Color,
            border: Color,
            focus_ring: Color,
        }
        shape {
            control_radius: Radius,
            panel_radius: Radius,
            gap: Length,
        }
        elevation {
            raised: ShadowLayer,
        }
    }
}

/// Color token group generated for [`ThemePack`].
pub type PaletteTokens = ThemePackPalette;
/// Shape token group generated for [`ThemePack`].
pub type ShapeTokens = ThemePackShape;
/// Elevation token group generated for [`ThemePack`].
pub type ElevationTokens = ThemePackElevation;

thread_local! {
    static CURRENT_THEME: RefCell<ThemePack> = RefCell::new(ThemePack::adwaita());
}

impl ThemePack {
    /// Returns the default muted Adwaita-like baseline.
    #[must_use]
    pub fn adwaita() -> Self {
        Self::try_from_source(&AdwaitaSource).expect("Adwaita source is infallible")
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

fn px(value: f32) -> Length {
    Length::new(value, LengthUnit::Px).expect("finite px length")
}

fn radius(value: f32) -> Radius {
    Radius::new(px(value)).expect("non-negative radius")
}

struct AdwaitaSource;

impl TokenSource for AdwaitaSource {
    type Error = Infallible;
}

impl ColorSource for AdwaitaSource {
    fn color(&self, path: &str) -> Result<Color, Self::Error> {
        Ok(match path {
            "palette.background" => Color::new(246, 245, 244),
            "palette.surface" | "palette.accent_text" => Color::new(255, 255, 255),
            "palette.surface_raised" => Color::new(250, 250, 250),
            "palette.text" => Color::new(36, 31, 49),
            "palette.text_muted" => Color::new(94, 92, 100),
            "palette.accent" => Color::new(53, 132, 228),
            "palette.border" => Color::new_rgba(0, 0, 0, 31),
            "palette.focus_ring" => Color::new(28, 113, 216),
            _ => unreachable!("unknown color token path: {path}"),
        })
    }
}

impl LengthSource for AdwaitaSource {
    fn length(&self, path: &str) -> Result<Length, Self::Error> {
        Ok(match path {
            "shape.gap" => px(8.0),
            _ => unreachable!("unknown length token path: {path}"),
        })
    }
}

impl RadiusSource for AdwaitaSource {
    fn radius(&self, path: &str) -> Result<Radius, Self::Error> {
        Ok(match path {
            "shape.control_radius" => radius(6.0),
            "shape.panel_radius" => radius(8.0),
            _ => unreachable!("unknown radius token path: {path}"),
        })
    }
}

impl ShadowSource for AdwaitaSource {
    fn shadow(&self, path: &str) -> Result<ShadowLayer, Self::Error> {
        assert_eq!(path, "elevation.raised");
        Ok(ShadowLayer::new(
            Color::new_rgba(0, 0, 0, 36),
            px(0.0),
            px(2.0),
            px(10.0),
            px(0.0),
        )
        .expect("default shadow is valid"))
    }
}

#[cfg(test)]
mod tests {
    use super::{Color, Length, ThemePack, set_theme_pack, with_theme_pack};

    #[test]
    fn color_reexport_uses_spectrum_parser() {
        let color = "#336699".parse::<Color>().unwrap();

        assert_eq!(color.to_string(), "#336699");
    }

    #[test]
    fn length_reexport_uses_spectrum_parser() {
        let length = "12px".parse::<Length>().unwrap();

        assert_eq!(length.to_string(), "12px");
    }

    #[test]
    fn adwaita_baseline_uses_muted_blue_accent() {
        let theme = ThemePack::adwaita();
        let accent = theme.palette.accent;

        assert!(accent.blue() > accent.red());
        assert!(accent.red() < 96);
        assert_eq!(accent.alpha(), 255);
    }

    #[test]
    fn default_elevation_is_subtle() {
        let shadow = ThemePack::adwaita().elevation.raised;

        assert!(shadow.color().alpha() <= 48);
        assert!(shadow.blur().value() <= 12.0);
    }

    #[test]
    fn thread_local_theme_can_be_replaced() {
        let accent = Color::new(26, 95, 180);
        let mut theme = ThemePack::adwaita();
        theme.palette.accent = accent;

        set_theme_pack(theme);

        with_theme_pack(|current| assert_eq!(current.palette.accent, accent));
        set_theme_pack(ThemePack::adwaita());
    }
}
