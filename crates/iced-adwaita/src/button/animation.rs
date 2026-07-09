use std::sync::Arc;

use aura_anim::{
    core::traits::BoxAnimation,
    prelude::{AnimationExt, Easing, Timing, Tween},
};

use crate::button::{ButtonMotion, ButtonMotionTransition, ButtonSignal};

/// A function that builds one button animation from a resolved transition.
pub type ButtonAnimationBuilder = Arc<dyn Fn(ButtonMotionTransition) -> BoxAnimation<ButtonMotion>>;

/// Runtime-configurable Adwaita button animations.
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
}

impl Default for ButtonAnimations {
    fn default() -> Self {
        Self::tween(adwaita_button_timing(200.0), adwaita_button_timing(200.0))
    }
}

fn tween_builder(timing: Timing) -> impl Fn(ButtonMotionTransition) -> BoxAnimation<ButtonMotion> {
    move |transition| Tween::between(transition.from, transition.to, timing).boxed()
}

/// Adwaita-style button timing with a configurable duration.
#[must_use]
pub fn adwaita_button_timing(duration_ms: f32) -> Timing {
    Timing::new(duration_ms).with_easing(Easing::Custom(adwaita_ease_out_quad))
}

fn adwaita_ease_out_quad(progress: f32) -> f32 {
    cubic_bezier_y_for_x(progress, 0.25, 0.46, 0.45, 0.94)
}

fn cubic_bezier_y_for_x(x: f32, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let x = x.clamp(0.0, 1.0);
    let mut low = 0.0;
    let mut high = 1.0;
    let mut t = x;

    for _ in 0..16 {
        let estimate = cubic_bezier(t, x1, x2);
        if (estimate - x).abs() < 0.000_001 {
            break;
        }
        if estimate < x {
            low = t;
        } else {
            high = t;
        }
        t = (low + high) * 0.5;
    }

    cubic_bezier(t, y1, y2)
}

fn cubic_bezier(t: f32, p1: f32, p2: f32) -> f32 {
    let inv = 1.0 - t;

    3.0 * inv * inv * t * p1 + 3.0 * inv * t * t * p2 + t * t * t
}

#[cfg(test)]
mod tests {
    use aura_anim::prelude::Easing;
    use float_cmp::assert_approx_eq;

    use super::{adwaita_button_timing, adwaita_ease_out_quad};

    #[test]
    fn adwaita_button_timing_keeps_css_curve_with_configurable_duration() {
        let timing = adwaita_button_timing(125.0);

        assert_approx_eq!(f64, timing.duration().as_millis(), 125.0);
        assert!(matches!(timing.easing(), Easing::Custom(_)));
        assert_approx_eq!(f32, adwaita_ease_out_quad(0.0), 0.0);
        assert_approx_eq!(f32, adwaita_ease_out_quad(1.0), 1.0);
        assert!(adwaita_ease_out_quad(0.5) > 0.5);
    }
}
