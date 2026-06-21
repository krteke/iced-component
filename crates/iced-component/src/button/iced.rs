//! Iced integration for animated buttons.

use iced::widget::{button, mouse_area, text};
use iced::{Background, Border, Color, Element, Shadow, Vector};
use spectrum_theme::iced::{IcedColorAdapter, IcedRadiusAdapter, IcedShadowAdapter};

use super::{AnimatedButton, AnimatedButtonSnapshot, ButtonInteraction};
use crate::component::ComponentContext;
use crate::{MotionError, MotionRuntime};

/// Iced view builder for [`AnimatedButton`].
pub struct AnimatedButtonView<'a, Message> {
    snapshot: AnimatedButtonSnapshot,
    label: &'a str,
    on_press: Option<Message>,
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
        Message: From<ButtonInteraction> + Clone + 'a,
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
        Message: From<ButtonInteraction> + Clone + 'a,
    {
        Ok(AnimatedButtonView {
            snapshot: self.snapshot(runtime, context)?,
            label: self.label(),
            on_press: None,
            padding: [8.0, 14.0],
        })
    }
}

impl<Message> AnimatedButtonView<'_, Message> {
    /// Sets the application message emitted when the button is released.
    #[must_use]
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Sets the application message emitted when the button is released, if any.
    #[must_use]
    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.on_press = message;
        self
    }

    /// Sets horizontal and vertical padding.
    #[must_use]
    pub const fn padding(mut self, padding: [f32; 2]) -> Self {
        self.padding = padding;
        self
    }
}

impl<'a, Message> From<AnimatedButtonView<'a, Message>> for Element<'a, Message>
where
    Message: From<ButtonInteraction> + Clone + 'a,
{
    fn from(view: AnimatedButtonView<'a, Message>) -> Self {
        let widget = button(text(view.label))
            .padding(view.padding)
            .style(move |_theme, _status| button_style(view.snapshot));

        if view.snapshot.disabled {
            widget.into()
        } else {
            mouse_area(widget)
                .on_enter(ButtonInteraction::HoverEnter.into())
                .on_exit(ButtonInteraction::HoverExit.into())
                .on_press(ButtonInteraction::PressDown.into())
                .on_release(
                    view.on_press
                        .unwrap_or_else(|| ButtonInteraction::PressUp.into()),
                )
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
        button::{AnimatedButton, ButtonInteraction},
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
        #[derive(Clone)]
        enum Message {
            Interaction,
            Save,
        }

        impl From<ButtonInteraction> for Message {
            fn from(_interaction: ButtonInteraction) -> Self {
                Self::Interaction
            }
        }

        let runtime = MotionRuntime::new();
        let context = ComponentContext::current();
        let button = AnimatedButton::primary("Save");
        let view = button.view(&runtime, &context).on_press(Message::Save);
        let _element: Element<'_, Message> = view.into();
    }
}
