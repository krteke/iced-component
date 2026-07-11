//! Unified adapter for theme-native button implementations.

use iced::{Element, widget::Space};
use iced_component_core::anim::MotionError;

use crate::context::{ThemeFamily, UpdateCx, ViewCx};

pub use iced_component_core::component::button::{ButtonEvent, ButtonOutcome, ButtonSignal};

/// Portable button emphasis understood by every adapter backend.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ButtonStyle {
    /// The theme's ordinary action button.
    #[default]
    Default,
    /// The theme's most prominent action button.
    Primary,
    /// A lower-emphasis filled or neutral action.
    Secondary,
    /// A minimal button without a persistent container.
    Quiet,
}

/// Stateful button backed by both themed component implementations.
///
/// Registering this adapter registers both implementations. Events are applied
/// to both so switching [`ThemeFamily`] is immediate and preserves interaction
/// state without a cross-theme animation.
#[derive(Debug)]
pub struct Button {
    style: ButtonStyle,
    adwaita: iced_adwaita::button::Button,
    material: iced_material::button::Button,
}

impl Button {
    /// Creates an ordinary theme-native button.
    #[must_use]
    pub fn new() -> Self {
        Self::with_style(ButtonStyle::Default)
    }

    /// Creates a prominent action button.
    #[must_use]
    pub fn primary() -> Self {
        Self::with_style(ButtonStyle::Primary)
    }

    /// Creates a lower-emphasis action button.
    #[must_use]
    pub fn secondary() -> Self {
        Self::with_style(ButtonStyle::Secondary)
    }

    /// Creates a minimal action button.
    #[must_use]
    pub fn quiet() -> Self {
        Self::with_style(ButtonStyle::Quiet)
    }

    /// Creates a button with portable style semantics.
    #[must_use]
    pub fn with_style(style: ButtonStyle) -> Self {
        Self {
            style,
            adwaita: adwaita_button(style),
            material: material_button(style),
        }
    }

    /// Returns this button with a disabled initial state.
    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.adwaita = self.adwaita.disabled(disabled);
        self.material = self.material.disabled(disabled);
        self
    }

    /// Registers both themed motion slots in the application runtime.
    pub fn register(&mut self, cx: &mut UpdateCx<'_>) {
        self.adwaita.register(&mut cx.adwaita());
        self.material.register(&mut cx.material());
    }

    /// Synchronizes both themed visual targets with their retained contexts.
    pub fn sync(&mut self, cx: &mut UpdateCx<'_>) -> Result<bool, MotionError> {
        let adwaita = self.adwaita.sync(&mut cx.adwaita())?;
        let material = self.material.sync(&mut cx.material())?;
        Ok(adwaita || material)
    }

    /// Replaces the portable style and synchronizes both implementations.
    pub fn set_style(
        &mut self,
        style: ButtonStyle,
        cx: &mut UpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        if self.style == style {
            return Ok(false);
        }

        self.style = style;
        self.adwaita.set_variant(adwaita_variant(style));
        self.material.set_variant(material_variant(style));
        self.sync(cx)
    }

    /// Enables or disables both themed implementations.
    pub fn set_disabled(
        &mut self,
        disabled: bool,
        cx: &mut UpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        let adwaita = self.adwaita.set_disabled(disabled, &mut cx.adwaita())?;
        let material = self.material.set_disabled(disabled, &mut cx.material())?;
        Ok(adwaita || material)
    }

    /// Applies one rendered event to both implementations.
    pub fn update_event(
        &mut self,
        event: ButtonEvent,
        cx: &mut UpdateCx<'_>,
    ) -> Result<ButtonOutcome, MotionError> {
        let family = cx.family();
        let adwaita = self.adwaita.update_event(event, &mut cx.adwaita())?;
        let material = self.material.update_event(event, &mut cx.material())?;

        Ok(match family {
            ThemeFamily::Adwaita => adwaita,
            ThemeFamily::Material => material,
        })
    }

    /// Builds a view using the currently selected themed backend.
    #[must_use]
    pub fn view<'a, 'cx, Message>(&'a self, cx: &'a ViewCx<'cx>) -> ButtonView<'a, 'cx, Message>
    where
        'cx: 'a,
        Message: Clone + 'a,
    {
        ButtonView {
            button: self,
            cx,
            content: Space::new().into(),
            on_event: None,
        }
    }

    /// Returns the portable style semantics.
    #[must_use]
    pub const fn style(&self) -> ButtonStyle {
        self.style
    }

    /// Returns whether both themed motion slots are registered.
    #[must_use]
    pub fn is_registered(&self) -> bool {
        self.adwaita.is_registered() && self.material.is_registered()
    }

    /// Returns whether this adapter is disabled.
    #[must_use]
    pub const fn is_disabled(&self) -> bool {
        self.adwaita.is_disabled()
    }
}

impl Default for Button {
    fn default() -> Self {
        Self::new()
    }
}

