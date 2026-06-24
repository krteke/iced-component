use iced::Length;

use crate::{button::view::ResolvedButtonLayout, component::ComponentContext};

/// Stable button content stored by the component.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ButtonContent {
    /// No default content. Callers may still provide per-view custom content.
    Empty,
    /// Text content rendered with Iced text during view construction.
    Text(String),
}

impl ButtonContent {
    /// Creates text content.
    #[must_use]
    pub fn text(text: impl Into<String>) -> Self {
        Self::Text(text.into())
    }

    /// Returns this content as text when text-backed.
    #[must_use]
    pub fn as_text(&self) -> Option<&str> {
        match self {
            Self::Text(text) => Some(text),
            Self::Empty => None,
        }
    }
}

impl From<String> for ButtonContent {
    fn from(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<&str> for ButtonContent {
    fn from(text: &str) -> Self {
        Self::Text(text.to_owned())
    }
}

/// Stable layout configuration stored by the component.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ButtonLayout {
    pub(crate) padding: Option<[f32; 2]>,
    pub(crate) width: Option<Length>,
    pub(crate) height: Option<Length>,
    pub(crate) center_content: bool,
}

impl ButtonLayout {
    pub(crate) fn resolve(&self, context: &ComponentContext) -> ResolvedButtonLayout {
        let button = &context.theme().theme().control.button;

        ResolvedButtonLayout {
            padding: self
                .padding
                .unwrap_or([button.padding_y.value(), button.padding_x.value()]),
            width: self.width,
            height: self.height,
            center_content: self.center_content,
        }
    }

    /// Returns the padding of the button.
    #[must_use]
    pub fn padding(&self) -> Option<[f32; 2]> {
        self.padding
    }

    /// Returns the width of the button.
    #[must_use]
    pub fn width(&self) -> Option<Length> {
        self.width
    }

    /// Returns the height of the button.
    #[must_use]
    pub fn height(&self) -> Option<Length> {
        self.height
    }

    /// Returns whether the content should be centered.
    #[must_use]
    pub fn center_content(&self) -> bool {
        self.center_content
    }
}
