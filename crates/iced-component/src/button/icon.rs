//! Icon-style animated button component.

mod source;

use aura_anim::prelude::{MotionError, MotionRuntime};
use iced::{Length, widget::svg};
use spectrum_theme::iced::IcedColorAdapter;
use std::borrow::Cow;
use std::path::PathBuf;

pub use source::IconSource;

use crate::{
    button::{
        Button, ButtonEvent, ButtonInteraction, ButtonRole, ButtonShape, ButtonSnapshot,
        ButtonTreatment, ButtonVariant, ButtonView, view::ResolvedButtonLayout,
    },
    component::{ComponentUpdateCx, ComponentViewCx},
};

/// Icon button control size.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IconButtonSize {
    /// Theme default icon button size.
    Default,
    /// Explicit fixed square size in pixels.
    Fixed(f32),
}

/// Icon-style animated button backed by [`Button`].
#[derive(Debug)]
pub struct IconButton {
    button: Button,
    icon: IconSource,
    size: IconButtonSize,
}

impl IconButton {
    /// Creates a standard SVG icon-style button from in-memory bytes.
    #[must_use]
    pub fn svg_bytes(bytes: impl Into<Cow<'static, [u8]>>) -> Self {
        Self::standard(IconSource::svg_bytes(bytes))
    }

    /// Creates a standard SVG icon-style button from static bytes.
    #[must_use]
    pub fn svg_static(bytes: &'static [u8]) -> Self {
        Self::standard(IconSource::svg_static(bytes))
    }

    /// Creates a standard SVG icon-style button from a filesystem path.
    #[must_use]
    pub fn svg_path(path: impl Into<PathBuf>) -> Self {
        Self::standard(IconSource::svg_path(path))
    }

    /// Creates a standard SVG icon-style button from an existing Iced SVG handle.
    #[must_use]
    pub fn svg_handle(handle: svg::Handle) -> Self {
        Self::standard(IconSource::svg_handle(handle))
    }

    /// Creates a standard text-fallback icon-style button.
    #[must_use]
    pub fn text(text: impl Into<String>) -> Self {
        Self::standard(IconSource::text(text))
    }

    /// Creates a standard icon-style button.
    #[must_use]
    pub fn new(icon: impl Into<IconSource>) -> Self {
        Self::standard(icon)
    }

    /// Creates a standard icon-style button.
    #[must_use]
    pub fn standard(icon: impl Into<IconSource>) -> Self {
        Self::from_button(icon, Button::empty(ButtonVariant::STANDARD))
    }

    /// Creates a suggested-action icon-style button.
    #[must_use]
    pub fn suggested(icon: impl Into<IconSource>) -> Self {
        Self::from_button(icon, Button::empty(ButtonVariant::SUGGESTED))
    }

    /// Creates a destructive-action icon-style button.
    #[must_use]
    pub fn destructive(icon: impl Into<IconSource>) -> Self {
        Self::from_button(icon, Button::empty(ButtonVariant::DESTRUCTIVE))
    }

