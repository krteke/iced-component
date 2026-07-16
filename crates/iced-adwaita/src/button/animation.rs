use std::sync::Arc;

use aura_anim::{
    core::traits::BoxAnimation,
    prelude::{AnimationExt, Easing, Timing, Tween},
};
use iced_component_core::component::animation::AnimationOverrides;

use crate::button::{ButtonMotion, ButtonMotionTransition, ButtonSignal};

const DEFAULT_PROFILE_BUTTON_DURATION_MS: f32 = 180.0;

/// A function that builds one button animation from a resolved transition.
pub type ButtonAnimationBuilder = Arc<dyn Fn(ButtonMotionTransition) -> BoxAnimation<ButtonMotion>>;

/// Runtime-configurable adwaita-like button animations.
#[derive(Clone)]
pub struct ButtonAnimations {
    interaction: ButtonAnimationBuilder,
    sync: ButtonAnimationBuilder,
}

impl ButtonAnimations {
    /// Creates button animations from explicit builders.
    #[must_use]
    pub fn new(
        interaction: impl Fn(ButtonMotionTransition) -> BoxAnimation<ButtonMotion> + 'static,
        sync: impl Fn(ButtonMotionTransition) -> BoxAnimation<ButtonMotion> + 'static,
    ) -> Self {
        Self {
            interaction: Arc::new(interaction),
            sync: Arc::new(sync),
        }
    }

    /// Creates tween-based animations for interaction and synchronization.
    #[must_use]
    pub fn tween(interaction: Timing, sync: Timing) -> Self {
        Self::new(tween_builder(interaction), tween_builder(sync))
    }

    /// Replaces the interaction animation with a tween.
    #[must_use]
    pub fn with_interaction_tween(mut self, timing: Timing) -> Self {
        self.interaction = Arc::new(tween_builder(timing));
        self
    }

    /// Replaces the synchronization animation with a tween.
    #[must_use]
    pub fn with_sync_tween(mut self, timing: Timing) -> Self {
        self.sync = Arc::new(tween_builder(timing));
        self
    }

    pub(crate) fn build(&self, transition: ButtonMotionTransition) -> BoxAnimation<ButtonMotion> {
        match transition.signal {
            ButtonSignal::Sync(_) => (self.sync)(transition),
            _ => (self.interaction)(transition),
        }
    }

    pub(crate) fn build_with_overrides(
        overrides: &AnimationOverrides,
        transition: ButtonMotionTransition,
    ) -> BoxAnimation<ButtonMotion> {
        overrides.get::<Self>().map_or_else(
            || default_animation(transition),
            |animations| animations.build(transition),
        )
    }
}

impl Default for ButtonAnimations {
    fn default() -> Self {
        Self::tween(
            profile_button_timing(DEFAULT_PROFILE_BUTTON_DURATION_MS),
            profile_button_timing(DEFAULT_PROFILE_BUTTON_DURATION_MS),
        )
    }
}

fn default_animation(transition: ButtonMotionTransition) -> BoxAnimation<ButtonMotion> {
    Tween::between(
        transition.from,
        transition.to,
        profile_button_timing(DEFAULT_PROFILE_BUTTON_DURATION_MS),
    )
    .boxed()
}

fn tween_builder(timing: Timing) -> impl Fn(ButtonMotionTransition) -> BoxAnimation<ButtonMotion> {
    move |transition| Tween::between(transition.from, transition.to, timing).boxed()
}

/// Profile button timing with a configurable duration.
///
/// This uses a standard ease-out curve and is intentionally independent from
/// any toolkit-private animation definition.
#[must_use]
pub fn profile_button_timing(duration_ms: f32) -> Timing {
    Timing::new(duration_ms).with_easing(Easing::EaseOut)
}

#[cfg(test)]
mod tests {
    use aura_anim::prelude::Easing;
    use float_cmp::assert_approx_eq;

    use super::profile_button_timing;

    #[test]
    fn profile_button_timing_uses_configurable_ease_out_duration() {
        let timing = profile_button_timing(125.0);

        assert_approx_eq!(f64, timing.duration().as_millis(), 125.0);
        assert!(matches!(timing.easing(), Easing::EaseOut));
    }
}
