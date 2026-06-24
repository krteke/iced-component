//! View builder for [`Button`] using Iced.

use iced::widget::{button, container, mouse_area, text};
use iced::{Background, Border, Color, Element, Length, Shadow, Vector};
use spectrum_theme::iced::{IcedColorAdapter, IcedRadiusAdapter, IcedShadowAdapter};

use super::{Button, ButtonEvent, ButtonInteraction, ButtonSnapshot};
use crate::component::ComponentContext;
use crate::{MotionError, MotionRuntime};

/// Iced view builder for [`Button`].
pub struct ButtonView<'a, Message, Action = ()> {
    snapshot: ButtonSnapshot,
    content: Element<'a, Message>,
    events: ButtonViewEvents<'a, Message, Action>,
    layout: ResolvedButtonLayout,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ResolvedButtonLayout {
    pub(crate) padding: [f32; 2],
    pub(crate) width: Option<Length>,
    pub(crate) height: Option<Length>,
    pub(crate) center_content: bool,
}

struct ButtonViewEvents<'a, Message, Action = ()> {
    on_event: Option<Box<dyn Fn(ButtonEvent<Action>) -> Message + 'a>>,
    on_press: Option<Action>,
}

impl<'a, Message, Action> ButtonViewEvents<'a, Message, Action> {
    fn new() -> Self {
        Self {
            on_event: None,
            on_press: None,
        }
    }

    fn map_event(mut self, mapper: impl Fn(ButtonEvent<Action>) -> Message + 'a) -> Self {
        self.on_event = Some(Box::new(mapper));
        self
    }

    fn on_press_event<NextAction>(
        action: NextAction,
        mapper: impl Fn(ButtonEvent<NextAction>) -> Message + 'a,
    ) -> ButtonViewEvents<'a, Message, NextAction> {
        ButtonViewEvents {
            on_event: Some(Box::new(mapper)),
            on_press: Some(action),
        }
    }

    fn on_press_maybe<NextAction>(
        action: Option<NextAction>,
    ) -> ButtonViewEvents<'a, Message, NextAction> {
        ButtonViewEvents {
            on_event: None,
            on_press: action,
        }
    }
}

impl Button {
    /// Builds an Iced view for this button.
    #[must_use]
    pub fn view<'a, Message>(
        &'a self,
        runtime: &MotionRuntime,
        context: &ComponentContext,
    ) -> ButtonView<'a, Message>
    where
        Message: Clone + 'a,
    {
        self.try_view(runtime, context)
            .expect("button motion handle belongs to the provided runtime")
    }

    /// Tries to build an Iced view for this button.
    pub fn try_view<'a, Message>(
        &'a self,
        runtime: &MotionRuntime,
        context: &ComponentContext,
    ) -> Result<ButtonView<'a, Message>, MotionError>
    where
        Message: Clone + 'a,
    {
        Ok(ButtonView::from_parts(
            self.snapshot(runtime, context)?,
            button_content(self),
            self.layout.resolve(context),
        ))
    }
}

impl<'a, Message> ButtonView<'a, Message>
where
    Message: 'a,
{
    pub(crate) fn from_parts(
        snapshot: ButtonSnapshot,
        content: Element<'a, Message>,
        layout: ResolvedButtonLayout,
    ) -> Self {
        Self {
            snapshot,
            content,
            events: ButtonViewEvents::new(),
            layout,
        }
    }
}

impl<'a, Message: 'a, Action> ButtonView<'a, Message, Action> {
    /// Maps internal button interactions into application messages.
    #[must_use]
    pub fn on_interaction(mut self, mapper: impl Fn(ButtonInteraction) -> Message + 'a) -> Self {
        self.events = self.events.map_event(move |event| match event {
            ButtonEvent::Interaction(interaction) => mapper(interaction),
            ButtonEvent::Pressed(_) => mapper(ButtonInteraction::PressUp),
        });
        self
    }

    /// Sets the release action and maps all button events into application messages.
    #[must_use]
    pub fn connect<NextAction>(
        self,
        action: NextAction,
        mapper: impl Fn(ButtonEvent<NextAction>) -> Message + 'a,
    ) -> ButtonView<'a, Message, NextAction> {
        ButtonView {
            snapshot: self.snapshot,
            content: self.content,
            events: ButtonViewEvents::<Message, Action>::on_press_event(action, mapper),
            layout: self.layout,
        }
    }

    /// Sets the application action emitted when the button is released, if any.
    #[must_use]
    pub fn on_press_maybe<NextAction>(
        self,
        action: Option<NextAction>,
    ) -> ButtonView<'a, Message, NextAction> {
        ButtonView {
            snapshot: self.snapshot,
            content: self.content,
            events: ButtonViewEvents::<Message, Action>::on_press_maybe(action),
            layout: self.layout,
        }
    }
}

fn button_content<'a, Message>(button: &'a Button) -> Element<'a, Message>
where
    Message: 'a,
{
    button
        .content()
        .as_text()
        .map_or_else(|| text("").into(), |label| text(label).into())
}

