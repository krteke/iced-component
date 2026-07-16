/// Stable content stored by an adwaita-like button.
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub enum ButtonContent {
    /// No stored content.
    #[default]
    Empty,
    /// Text content.
    Text(String),
}

impl ButtonContent {
    pub(crate) const fn default_layout(&self) -> ButtonContentLayout {
        match self {
            Self::Empty => ButtonContentLayout::Plain,
            Self::Text(_) => ButtonContentLayout::Text,
        }
    }

    /// Returns this content as text when available.
    #[must_use]
    pub fn as_text(&self) -> Option<&str> {
        match self {
            Self::Empty => None,
            Self::Text(text) => Some(text.as_str()),
        }
    }
}

/// Content layout category used by the profile button metrics.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ButtonContentLayout {
    /// Base button layout without content-specific spacing adjustments.
    #[default]
    Plain,
    /// Text button layout.
    Text,
    /// Image button layout.
    Image,
    /// Image and text layout.
    ImageText,
}

impl From<&str> for ButtonContent {
    fn from(value: &str) -> Self {
        Self::Text(value.to_owned())
    }
}

impl From<String> for ButtonContent {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}
