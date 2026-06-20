//! Theme token value types reused from `spectrum-theme`.

mod context;
mod error;
mod pack;
mod surface;

pub use context::{ThemeContext, with_theme_context};
pub use error::ThemeLoadError;
pub use pack::{
    ADWAITA_LIGHT_TOML, AppTokens, ButtonPrimaryTokens, ButtonStandardTokens, SurfaceRaisedTokens,
    SurfaceTokens, ThemePack, set_theme_pack, with_theme_pack,
};
pub use spectrum_theme::{
    Color, ColorParseError, FontStyle, FontStyleParseError, FontWeight, FontWeightParseError,
    Length, LengthParseError, LengthUnit, LineHeight, LineHeightParseError, Radius,
    RadiusParseError, Rgb, Rgba, ShadowError, ShadowLayer, ThemeBuildError,
};
pub use surface::{SurfaceRole, SurfaceStyleTokens};

#[cfg(test)]
mod tests {
    use super::{Color, Length};

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
}