impl<'a, Message, Action> From<ButtonView<'a, Message, Action>> for Element<'a, Message>
where
    Message: Clone + 'a,
    Action: 'a,
{
    fn from(view: ButtonView<'a, Message, Action>) -> Self {
        let content = if view.layout.center_content {
            container(view.content).center(Length::Fill).into()
        } else {
            view.content
        };

        let mut widget = button(content)
            .padding(view.layout.padding)
            .style(move |_theme, _status| button_style(view.snapshot));
        if let Some(width) = view.layout.width {
            widget = widget.width(width);
        }
        if let Some(height) = view.layout.height {
            widget = widget.height(height);
        }

        if view.snapshot.disabled {
            widget.into()
        } else {
            let Some(on_event) = view.events.on_event else {
                return widget.into();
            };

            mouse_area(widget)
                .on_enter(on_event(ButtonEvent::Interaction(
                    ButtonInteraction::HoverEnter,
                )))
                .on_exit(on_event(ButtonEvent::Interaction(
                    ButtonInteraction::HoverExit,
                )))
                .on_press(on_event(ButtonEvent::Interaction(
                    ButtonInteraction::PressDown,
                )))
                .on_release(match view.events.on_press {
                    Some(action) => on_event(ButtonEvent::Pressed(action)),
                    None => on_event(ButtonEvent::Interaction(ButtonInteraction::PressUp)),
                })
                .into()
        }
    }
}

/// Converts an animated button snapshot into an Iced button style.
#[must_use]
pub fn button_style(snapshot: ButtonSnapshot) -> button::Style {
    let style = snapshot.style;
    let motion = snapshot.motion;

    let shadow = Shadow {
        offset: Vector::new(
            style.shadow.offset_x().value(),
            motion.shadow_y * style.shadow.offset_y().value(),
        ),
        ..style.shadow.shadow_px()
    };

    button::Style {
        background: Some(Background::Color(color_with_alpha(
            style.background.color(),
            motion.bg_alpha,
        ))),
        text_color: style.foreground.color(),
        border: Border {
            color: if motion.focus_alpha > 0.0 {
                color_with_alpha(style.focus_ring.color(), motion.focus_alpha)
            } else {
                style.border.color()
            },
            width: style.border_width.value() + motion.border_glow,
            radius: style.radius.radius_px(),
        },
        shadow,
        snap: true,
    }
}

fn color_with_alpha(color: Color, alpha_multiplier: f32) -> Color {
    Color {
        a: color.a * alpha_multiplier.clamp(0.0, 1.0),
        ..color
    }
}

#[cfg(test)]
mod tests {
    use aura_anim_core::MotionRuntime;
    use iced::Element;

    use crate::{
        button::{Button, ButtonEvent, ButtonInteraction},
        component::ComponentContext,
    };

    use super::button_style;

    #[test]
    fn button_style_uses_snapshot_motion() {
        let mut runtime = MotionRuntime::new();
        let context = ComponentContext::current();
        let mut button = Button::standard("Save");

        button
            .update(ButtonInteraction::SetDisabled(true), &mut runtime)
            .unwrap();

        let snapshot = button.snapshot(&runtime, &context).unwrap();
        let style = button_style(snapshot);

        let Some(iced::Background::Color(background)) = style.background else {
            panic!("button style should use a solid color background");
        };

        assert!(background.a < 1.0);
        assert!(style.shadow.offset.y.abs() <= f32::EPSILON);
    }

    #[test]
    fn view_builder_accepts_app_press_message() {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        enum Action {
            Save,
        }

        #[derive(Clone)]
        enum Message {
            Button(ButtonEvent<Action>),
        }

        let runtime = MotionRuntime::new();
        let context = ComponentContext::current();
        let button = Button::suggested("Save");
        let view = button
            .view(&runtime, &context)
            .connect(Action::Save, Message::Button);
        let _element: Element<'_, Message> = view.into();

        let Message::Button(event) = Message::Button(ButtonEvent::Pressed(Action::Save));
        assert_eq!(event, ButtonEvent::Pressed(Action::Save));
    }

    #[test]
    fn disabled_view_does_not_require_interaction_mapper() {
        #[derive(Clone)]
        enum Message {}

        let mut runtime = MotionRuntime::new();
        let context = ComponentContext::current();
        let mut button = Button::suggested("Save");

        button
            .update(ButtonInteraction::SetDisabled(true), &mut runtime)
            .unwrap();

        let view = button.view(&runtime, &context);
        let _element: Element<'_, Message> = view.into();
    }

    #[test]
    fn enabled_view_without_event_mapper_renders_static_button() {
        #[derive(Clone)]
        enum Message {}

        let runtime = MotionRuntime::new();
        let context = ComponentContext::current();
        let button = Button::suggested("Save");

        let view = button.view(&runtime, &context);
        let _element: Element<'_, Message> = view.into();
    }
}
