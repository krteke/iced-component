//! Adwaita-like icon button component.

mod content;
mod source;
mod text_button;

use iced::widget::svg;
use spectrum_theme::iced::IcedColorAdapter;

use crate::context::{UpdateCx, ViewCx};

use super::{
    Button, ButtonContentLayout, ButtonEvent, ButtonSignal, ButtonSnapshot, ButtonStyleOverride,
    ButtonVariant, ButtonView,
};

pub use content::IconTextContent;
pub use source::IconSource;
pub use text_button::IconTextButton;

/// Stateful adwaita-like icon button backed by [`Button`].
#[derive(Debug)]
pub struct IconButton {
    button: Button,
    icon: IconSource,
    icon_size: Option<f32>,
}

impl IconButton {
    /// Creates a standard profile icon button.
    #[must_use]
    pub fn new(icon: impl Into<IconSource>) -> Self {
        Self::from_button(icon, Button::empty())
    }

    /// Creates a suggested-action icon button.
    #[must_use]
    pub fn suggested(icon: impl Into<IconSource>) -> Self {
        Self::from_button(icon, Button::empty().with_variant(ButtonVariant::SUGGESTED))
    }

    /// Creates a destructive-action icon button.
    #[must_use]
    pub fn destructive(icon: impl Into<IconSource>) -> Self {
        Self::from_button(
            icon,
            Button::empty().with_variant(ButtonVariant::DESTRUCTIVE),
        )
    }

    /// Creates an SVG icon button from static in-memory bytes.
    #[must_use]
    pub fn svg_static(bytes: &'static [u8]) -> Self {
        Self::new(IconSource::svg_static(bytes))
    }

    /// Creates an SVG icon button from in-memory bytes.
    #[must_use]
    pub fn svg_bytes(bytes: impl Into<std::borrow::Cow<'static, [u8]>>) -> Self {
        Self::new(IconSource::svg_bytes(bytes))
    }

    /// Creates an SVG icon button from a filesystem path.
    #[must_use]
    pub fn svg_path(path: impl Into<std::path::PathBuf>) -> Self {
        Self::new(IconSource::svg_path(path))
    }

    /// Creates an SVG icon button from an existing Iced SVG handle.
    #[must_use]
    pub fn svg_handle(handle: svg::Handle) -> Self {
        Self::new(IconSource::svg_handle(handle))
    }

    /// Creates a text fallback icon button.
    #[must_use]
    pub fn text(text: impl Into<String>) -> Self {
        Self::new(IconSource::text(text))
    }

    /// Wraps an existing button as an icon button.
    #[must_use]
    pub fn from_button(icon: impl Into<IconSource>, button: Button) -> Self {
        Self {
            button: button.with_content_layout(ButtonContentLayout::Image),
            icon: icon.into(),
            icon_size: None,
        }
    }

    /// Returns this icon button as a flat button.
    #[must_use]
    pub fn flat(mut self) -> Self {
        self.button = self.button.flat();
        self
    }

    /// Returns this icon button with pill shape.
    #[must_use]
    pub fn pill(mut self) -> Self {
        self.button = self.button.pill();
        self
    }

    /// Returns this icon button with circular shape.
    #[must_use]
    pub fn circular(mut self) -> Self {
        self.button = self.button.circular();
        self
    }

    /// Returns this icon button with explicit icon size.
    #[must_use]
    pub const fn icon_size(mut self, size: f32) -> Self {
        self.icon_size = Some(size);
        self
    }

