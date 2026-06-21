//! Iced integration for animated buttons.

use iced::widget::{button, mouse_area, text};
use iced::{Background, Border, Color, Element, Shadow, Vector};
use spectrum_theme::iced::{IcedColorAdapter, IcedRadiusAdapter, IcedShadowAdapter};

use super::{AnimatedButton, AnimatedButtonSnapshot, ButtonEvent, ButtonInteraction};
use crate::component::ComponentContext;
use crate::{MotionError, MotionRuntime};

/// Iced view builder for [`AnimatedButton`].
pub struct AnimatedButtonView<'a, Message, Action = ()> {
    snapshot: AnimatedButtonSnapshot,
    label: &'a str,
    on_event: Option<Box<dyn Fn(ButtonEvent<Action>) -> Message + 'a>>,
    on_press: Option<Action>,
    padding: [f32; 2],
}

impl AnimatedButton {
    /// Builds an Iced view for this button.
    #[must_use]
    pub fn view<'a, Message>(
        &'a self,
        runtime: &MotionRuntime,
        context: &ComponentContext,
    ) -> AnimatedButtonView<'a, Message>
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
    ) -> Result<AnimatedButtonView<'a, Message>, MotionError>
    where
        Message: Clone + 'a,
    {
        Ok(AnimatedButtonView {
            snapshot: self.snapshot(runtime, context)?,
            label: self.label(),
            on_event: None,
            on_press: None,
            padding: [8.0, 14.0],
        })
    }
}

impl<'a, Message, Action> AnimatedButtonView<'a, Message, Action> {
    /// Maps button events into application messages.
    #[must_use]
    pub fn on_event(mut self, mapper: impl Fn(ButtonEvent<Action>) -> Message + 'a) -> Self {
        self.on_event = Some(Box::new(mapper));
        self
    }

    /// Maps internal button interactions into application messages.
    #[must_use]
    pub fn on_interaction(mut self, mapper: impl Fn(ButtonInteraction) -> Message + 'a) -> Self {
        self.on_event = Some(Box::new(move |event| match event {
            ButtonEvent::Interaction(interaction) => mapper(interaction),
            ButtonEvent::Pressed(_) => mapper(ButtonInteraction::PressUp),
        }));
        self
    }

    /// Sets the application action emitted when the button is released.
    #[must_use]
    pub fn on_press<NextAction>(
        self,
        action: NextAction,
    ) -> AnimatedButtonView<'a, Message, NextAction> {
        AnimatedButtonView {
            snapshot: self.snapshot,
            label: self.label,
            on_event: None,
            on_press: Some(action),
            padding: self.padding,
        }
    }

    /// Sets the application action emitted when the button is released, if any.
    #[must_use]
    pub fn on_press_maybe<NextAction>(
        self,
        action: Option<NextAction>,
    ) -> AnimatedButtonView<'a, Message, NextAction> {
        AnimatedButtonView {
            snapshot: self.snapshot,
            label: self.label,
            on_event: None,
            on_press: action,
            padding: self.padding,
        }
    }

    /// Sets horizontal and vertical padding.
    #[must_use]
    pub const fn padding(mut self, padding: [f32; 2]) -> Self {
        self.padding = padding;
        self
    }
}

impl<'a, Message, Action> From<AnimatedButtonView<'a, Message, Action>> for Element<'a, Message>
where
    Message: Clone + 'a,
    Action: 'a,
{
    fn from(view: AnimatedButtonView<'a, Message, Action>) -> Self {
        let widget = button(text(view.label))
            .padding(view.padding)
            .style(move |_theme, _status| button_style(view.snapshot));

        if view.snapshot.disabled {
            widget.into()
        } else {
            let on_event = view
                .on_event
                .expect("AnimatedButtonView requires on_event for enabled buttons");

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
                .on_release(match view.on_press {
                    Some(action) => on_event(ButtonEvent::Pressed(action)),
                    None => on_event(ButtonEvent::Interaction(ButtonInteraction::PressUp)),
                })
                .into()
        }
    }
}

/// Converts an animated button snapshot into an Iced button style.
#[must_use]
pub fn button_style(snapshot: AnimatedButtonSnapshot) -> button::Style {
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
            width: 1.0 + motion.border_glow,
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
        button::{AnimatedButton, ButtonEvent, ButtonInteraction},
        component::ComponentContext,
    };

    use super::button_style;

    #[test]
    fn button_style_uses_snapshot_motion() {
        let mut runtime = MotionRuntime::new();
        let context = ComponentContext::current();
        let mut button = AnimatedButton::standard("Save");

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
        let button = AnimatedButton::primary("Save");
        let view = button
            .view(&runtime, &context)
            .on_press(Action::Save)
            .on_event(Message::Button);
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
        let mut button = AnimatedButton::primary("Save");

        button
            .update(ButtonInteraction::SetDisabled(true), &mut runtime)
            .unwrap();

        let view = button.view(&runtime, &context);
        let _element: Element<'_, Message> = view.into();
    }
}
