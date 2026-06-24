use std::borrow::Cow;
use std::path::PathBuf;

/// Icon source rendered by icon-aware components.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IconSource {
    /// SVG loaded from a filesystem path.
    SvgPath(PathBuf),
    /// SVG loaded from static in-memory bytes.
    SvgBytes(Cow<'static, [u8]>),
    /// Explicit text fallback, useful for tests or icon fonts.
    Text(String),
}

impl IconSource {
    /// Creates an SVG icon source from a filesystem path.
    #[must_use]
    pub fn svg_path(path: impl Into<PathBuf>) -> Self {
        Self::SvgPath(path.into())
    }

    /// Creates an SVG icon source from static in-memory bytes.
    ///
    /// If the bytes are owned, they will be leaked.
    /// Input `&'static [u8]` -> `Cow::Borrowed`.
    /// Input `Vec<u8>` -> `Cow::Owned` -> leaked -> `Cow::Borrowed`
    #[must_use]
    pub fn svg_bytes(bytes: impl Into<Cow<'static, [u8]>>) -> Self {
        let bytes = bytes.into();

        match bytes {
            Cow::Borrowed(_) => Self::SvgBytes(bytes),
            Cow::Owned(bytes) => {
                let leaked = Box::leak(bytes.into());
                Self::SvgBytes(Cow::Borrowed(leaked))
            }
        }
    }

    /// Creates an SVG icon source from static in-memory bytes.
    #[must_use]
    pub fn from_static(bytes: &'static [u8]) -> Self {
        Self::SvgBytes(Cow::Borrowed(bytes))
    }

    /// Creates an SVG icon source from a `Vec` of bytes.
    ///
    /// The bytes will be leaked.
    #[must_use]
    pub fn from_vec_leak(bytes: Vec<u8>) -> Self {
        let leaked = Box::leak(bytes.into());
        Self::SvgBytes(Cow::Borrowed(leaked))
    }

    /// Creates an SVG icon source from a `Vec` of bytes.
    ///
    /// However, using it may cause performance issues in [`crate::button::IconButton::view`].
    #[must_use]
    pub fn from_vec(bytes: Vec<u8>) -> Self {
        Self::SvgBytes(Cow::Owned(bytes))
    }

    /// Creates an explicit text fallback icon source.
    #[must_use]
    pub fn text(text: impl Into<String>) -> Self {
        Self::Text(text.into())
    }

    /// Returns fallback text when this source is text-backed.
    #[must_use]
    pub fn text_fallback(&self) -> Option<&str> {
        match self {
            Self::Text(text) => Some(text),
            Self::SvgPath(_) | Self::SvgBytes(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IconSource;

    const TEST_ICON: &[u8] = br#"<svg viewBox="0 0 16 16"></svg>"#;

    #[test]
    fn svg_bytes_preserves_source_kind() {
        assert!(matches!(
            IconSource::svg_bytes(TEST_ICON),
            IconSource::SvgBytes(_)
        ));
    }

    #[test]
    fn text_source_is_explicit_fallback() {
        assert_eq!(IconSource::text("!").text_fallback(), Some("!"));
    }
}