    /// Returns this icon button with a disabled initial state.
    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.button = self.button.disabled(disabled);
        self
    }

    /// Returns this icon button with style overrides.
    #[must_use]
    pub fn with_style_override(mut self, style_override: ButtonStyleOverride) -> Self {
        self.button = self.button.with_style_override(style_override);
        self
    }

    /// Returns this icon button with a different icon source.
    #[must_use]
    pub fn with_icon(mut self, icon: impl Into<IconSource>) -> Self {
        self.icon = icon.into();
        self
    }

    /// Replaces this icon button's icon source.
    pub fn set_icon(&mut self, icon: impl Into<IconSource>) {
        self.icon = icon.into();
    }

    /// Replaces this icon button's icon size override.
    pub fn set_icon_size(&mut self, size: f32) {
        self.icon_size = Some(size);
    }

    /// Clears this icon button's icon size override.
    pub fn clear_icon_size(&mut self) {
        self.icon_size = None;
    }

    /// Registers the inner button motion handle.
    pub fn register(&mut self, cx: &mut UpdateCx<'_>) {
        self.button.register(cx);
    }

    /// Synchronizes the inner button's motion target.
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

    /// Applies a rendered event and reports whether it activated the button.
    pub fn update_event(
        &mut self,
        event: ButtonEvent,
        cx: &mut UpdateCx<'_>,
    ) -> Result<super::ButtonOutcome, iced_component_core::anim::MotionError> {
        self.button.update_event(event, cx)
    }

    /// Enables or disables this icon button.
    pub fn set_disabled(
        &mut self,
        disabled: bool,
        cx: &mut UpdateCx<'_>,
    ) -> Result<bool, iced_component_core::anim::MotionError> {
        self.button.set_disabled(disabled, cx)
    }

    /// Returns a rendering snapshot of the inner button.
    pub fn snapshot(
        &self,
        cx: &ViewCx<'_>,
    ) -> Result<ButtonSnapshot, iced_component_core::anim::MotionError> {
        self.button.snapshot(cx)
    }

    /// Builds an Iced view for this icon button.
    #[must_use]
    pub fn view<'a, Message>(&'a self, cx: &ViewCx<'_>) -> ButtonView<'a, Message>
    where
        Message: Clone + 'a,
    {
        self.try_view(cx)
            .expect("button motion handle belongs to the provided runtime")
    }

    /// Tries to build an Iced view for this icon button.
    pub fn try_view<'a, Message>(
        &'a self,
        cx: &ViewCx<'_>,
    ) -> Result<ButtonView<'a, Message>, iced_component_core::anim::MotionError>
    where
        Message: Clone + 'a,
    {
        let snapshot = self.button.snapshot(cx)?;
        let color = snapshot.style.foreground.color();
        let icon = content::icon_element(&self.icon, self.resolved_icon_size(cx), color);

        Ok(self.button.try_view(cx)?.content(icon))
    }

    /// Returns this icon button source.
    #[must_use]
    pub const fn icon(&self) -> &IconSource {
        &self.icon
    }

    /// Returns the icon size override.
    #[must_use]
    pub const fn icon_size_override(&self) -> Option<f32> {
        self.icon_size
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

    fn resolved_icon_size(&self, cx: &ViewCx<'_>) -> f32 {
        self.icon_size
            .unwrap_or_else(|| cx.theme().pack().button.icon_size.value())
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;
    use iced::{Element, Length};
    use iced_component_core::anim::MotionRuntime;

    use crate::{
        Context,
        button::{Button, ButtonContentLayout, ButtonEvent, ButtonSignal, ButtonStyleState},
        context::{UpdateCx, ViewCx},
    };

    use super::{IconButton, IconSource};

    const TEST_ICON: &[u8] = br#"<svg viewBox="0 0 16 16"><path d="M3 8h10v2H3z"/></svg>"#;

    #[test]
    fn icon_button_marks_inner_button_as_image_content() {
        let icon = IconButton::svg_static(TEST_ICON);

        assert_eq!(icon.button().content_layout(), ButtonContentLayout::Image);
        assert!(matches!(icon.icon(), IconSource::Svg(_)));
    }

    #[test]
    fn icon_button_uses_profile_image_layout_tokens() {
        let context = Context::light();
        let icon = IconButton::svg_static(TEST_ICON);
        let theme = context.theme().pack();
        let (width, height, padding_x, padding_y) = icon.button().resolved_layout(theme);
        let expected_width =
            theme.button.image_min_width.value() + theme.button.image_padding_x.value() * 2.0;
        let expected_height =
            theme.button.min_height.value() + theme.button.padding_y.value() * 2.0;

        assert_eq!(width, Some(Length::Fixed(expected_width)));
        assert_eq!(height, Some(Length::Fixed(expected_height)));
        assert_approx_eq!(f32, padding_x, theme.button.image_padding_x.value());
        assert_approx_eq!(f32, padding_y, theme.button.padding_y.value());
    }

    #[test]
    fn icon_button_delegates_button_events() {
        let mut runtime = MotionRuntime::new();
        let mut context = Context::light();
        let mut icon = IconButton::svg_static(TEST_ICON);

        {
            let mut cx = UpdateCx::new(&mut runtime, &mut context);
            icon.update(ButtonSignal::HoverEnter, &mut cx).unwrap();
            assert_eq!(
                icon.update_event(ButtonEvent::Pressed, &mut cx).unwrap(),
                crate::button::ButtonOutcome::Activated
            );
        }

        let cx = ViewCx::new(&runtime, &context);
        let snapshot = icon.snapshot(&cx).unwrap();
        assert_eq!(snapshot.style_state, ButtonStyleState::Hovered);
    }

    #[test]
    fn icon_button_accepts_existing_button_and_text_fallback() {
        let icon = IconButton::from_button(
            IconSource::text("!"),
            Button::suggested("unused").flat().circular(),
        );

        assert_eq!(icon.icon().text_fallback(), Some("!"));
        assert_eq!(icon.button().content_layout(), ButtonContentLayout::Image);
    }

    #[test]
    fn icon_button_builds_iced_view() {
        #[derive(Clone)]
        enum Message {}

        let runtime = MotionRuntime::new();
        let context = Context::light();
        let cx = ViewCx::new(&runtime, &context);
        let icon = IconButton::svg_static(TEST_ICON);

        let _element: Element<'_, Message> = icon.view(&cx).into();
    }

    #[test]
    fn icon_size_can_be_overridden() {
        let runtime = MotionRuntime::new();
        let context = Context::light();
        let cx = ViewCx::new(&runtime, &context);
        let icon = IconButton::text("!").icon_size(20.0);

        assert_eq!(icon.icon_size_override(), Some(20.0));
        assert_approx_eq!(f32, icon.resolved_icon_size(&cx), 20.0);
    }
}
