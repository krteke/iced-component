use iced::widget::svg;
use std::{borrow::Cow, path::PathBuf};

/// Icon source rendered by Adwaita icon-aware components.
#[derive(Clone, Debug, PartialEq)]
pub enum IconSource {
    /// SVG loaded through an Iced SVG handle.
    Svg(svg::Handle),
    /// Explicit text fallback, useful for tests and icon fonts.
    Text(String),
}

impl IconSource {
    /// Creates an SVG icon source from a filesystem path.
    #[must_use]
    pub fn svg_path(path: impl Into<PathBuf>) -> Self {
        Self::Svg(svg::Handle::from_path(path))
    }

    /// Creates an SVG icon source from static in-memory bytes.
    #[must_use]
    pub fn svg_static(bytes: &'static [u8]) -> Self {
        Self::Svg(svg::Handle::from_memory(bytes))
    }

    /// Creates an SVG icon source from in-memory bytes.
    #[must_use]
    pub fn svg_bytes(bytes: impl Into<Cow<'static, [u8]>>) -> Self {
        Self::Svg(svg::Handle::from_memory(bytes))
    }

    /// Creates an SVG icon source from an existing Iced SVG handle.
    #[must_use]
    pub fn svg_handle(handle: svg::Handle) -> Self {
        Self::Svg(handle)
    }

    /// Returns the SVG handle when this source is SVG-backed.
    #[must_use]
    pub const fn svg_handle_ref(&self) -> Option<&svg::Handle> {
        match self {
            Self::Svg(handle) => Some(handle),
            Self::Text(_) => None,
        }
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
            Self::Text(text) => Some(text.as_str()),
            Self::Svg(_) => None,
        }
    }
}

impl From<svg::Handle> for IconSource {
    fn from(handle: svg::Handle) -> Self {
        Self::svg_handle(handle)
    }
}