    /// Wraps an existing animated button as an icon-style button.
    #[must_use]
    pub fn from_button(icon: impl Into<IconSource>, button: Button) -> Self {
        Self {
            button,
            icon: icon.into(),
            size: IconButtonSize::Default,
        }
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

    /// Returns this icon button with a different inner button.
    #[must_use]
    pub fn with_button(mut self, button: Button) -> Self {
        self.button = button;
        self
    }

    /// Replaces this icon button's inner button.
    pub fn set_button(&mut self, button: Button) {
        self.button = button;
    }

    /// Updates this icon button's visual variant.
    pub fn set_variant(&mut self, variant: ButtonVariant) {
        self.button.set_variant(variant);
    }

    /// Updates this icon button's semantic role.
    pub fn set_role(&mut self, role: ButtonRole) {
        self.button.set_role(role);
    }

    /// Updates this icon button's visual treatment.
    pub fn set_treatment(&mut self, treatment: ButtonTreatment) {
        self.button.set_treatment(treatment);
    }

    /// Updates this icon button's outline shape.
    pub fn set_shape(&mut self, shape: ButtonShape) {
        self.button.set_shape(shape);
    }

    /// Returns this icon button as a standard action.
    #[must_use]
    pub fn as_standard(mut self) -> Self {
        self.button = self.button.as_standard();
        self
    }

    /// Returns this icon button as a suggested action.
    #[must_use]
    pub fn as_suggested(mut self) -> Self {
        self.button = self.button.as_suggested();
        self
    }

    /// Returns this icon button as a destructive action.
    #[must_use]
    pub fn as_destructive(mut self) -> Self {
        self.button = self.button.as_destructive();
        self
    }

    /// Returns this icon button with filled treatment.
    #[must_use]
    pub fn filled(mut self) -> Self {
        self.button = self.button.filled();
        self
    }

    /// Returns this icon button with flat treatment.
    #[must_use]
    pub fn flat(mut self) -> Self {
        self.button = self.button.flat();
        self
    }

    /// Returns this icon button with raised treatment.
    #[must_use]
    pub fn raised(mut self) -> Self {
        self.button = self.button.raised();
        self
    }

    /// Returns this icon button with rounded rectangle shape.
    #[must_use]
    pub fn rounded(mut self) -> Self {
        self.button = self.button.rounded();
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

    /// Returns this icon button with explicit square size in pixels.
    #[must_use]
    pub const fn with_size(self, size: f32) -> Self {
        self.size(size)
    }

    /// Returns this icon button with explicit square size in pixels.
    #[must_use]
    pub const fn size(mut self, size: f32) -> Self {
        self.size = IconButtonSize::Fixed(size);
        self
    }

    /// Updates this icon button's explicit square size in pixels.
    pub fn set_size(&mut self, size: f32) {
        self.size = IconButtonSize::Fixed(size);
    }

    /// Restores theme default icon button sizing.
    pub fn clear_size(&mut self) {
        self.size = IconButtonSize::Default;
    }

    /// Returns this icon button with disabled state preconfigured.
    #[must_use]
    pub fn with_disabled(self, disabled: bool) -> Self {
        self.disabled(disabled)
    }

    /// Returns this icon button with disabled state preconfigured.
    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.button = self.button.disabled(disabled);
        self
    }

    /// Registers the inner button motion handle in the application runtime.
    pub fn register(&mut self, runtime: &mut MotionRuntime) {
        self.button.register(runtime);
    }

    /// Synchronizes the inner button's current motion target with the runtime.
    pub fn sync(&mut self, cx: &mut ComponentUpdateCx<'_>) -> Result<bool, MotionError> {
        self.button.sync(cx)
    }

    /// Applies an interaction to the inner button.
    pub fn update(
        &mut self,
        interaction: ButtonInteraction,
        cx: &mut ComponentUpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        self.button.update(interaction, cx)
    }

    /// Applies a button event and returns its application action, if any.
    pub fn update_event<Action>(
        &mut self,
        event: ButtonEvent<Action>,
        cx: &mut ComponentUpdateCx<'_>,
    ) -> Result<Option<Action>, MotionError> {
        self.button.update_event(event, cx)
    }

    /// Applies a button event and invokes `on_action` when release yields an action.
    pub fn update_event_with<Action>(
        &mut self,
        event: ButtonEvent<Action>,
        cx: &mut ComponentUpdateCx<'_>,
        on_action: impl FnOnce(Action),
    ) -> Result<bool, MotionError> {
        self.button.update_event_with(event, cx, on_action)
    }

    /// Enables or disables this icon button and updates its motion target.
    pub fn set_disabled(
        &mut self,
        disabled: bool,
        cx: &mut ComponentUpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        self.button.set_disabled(disabled, cx)
    }

    /// Returns a rendering snapshot of the inner button.
    pub fn snapshot(&self, cx: &ComponentViewCx<'_>) -> Result<ButtonSnapshot, MotionError> {
        self.button.snapshot(cx)
    }

    /// Returns this icon button visual variant.
    #[must_use]
    pub const fn variant(&self) -> ButtonVariant {
        self.button.variant()
    }

    /// Returns this icon button source.
    #[must_use]
    pub const fn icon(&self) -> &IconSource {
        &self.icon
    }

    /// Returns this icon button size mode.
    #[must_use]
    pub const fn size_mode(&self) -> IconButtonSize {
        self.size
    }

    /// Returns the inner animated button.
    #[must_use]
    pub const fn as_button(&self) -> &Button {
        &self.button
    }

    /// Returns the mutable inner animated button.
    pub fn as_button_mut(&mut self) -> &mut Button {
        &mut self.button
    }

    /// Consumes this icon button and returns the inner animated button.
    #[must_use]
    pub fn into_button(self) -> Button {
        self.button
    }

    /// Returns whether this icon button is disabled.
    #[must_use]
    pub const fn is_disabled(&self) -> bool {
        self.button.is_disabled()
    }

    /// Returns whether this icon button is focused.
    #[must_use]
    pub const fn is_focused(&self) -> bool {
        self.button.is_focused()
    }
}

impl IconButton {
    /// Builds an Iced view for this icon button.
    #[must_use]
    pub fn view<'a, Message>(&'a self, cx: &ComponentViewCx<'_>) -> ButtonView<'a, Message>
    where
        Message: Clone + 'a,
    {
        let metrics = &cx.context().theme().theme().control.icon_button;
        let size = match self.size {
            IconButtonSize::Default => metrics.size.value(),
            IconButtonSize::Fixed(size) => size,
        };
        let snapshot = self
            .button
            .snapshot(cx)
            .expect("button motion handle belongs to the provided runtime");
        let icon = self.icon_element(metrics.icon_size.value(), snapshot.style.foreground.color());

        ButtonView::from_parts(
            snapshot,
            icon,
            ResolvedButtonLayout {
                padding: [0.0, 0.0],
                width: Some(Length::Fixed(size)),
                height: Some(Length::Fixed(size)),
                center_content: true,
            },
        )
    }