/// Iced view builder for an adapter [`Button`].
pub struct ButtonView<'a, 'cx, Message> {
    button: &'a Button,
    cx: &'a ViewCx<'cx>,
    content: Element<'a, Message>,
    on_event: Option<Box<dyn Fn(ButtonEvent) -> Message + 'a>>,
}

impl<'a, Message> ButtonView<'a, '_, Message>
where
    Message: 'a,
{
    /// Replaces the content rendered by the selected backend.
    #[must_use]
    pub fn content(mut self, content: impl Into<Element<'a, Message>>) -> Self {
        self.content = content.into();
        self
    }

    /// Maps rendered button events into application messages.
    #[must_use]
    pub fn on_event(mut self, mapper: impl Fn(ButtonEvent) -> Message + 'a) -> Self {
        self.on_event = Some(Box::new(mapper));
        self
    }
}

impl<'a, 'cx, Message> From<ButtonView<'a, 'cx, Message>> for Element<'a, Message>
where
    'cx: 'a,
    Message: Clone + 'a,
{
    fn from(view: ButtonView<'a, 'cx, Message>) -> Self {
        match view.cx.family() {
            ThemeFamily::Adwaita => {
                let cx = view.cx.adwaita();
                let button = view.button.adwaita.view(&cx).content(view.content);
                match view.on_event {
                    Some(mapper) => button.on_event(mapper).into(),
                    None => button.into(),
                }
            }
            ThemeFamily::Material => {
                let cx = view.cx.material();
                let button = view.button.material.view(&cx).content(view.content);
                match view.on_event {
                    Some(mapper) => button.on_event(mapper).into(),
                    None => button.into(),
                }
            }
        }
    }
}

fn adwaita_button(style: ButtonStyle) -> iced_adwaita::button::Button {
    iced_adwaita::button::Button::empty()
        .with_content_layout(iced_adwaita::button::ButtonContentLayout::Text)
        .with_variant(adwaita_variant(style))
}

const fn adwaita_variant(style: ButtonStyle) -> iced_adwaita::button::ButtonVariant {
    use iced_adwaita::button::{ButtonTreatment, ButtonVariant};

    match style {
        ButtonStyle::Default | ButtonStyle::Secondary => ButtonVariant::STANDARD,
        ButtonStyle::Primary => ButtonVariant::SUGGESTED,
        ButtonStyle::Quiet => ButtonVariant::STANDARD.with_treatment(ButtonTreatment::Flat),
    }
}

const fn material_button(style: ButtonStyle) -> iced_material::button::Button {
    iced_material::button::Button::with_variant(material_variant(style))
}

const fn material_variant(style: ButtonStyle) -> iced_material::button::ButtonVariant {
    use iced_material::button::ButtonVariant;

    match style {
        ButtonStyle::Default => ButtonVariant::ELEVATED,
        ButtonStyle::Primary => ButtonVariant::FILLED,
        ButtonStyle::Secondary => ButtonVariant::FILLED_TONAL,
        ButtonStyle::Quiet => ButtonVariant::TEXT,
    }
}

#[cfg(test)]
mod tests {
    use iced::{Element, widget::text};
    use iced_component_core::anim::MotionRuntime;

    use super::{Button, ButtonEvent, ButtonStyle};
    use crate::context::{Context, ThemeFamily, UpdateCx, ViewCx};

    #[test]
    fn register_prepares_both_backends_for_direct_switching() {
        let mut runtime = MotionRuntime::new();
        let mut context = Context::default();
        let mut button = Button::primary();

        button.register(&mut UpdateCx::new(&mut runtime, &mut context));

        assert!(button.is_registered());
        assert_eq!(runtime.motion_count(), 2);
    }

    #[test]
    fn interaction_state_is_kept_in_sync_across_backends() {
        let mut runtime = MotionRuntime::new();
        let mut context = Context::default();
        let mut button = Button::new();
        button.register(&mut UpdateCx::new(&mut runtime, &mut context));

        button
            .update_event(
                ButtonEvent::Signal(super::ButtonSignal::HoverEnter),
                &mut UpdateCx::new(&mut runtime, &mut context),
            )
            .unwrap();

        assert_eq!(
            button.adwaita.style_state(),
            iced_adwaita::button::ButtonStyleState::Hovered
        );
        assert_eq!(
            button.material.style_state(),
            iced_material::button::ButtonStyleState::Hover
        );
    }

    #[test]
    fn view_builds_for_each_selected_family() {
        #[derive(Clone)]
        struct Message;

        let mut runtime = MotionRuntime::new();
        let mut context = Context::default();
        let button = Button::with_style(ButtonStyle::Secondary);

        let view = ViewCx::new(&runtime, &context);
        let _: Element<'_, Message> = button
            .view(&view)
            .content(text("Action"))
            .on_event(|_| Message)
            .into();

        UpdateCx::new(&mut runtime, &mut context).set_family(ThemeFamily::Material);
        let view = ViewCx::new(&runtime, &context);
        let _: Element<'_, Message> = button
            .view(&view)
            .content(text("Action"))
            .on_event(|_| Message)
            .into();
    }
}
