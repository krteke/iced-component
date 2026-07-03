//! Built-in Adwaita-like animation providers.

use aura_anim::{
    core::traits::BoxAnimation,
    prelude::{AnimationExt, Timing, Tween},
};
use std::sync::Arc;

use crate::button::{
    ButtonAnimationBuilder, ButtonAnimationProvider, ButtonMotion, ButtonMotionTransition,
    ButtonMotionTrigger,
};

/// Default Adwaita-like button animation provider.
#[derive(Clone, Copy, Debug, Default)]
pub struct AdwaitaButtonAnimationProvider;

impl ButtonAnimationProvider for AdwaitaButtonAnimationProvider {
    fn button_animation(&self, transition: &ButtonMotionTransition) -> ButtonAnimationBuilder {
        Arc::new(tween(match transition.trigger {
            ButtonMotionTrigger::HoverEnter => 150.0,
            ButtonMotionTrigger::HoverExit => 180.0,
            ButtonMotionTrigger::PressDown => 90.0,
            ButtonMotionTrigger::PressUp | ButtonMotionTrigger::Enable => 160.0,
            ButtonMotionTrigger::Focus
            | ButtonMotionTrigger::Blur
            | ButtonMotionTrigger::Disable => 120.0,
            ButtonMotionTrigger::Sync => 200.0,
        }))
    }
}

fn tween(ms: f32) -> impl Fn(ButtonMotionTransition) -> BoxAnimation<ButtonMotion> + 'static {
    move |transition| Tween::between(transition.from, transition.to, Timing::ease_out(ms)).boxed()
}
