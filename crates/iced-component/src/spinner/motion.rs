use std::sync::Arc;

use aura_anim::{core::traits::BoxAnimation, prelude::Animatable};

/// Animatable spinner values.
#[derive(Clone, Copy, Debug, PartialEq, Animatable)]
pub struct SpinnerMotion {
    /// Current clockwise rotation in degrees.
    pub rotation: f32,
}

impl Default for SpinnerMotion {
    fn default() -> Self {
        Self { rotation: 0.0 }
    }
}

/// Spinner animation trigger.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpinnerMotionTrigger {
    /// Continuous spinning started.
    Start,
    /// Spinning stopped.
    Stop,
    /// Runtime motion synchronized with current state.
    Sync,
}

/// Data passed to spinner animation providers.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpinnerMotionTransition {
    /// Current visual value used as animation start.
    pub from: SpinnerMotion,
    /// Resolved target visual value.
    pub to: SpinnerMotion,
    /// Transition trigger.
    pub trigger: SpinnerMotionTrigger,
}

/// A function type for building spinner animations.
pub type SpinnerAnimationBuilder =
    Arc<dyn Fn(SpinnerMotionTransition) -> BoxAnimation<SpinnerMotion>>;

/// Provides spinner animations for an animation theme.
pub trait SpinnerAnimationProvider: 'static {
    /// Returns the animation builder for one resolved transition.
    fn spinner_animation(&self, transition: &SpinnerMotionTransition) -> SpinnerAnimationBuilder;
}
