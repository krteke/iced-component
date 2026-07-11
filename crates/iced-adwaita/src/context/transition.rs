//! Context-wide style transition shared by Adwaita components.

use std::rc::Rc;

use aura_anim::{
    core::traits::BoxAnimation,
    prelude::{AnimationExt, Motion, MotionRuntime, Timing, Tween},
};

use crate::theme::ThemePack;

/// Builds one normalized style-transition progress animation from `0` to `1`.
pub type StyleTransitionBuilder = Rc<dyn Fn() -> BoxAnimation<f32>>;

/// Runtime-configurable transition used for changes within an Adwaita theme.
#[derive(Clone)]
pub struct StyleTransition {
    builder: StyleTransitionBuilder,
}

impl StyleTransition {
    /// Creates a style transition from an arbitrary Aura animation builder.
    ///
    /// The produced animation must start at `0` and settle at `1`.
    #[must_use]
    pub fn new(builder: impl Fn() -> BoxAnimation<f32> + 'static) -> Self {
        Self {
            builder: Rc::new(builder),
        }
    }

    /// Creates a tween-based style transition.
    #[must_use]
    pub fn tween(timing: Timing) -> Self {
        Self::new(move || Tween::between(0.0, 1.0, timing).boxed())
    }

    fn build(&self) -> BoxAnimation<f32> {
        (self.builder)()
    }
}

impl Default for StyleTransition {
    fn default() -> Self {
        Self::tween(Timing::ease_out(200.0))
    }
}

pub(super) struct StyleTransitionState {
    config: StyleTransition,
    from: Option<ThemePack>,
    progress: Option<Motion<f32>>,
}

impl StyleTransitionState {
    pub(super) fn new() -> Self {
        Self {
            config: StyleTransition::default(),
            from: None,
            progress: None,
        }
    }

    pub(super) const fn config(&self) -> &StyleTransition {
        &self.config
    }

    pub(super) fn set_config(&mut self, config: StyleTransition) {
        self.config = config;
    }

    pub(super) fn start(
        &mut self,
        from: ThemePack,
        runtime: &mut MotionRuntime,
        reduce_motion: bool,
    ) {
        let progress = *self.progress.get_or_insert_with(|| runtime.motion(0.0));

        progress
            .play(self.config.build(), runtime)
            .expect("style transition motion belongs to its context runtime");
        if reduce_motion {
            progress
                .finish(runtime)
                .expect("new style transition can be finished");
        }
        self.from = Some(from);
    }

    pub(super) fn finish(&mut self, runtime: &mut MotionRuntime) {
        if let Some(progress) = self.progress {
            progress
                .finish(runtime)
                .expect("style transition motion belongs to its context runtime");
        }
    }

    pub(super) fn snapshot(&self, runtime: &MotionRuntime) -> Option<StyleTransitionSnapshot<'_>> {
        let from = self.from.as_ref()?;
        let progress = *self
            .progress?
            .value_ref(runtime)
            .expect("style transition motion belongs to its context runtime");

        (progress < 1.0 - f32::EPSILON).then_some(StyleTransitionSnapshot { from, progress })
    }
}

impl Clone for StyleTransitionState {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            from: None,
            progress: None,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct StyleTransitionSnapshot<'a> {
    pub(crate) from: &'a ThemePack,
    pub(crate) progress: f32,
}

#[cfg(test)]
mod tests {
    use aura_anim::prelude::{Duration, MotionRuntime, Timing};

    use super::StyleTransitionState;
    use crate::theme::ThemePack;

    #[test]
    fn style_transition_exposes_progress_until_it_completes() {
        let mut runtime = MotionRuntime::new();
        let mut transition = StyleTransitionState::new();
        transition.set_config(super::StyleTransition::tween(Timing::linear(100.0)));

        transition.start(ThemePack::light(), &mut runtime, false);
        runtime.tick(Duration::from_millis(50.0));

        let snapshot = transition.snapshot(&runtime).expect("transition is active");
        assert!((snapshot.progress - 0.5).abs() < 0.001);

        runtime.tick(Duration::from_millis(50.0));
        assert!(transition.snapshot(&runtime).is_none());
    }
}
