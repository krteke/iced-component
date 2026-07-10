use iced::widget::svg;
use spectrum_theme::iced::IcedColorAdapter;

use crate::context::{UpdateCx, ViewCx};

use super::{
    super::{
        Button, ButtonContentLayout, ButtonEvent, ButtonSignal, ButtonSnapshot,
        ButtonStyleOverride, ButtonVariant, ButtonView,
    },
    IconSource, IconTextContent,
};

/// Stateful icon and text button backed by [`Button`].
#[derive(Debug)]
pub struct IconTextButton {
    button: Button,
    content: IconTextContent,
}

impl IconTextButton {
    /// Creates a standard Adwaita icon and text button.
    #[must_use]
    pub fn new(icon: impl Into<IconSource>, label: impl Into<String>) -> Self {
        Self::from_button(icon, label, Button::empty())
    }

    /// Creates a suggested-action icon and text button.
    #[must_use]
    pub fn suggested(icon: impl Into<IconSource>, label: impl Into<String>) -> Self {
        Self::from_button(
            icon,
            label,
            Button::empty().with_variant(ButtonVariant::SUGGESTED),
        )
    }

    /// Creates a destructive-action icon and text button.
    #[must_use]
    pub fn destructive(icon: impl Into<IconSource>, label: impl Into<String>) -> Self {
        Self::from_button(
            icon,
            label,
            Button::empty().with_variant(ButtonVariant::DESTRUCTIVE),
        )
    }

    /// Creates an SVG-backed icon and text button from static bytes.
    #[must_use]
    pub fn svg_static(bytes: &'static [u8], label: impl Into<String>) -> Self {
        Self::new(IconSource::svg_static(bytes), label)
    }

    /// Creates an SVG-backed icon and text button from in-memory bytes.
    #[must_use]
    pub fn svg_bytes(
        bytes: impl Into<std::borrow::Cow<'static, [u8]>>,
        label: impl Into<String>,
    ) -> Self {
        Self::new(IconSource::svg_bytes(bytes), label)
    }

    /// Creates an SVG-backed icon and text button from a filesystem path.
    #[must_use]
    pub fn svg_path(path: impl Into<std::path::PathBuf>, label: impl Into<String>) -> Self {
        Self::new(IconSource::svg_path(path), label)
    }

    /// Creates an SVG-backed icon and text button from an Iced SVG handle.
    #[must_use]
    pub fn svg_handle(handle: svg::Handle, label: impl Into<String>) -> Self {
        Self::new(IconSource::svg_handle(handle), label)
    }

    /// Creates a text-fallback icon and text button.
    #[must_use]
    pub fn text(icon: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(IconSource::text(icon), label)
    }

    /// Combines existing button configuration with icon and text content.
    #[must_use]
    pub fn from_button(
        icon: impl Into<IconSource>,
        label: impl Into<String>,
        button: Button,
    ) -> Self {
        Self {
            button: button.with_content_layout(ButtonContentLayout::ImageText),
            content: IconTextContent::new(icon, label),
        }
    }

    /// Returns this button with flat treatment.
    #[must_use]
    pub fn flat(mut self) -> Self {
        self.button = self.button.flat();
        self
    }

    /// Returns this button with pill shape.
    #[must_use]
    pub fn pill(mut self) -> Self {
        self.button = self.button.pill();
        self
    }

