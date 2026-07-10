use crate::context::ViewCx;
use iced::{
    Color, Element, Font, Length, Padding, alignment,
    font::Weight,
    widget::{container, row, svg, text},
};

use super::IconSource;

/// Icon and text content matching libadwaita's `AdwButtonContent` inside a button.
#[derive(Clone, Debug)]
pub struct IconTextContent {
    icon: IconSource,
    label: String,
    icon_size: Option<f32>,
}

impl IconTextContent {
    /// Creates icon and text content for an Adwaita button.
    #[must_use]
    pub fn new(icon: impl Into<IconSource>, label: impl Into<String>) -> Self {
        Self {
            icon: icon.into(),
            label: label.into(),
            icon_size: None,
        }
    }

    /// Returns this content with an explicit icon size.
    #[must_use]
    pub const fn icon_size(mut self, size: f32) -> Self {
        self.icon_size = Some(size);
        self
    }

    /// Returns this content with a different icon source.
    #[must_use]
    pub fn with_icon(mut self, icon: impl Into<IconSource>) -> Self {
        self.set_icon(icon);
        self
    }

    /// Returns this content with different visible text.
    #[must_use]
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.set_label(label);
        self
    }

    /// Replaces the icon source.
    pub fn set_icon(&mut self, icon: impl Into<IconSource>) {
        self.icon = icon.into();
    }

    /// Replaces the visible text.
    pub fn set_label(&mut self, label: impl Into<String>) {
        self.label = label.into();
    }

    /// Replaces the icon size override.
    pub fn set_icon_size(&mut self, size: f32) {
        self.icon_size = Some(size);
    }

    /// Clears the icon size override.
    pub fn clear_icon_size(&mut self) {
        self.icon_size = None;
    }

    /// Returns the icon source.
    #[must_use]
    pub const fn icon(&self) -> &IconSource {
        &self.icon
    }

    /// Returns the visible text.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns whether this content will render visible text.
    #[must_use]
    pub fn has_label(&self) -> bool {
        !self.label.is_empty()
    }

    /// Returns the icon size override.
    #[must_use]
    pub const fn icon_size_override(&self) -> Option<f32> {
        self.icon_size
    }

    pub(crate) fn element<'a, Message>(
        &'a self,
        cx: &ViewCx<'_>,
        color: Color,
    ) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let tokens = &cx.theme().pack().button;
        let icon = icon_element(
            &self.icon,
            self.icon_size.unwrap_or_else(|| tokens.icon_size.value()),
            color,
        );

        if !self.has_label() {
            return icon;
        }

        let label = container(
            text(&self.label)
                .font(Font {
                    weight: Weight::Bold,
                    ..Font::DEFAULT
                })
                .color(color),
        )
        .padding(Padding::from(tokens.image_text_label_padding_x.value()));

        row![icon, label,]
            .spacing(tokens.image_text_spacing.value())
            .align_y(alignment::Vertical::Center)
            .into()
    }
}

pub(crate) fn icon_element<'a, Message>(
    icon: &'a IconSource,
    size: f32,
    color: Color,
) -> Element<'a, Message>
where
    Message: 'a,
{
    match icon {
        IconSource::Svg(handle) => svg(handle.clone())
            .width(Length::Fixed(size))
            .height(Length::Fixed(size))
            .style(move |_theme: &iced::Theme, _status| svg::Style { color: Some(color) })
            .into(),
        IconSource::Text(icon) => text(icon).size(size).color(color).into(),
    }
}
