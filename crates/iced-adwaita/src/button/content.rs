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
    /// Returns this content as text when available.
    #[must_use]
    pub fn as_text(&self) -> Option<&str> {
        match self {
            Self::Empty => None,
            Self::Text(text) => Some(text.as_str()),
        }
    }
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
