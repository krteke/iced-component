/// Stable content stored by an Adwaita button.
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

/// Adwaita button content layout class.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ButtonContentLayout {
    /// Base button layout, matching a button without content-specific classes.
    #[default]
    Plain,
    /// Text button layout, matching `.text-button`.
    Text,
    /// Image button layout, matching `.image-button`.
    Image,
    /// Image and text layout, matching `.image-text-button`.
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
