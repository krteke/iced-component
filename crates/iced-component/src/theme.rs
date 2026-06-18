//! Theme token value types reused from `spectrum-theme`.

pub use spectrum_theme::{
    Color, ColorParseError, FontStyle, FontStyleParseError, FontWeight, FontWeightParseError,
    Length, LengthParseError, LengthUnit, LineHeight, LineHeightParseError, Radius,
    RadiusParseError, Rgb, Rgba, ShadowError, ShadowLayer, ThemeBuildError,
};

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