    /// Returns this button with a disabled initial state.
    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.button = self.button.disabled(disabled);
        self
    }

    /// Returns this button with style overrides.
    #[must_use]
    pub fn with_style_override(mut self, style_override: ButtonStyleOverride) -> Self {
        self.button = self.button.with_style_override(style_override);
        self
    }

    /// Returns this button with an explicit icon size.
    #[must_use]
    pub fn icon_size(mut self, size: f32) -> Self {
        self.content = self.content.icon_size(size);
        self
    }

    /// Returns this button with a different icon source.
    #[must_use]
    pub fn with_icon(mut self, icon: impl Into<IconSource>) -> Self {
        self.content = self.content.with_icon(icon);
        self
    }

    /// Returns this button with different visible text.
    #[must_use]
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.content = self.content.with_label(label);
        self
    }

    /// Replaces the icon source.
    pub fn set_icon(&mut self, icon: impl Into<IconSource>) {
        self.content.set_icon(icon);
    }

    /// Replaces the visible text.
    pub fn set_label(&mut self, label: impl Into<String>) {
        self.content.set_label(label);
    }

    /// Replaces the icon size override.
    pub fn set_icon_size(&mut self, size: f32) {
        self.content.set_icon_size(size);
    }

    /// Clears the icon size override.
    pub fn clear_icon_size(&mut self) {
        self.content.clear_icon_size();
    }

    /// Returns the icon source.
    #[must_use]
    pub const fn icon(&self) -> &IconSource {
        self.content.icon()
    }

    /// Returns the visible text.
    #[must_use]
    pub fn label(&self) -> &str {
        self.content.label()
    }

    /// Returns whether this button renders visible text beside its icon.
    #[must_use]
    pub fn has_label(&self) -> bool {
        self.content.has_label()
    }

    /// Returns the icon size override.
    #[must_use]
    pub const fn icon_size_override(&self) -> Option<f32> {
        self.content.icon_size_override()
    }

    /// Registers the inner button motion handle.
    pub fn register(&mut self, cx: &mut UpdateCx<'_>) {
        self.button.register(cx);
    }

    /// Synchronizes the inner button motion target.
    pub fn sync(
        &mut self,
        cx: &mut UpdateCx<'_>,
    ) -> Result<bool, iced_component_core::anim::MotionError> {
        self.button.sync(cx)
    }

    /// Applies one button signal to the inner button.
    pub fn update(
        &mut self,
        signal: ButtonSignal,
        cx: &mut UpdateCx<'_>,
    ) -> Result<bool, iced_component_core::anim::MotionError> {
        self.button.update(signal, cx)
    }

    /// Applies an event and returns its application action, if any.
    pub fn update_event<Action>(
        &mut self,
        event: ButtonEvent<Action>,
        cx: &mut UpdateCx<'_>,
    ) -> Result<Option<Action>, iced_component_core::anim::MotionError> {
        self.button.update_event(event, cx)
    }

    /// Enables or disables this button.
    pub fn set_disabled(
        &mut self,
        disabled: bool,
        cx: &mut UpdateCx<'_>,
    ) -> Result<bool, iced_component_core::anim::MotionError> {
        self.button.set_disabled(disabled, cx)
    }

    /// Returns the inner button snapshot.
    pub fn snapshot(
        &self,
        cx: &ViewCx<'_>,
    ) -> Result<ButtonSnapshot, iced_component_core::anim::MotionError> {
        self.button.snapshot(cx)
    }

    /// Builds an Iced view for this button.
    #[must_use]
    pub fn view<'a, Message>(&'a self, cx: &ViewCx<'_>) -> ButtonView<'a, Message>
    where
        Message: Clone + 'a,
    {
        self.try_view(cx)
            .expect("button motion handle belongs to the provided runtime")
    }

    /// Tries to build an Iced view for this button.
    pub fn try_view<'a, Message>(
        &'a self,
        cx: &ViewCx<'_>,
    ) -> Result<ButtonView<'a, Message>, iced_component_core::anim::MotionError>
    where
        Message: Clone + 'a,
    {
        let snapshot = self.button.snapshot(cx)?;
        let content = self.content.element(cx, snapshot.style.foreground.color());

        Ok(self.button.try_view(cx)?.content(content))
    }

    /// Returns the icon and text content.
    #[must_use]
    pub const fn content(&self) -> &IconTextContent {
        &self.content
    }

    /// Returns the inner button.
    #[must_use]
    pub const fn button(&self) -> &Button {
        &self.button
    }

    /// Returns the mutable inner button.
    pub fn button_mut(&mut self) -> &mut Button {
        &mut self.button
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;
    use iced::{Element, Length};
    use iced_component_core::anim::MotionRuntime;

    use crate::{
        Context,
        button::{ButtonContentLayout, ButtonEvent, ButtonSignal, ButtonStyleState},
        context::{UpdateCx, ViewCx},
    };

    use super::IconTextButton;

    const TEST_ICON: &[u8] = br#"<svg viewBox="0 0 16 16"><path d="M3 8h10v2H3z"/></svg>"#;

    #[test]
    fn icon_text_button_marks_inner_button_with_image_text_layout() {
        let button = IconTextButton::svg_static(TEST_ICON, "Open");

        assert_eq!(
            button.button().content_layout(),
            ButtonContentLayout::ImageText
        );
        assert_eq!(button.content().label(), "Open");
        assert!(button.has_label());
    }

    #[test]
    fn empty_label_is_not_rendered() {
        let button = IconTextButton::svg_static(TEST_ICON, "");

        assert!(!button.has_label());
    }

    #[test]
    fn icon_text_button_uses_libadwaita_image_text_padding() {
        let context = Context::light();
        let button = IconTextButton::svg_static(TEST_ICON, "Open");
        let (width, height, padding_x, padding_y) =
            button.button().resolved_layout(context.theme().pack());

        assert_eq!(width, None);
        assert_eq!(height, Some(Length::Fixed(34.0)));
        assert_approx_eq!(f32, padding_x, 9.0);
        assert_approx_eq!(f32, padding_y, 5.0);
    }

    #[test]
    fn icon_text_button_delegates_button_events() {
        let mut runtime = MotionRuntime::new();
        let mut context = Context::light();
        let mut button = IconTextButton::svg_static(TEST_ICON, "Open");

        {
            let mut cx = UpdateCx::new(&mut runtime, &mut context);
            button.update(ButtonSignal::HoverEnter, &mut cx).unwrap();
            assert_eq!(
                button
                    .update_event(ButtonEvent::Pressed("open"), &mut cx)
                    .unwrap(),
                Some("open")
            );
        }

        let cx = ViewCx::new(&runtime, &context);
        assert_eq!(
            button.snapshot(&cx).unwrap().style_state,
            ButtonStyleState::Hovered
        );
    }

    #[test]
    fn icon_text_button_builds_iced_view() {
        #[derive(Clone)]
        enum Message {}

        let runtime = MotionRuntime::new();
        let context = Context::light();
        let cx = ViewCx::new(&runtime, &context);
        let button = IconTextButton::svg_static(TEST_ICON, "Open");

        let _element: Element<'_, Message> = button.view(&cx).into();
    }
}