    fn icon_element<'a, Message>(
        &'a self,
        size: f32,
        color: iced::Color,
    ) -> iced::Element<'a, Message>
    where
        Message: 'a,
    {
        match &self.icon {
            IconSource::Svg(handle) => iced::widget::svg(handle.clone())
                .width(iced::Length::Fixed(size))
                .height(iced::Length::Fixed(size))
                .style(
                    move |_theme: &iced::Theme, _status| iced::widget::svg::Style {
                        color: Some(color),
                    },
                )
                .into(),
            IconSource::Text(text) => iced::widget::text(text).size(size).into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use aura_anim::prelude::*;

    use crate::{
        button::{
            Button, ButtonEvent, ButtonInteraction, ButtonRole, ButtonShape, ButtonStyleState,
            ButtonTreatment, ButtonVariant,
            icon::{IconButton, IconButtonSize, source::IconSource},
        },
        component::{ComponentContext, ComponentUpdateCx, ComponentViewCx},
    };

    const TEST_ICON: &[u8] = br#"<svg viewBox="0 0 16 16"><path d="M3 8h10v2H3z"/></svg>"#;

    #[test]
    fn icon_button_wraps_button_without_forcing_shape() {
        let icon = IconButton::from_button(
            IconSource::svg_bytes(TEST_ICON),
            Button::empty(ButtonVariant::SUGGESTED),
        );

        assert_eq!(icon.variant(), ButtonVariant::SUGGESTED);
        assert!(matches!(icon.icon(), IconSource::Svg(_)));
    }

    #[test]
    fn standard_icon_button_defaults_to_filled_rounded() {
        let icon = IconButton::svg_bytes(TEST_ICON);

        assert_eq!(icon.variant(), ButtonVariant::STANDARD);
    }

    #[test]
    fn icon_button_delegates_interaction_state() {
        let mut runtime = MotionRuntime::new();
        let mut context = ComponentContext::adwaita();
        let mut icon = IconButton::svg_bytes(TEST_ICON);

        {
            let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
            icon.update(ButtonInteraction::HoverEnter, &mut cx).unwrap();
        }

        let cx = ComponentViewCx::new(&runtime, &context);
        let snapshot = icon.snapshot(&cx).unwrap();
        assert_eq!(snapshot.style_state, ButtonStyleState::Hovered);
    }

    #[test]
    fn icon_button_delegates_press_events() {
        let mut runtime = MotionRuntime::new();
        let mut context = ComponentContext::adwaita();
        let mut icon = IconButton::svg_bytes(TEST_ICON);

        let action = {
            let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
            icon.update_event(ButtonEvent::Pressed("open"), &mut cx)
                .unwrap()
        };

        assert_eq!(action, Some("open"));
    }

    #[test]
    fn icon_button_can_change_source() {
        let icon = IconButton::svg_bytes(TEST_ICON).with_icon(IconSource::text("!"));

        assert_eq!(icon.icon().text_fallback(), Some("!"));
    }

    #[test]
    fn icon_button_setters_update_stable_config() {
        let mut icon = IconButton::svg_bytes(TEST_ICON)
            .with_size(42.0)
            .with_disabled(true);

        assert!(icon.is_disabled());
        assert_eq!(icon.size_mode(), IconButtonSize::Fixed(42.0));

        icon.set_icon(IconSource::text("?"));
        icon.set_role(ButtonRole::Destructive);
        icon.set_treatment(ButtonTreatment::Raised);
        icon.set_shape(ButtonShape::Pill);
        icon.set_size(36.0);

        assert_eq!(icon.icon().text_fallback(), Some("?"));
        assert_eq!(
            icon.variant(),
            ButtonVariant::DESTRUCTIVE.set_raised().set_pill()
        );
        assert_eq!(icon.size_mode(), IconButtonSize::Fixed(36.0));

        icon.clear_size();
        icon.set_button(Button::suggested("unused").flat());
        icon.as_button_mut().set_shape(ButtonShape::Circular);

        assert_eq!(icon.size_mode(), IconButtonSize::Default);
        assert_eq!(
            icon.variant(),
            ButtonVariant::SUGGESTED.set_flat().set_circular()
        );
    }

    #[test]
    fn icon_button_accepts_existing_svg_handle() {
        let handle = iced::widget::svg::Handle::from_memory(TEST_ICON);
        let icon = IconButton::svg_handle(handle.clone());

        assert_eq!(icon.icon().svg_handle_ref(), Some(&handle));
    }

    #[test]
    fn icon_button_builds_iced_view_from_shared_button_builder() {
        use iced::Element;

        let runtime = MotionRuntime::new();
        let context = ComponentContext::adwaita();
        let cx = ComponentViewCx::new(&runtime, &context);
        let icon = IconButton::svg_bytes(TEST_ICON);

        let view = icon.view(&cx).connect((), |_| ());
        let _element: Element<'_, ()> = view.into();
    }
}
