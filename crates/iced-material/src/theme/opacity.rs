use spectrum_theme::{ThemeBuildError, config::TomlThemeSource, source::ThemeValue};

/// A normalized alpha value loaded from a typed theme source.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Opacity(f32);

impl Opacity {
    /// Returns the normalized alpha value.
    #[must_use]
    pub const fn value(self) -> f32 {
        self.0
    }
}

impl ThemeValue<TomlThemeSource> for Opacity {
    fn read(source: &TomlThemeSource, path: &str) -> Result<Self, ThemeBuildError> {
        let value = source.token_text(path)?.parse::<f32>().map_err(|error| {
            ThemeBuildError::InvalidTokenValue {
                path: path.to_owned(),
                message: error.to_string(),
            }
        })?;

        if !(0.0..=1.0).contains(&value) {
            return Err(ThemeBuildError::InvalidTokenValue {
                path: path.to_owned(),
                message: "opacity must be between 0 and 1".to_owned(),
            });
        }

        Ok(Self(value))
    }
}
