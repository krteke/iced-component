//! Shared component-side motion slots.

mod context;
mod macros;

use aura_anim::{
    core::{
        runtime::PlaybackId,
        traits::{Animatable, IntoMotionAnimation},
    },
    prelude::{Animation, Motion, MotionError, MotionRuntime, Timing, tween_to},
};
pub use context::{ComponentContext, ComponentUpdateCx, ComponentViewCx, ThemeRevision};

/// Component-owned optional motion slot.
///
/// Components can be constructed before an application has a `MotionRuntime`.
/// Calling [`register`](Self::register) later inserts the animation into the
/// application-owned runtime exactly once.
///
/// The slot records the [`ThemeRevision`] associated with the registered
/// runtime value. Rendering code should use [`value_if_current`](Self::value_if_current)
/// and fall back to freshly resolved theme tokens when the slot is stale.
/// Interaction paths that need a fresh starting value after a theme change
/// should use [`tween_from_to_or_finish`](Self::tween_from_to_or_finish), which
/// re-registers stale runtime handles from the provided initial value.
#[derive(Debug)]
pub struct MotionSlot<T: Animatable> {
    motion: Option<Motion<T>>,
    theme_revision: Option<ThemeRevision>,
}

impl<T: Animatable> MotionSlot<T> {
    /// Creates an unregistered component motion slot.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            motion: None,
            theme_revision: None,
        }
    }

    /// Registers this slot in the runtime if needed and returns the motion.
    pub fn register(
        &mut self,
        runtime: &mut MotionRuntime,
        initial: T,
        theme_revision: ThemeRevision,
    ) -> Motion<T> {
        self.register_with(runtime, initial, theme_revision, Timing::default())
    }

    /// Registers this slot with a fallback timing for direct `transition_to` calls.
    pub fn register_with(
        &mut self,
        runtime: &mut MotionRuntime,
        initial: T,
        theme_revision: ThemeRevision,
        timing: Timing,
    ) -> Motion<T> {
        if let Some(motion) = self.motion {
            return motion;
        }

        let motion = runtime.motion_with(initial, timing);
        self.motion = Some(motion);
        self.theme_revision = Some(theme_revision);
        motion
    }

    /// Plays a tween toward `target` using a timing resolved at call time.
    ///
    /// Before registration this only updates the fallback value and returns
    /// `Ok(false)`.
    pub fn tween_to(
        &mut self,
        target: T,
        theme_revision: ThemeRevision,
        timing: Timing,
        runtime: &mut MotionRuntime,
    ) -> Result<bool, MotionError> {
        let Some(motion) = self.motion else {
            return Ok(false);
        };

        motion.play(tween_to(target, timing), runtime)?;
        self.theme_revision = Some(theme_revision);
        Ok(true)
    }

    /// Plays a tween and immediately finishes it when reduced motion is enabled.
    ///
    /// Finishing the real animation keeps playback completion semantics owned by
    /// `aura-anim`, including direction, iteration, spring, and sequence rules.
    pub fn tween_to_or_finish(
        &mut self,
        target: T,
        timing: Timing,
        cx: &mut ComponentUpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        let Some(motion) = self.motion else {
            return Ok(false);
        };

        motion.play(tween_to(target, timing), cx.runtime)?;
        finish_if_reduced(motion, cx.reduce_motion(), cx.runtime)?;
        self.theme_revision = Some(cx.context().theme_revision());
        Ok(true)
    }

    /// Registers from `initial` when needed, then plays a tween toward `target`.
    pub fn tween_from_to(
        &mut self,
        initial: T,
        target: T,
        theme_revision: ThemeRevision,
        timing: Timing,
        runtime: &mut MotionRuntime,
    ) -> Result<bool, MotionError> {
        let motion = self.register_fresh_if_stale(runtime, initial, theme_revision, timing)?;

        motion.play(tween_to(target, timing), runtime)?;
        self.theme_revision = Some(theme_revision);
        Ok(true)
    }

    /// Registers from `initial` when needed, plays a tween, and finishes it when reduced.
    pub fn tween_from_to_or_finish(
        &mut self,
        initial: T,
        target: T,
        timing: Timing,
        cx: &mut ComponentUpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        let theme_revision = cx.context().theme_revision();
        let motion = self.register_fresh_if_stale(cx.runtime, initial, theme_revision, timing)?;

        motion.play(tween_to(target, timing), cx.runtime)?;
        finish_if_reduced(motion, cx.reduce_motion(), cx.runtime)?;
        self.theme_revision = Some(theme_revision);
        Ok(true)
    }

    /// Registers from `initial` when needed, plays an arbitrary animation, and
    /// finishes it when reduced motion is enabled.
    pub fn play_from_or_finish<A>(
        &mut self,
        initial: T,
        playback: A,
        cx: &mut ComponentUpdateCx<'_>,
    ) -> Result<bool, MotionError>
    where
        A: Animation<T>,
    {
        let theme_revision = cx.context().theme_revision();
        let motion =
            self.register_fresh_if_stale(cx.runtime, initial, theme_revision, Timing::default())?;

        motion.play(playback, cx.runtime)?;
        finish_if_reduced(motion, cx.reduce_motion(), cx.runtime)?;
        self.theme_revision = Some(theme_revision);
        Ok(true)
    }

    /// Registers from `initial` when needed, then plays an arbitrary animation.
    pub fn play_from<A>(
        &mut self,
        initial: T,
        playback: A,
        cx: &mut ComponentUpdateCx<'_>,
    ) -> Result<bool, MotionError>
    where
        A: Animation<T>,
    {
        let theme_revision = cx.context().theme_revision();
        let motion =
            self.register_fresh_if_stale(cx.runtime, initial, theme_revision, Timing::default())?;

        motion.play(playback, cx.runtime)?;
        self.theme_revision = Some(theme_revision);
        Ok(true)
    }

    /// Returns the registered motion handle, if registration happened.
    #[must_use]
    pub fn motion(&self) -> Option<Motion<T>> {
        self.motion
    }

    /// Returns whether this handle has been registered.
    #[must_use]
    pub fn is_registered(&self) -> bool {
        self.motion.is_some()
    }

    /// Returns whether the registered motion value belongs to `theme_revision`.
    #[must_use]
    pub fn is_current(&self, theme_revision: ThemeRevision) -> bool {
        self.theme_revision == Some(theme_revision)
    }

    /// Returns the theme revision associated with the registered motion value.
    #[must_use]
    pub const fn theme_revision(&self) -> Option<ThemeRevision> {
        self.theme_revision
    }

    /// Transitions the registered motion toward `target`.
    ///
    /// Before registration this only updates the fallback value and returns
    /// `Ok(false)`.
    pub fn transition_to(
        &mut self,
        target: T,
        runtime: &mut MotionRuntime,
    ) -> Result<bool, MotionError> {
        let Some(motion) = self.motion else {
            return Ok(false);
        };

        motion.transition_to(target, runtime)?;
        Ok(true)
    }

    /// Replaces the registered motion's current animation.
    ///
    /// Returns `Ok(false)` when called before registration.
    pub fn play<P, Kind>(
        &self,
        playback: P,
        runtime: &mut MotionRuntime,
    ) -> Result<bool, MotionError>
    where
        P: IntoMotionAnimation<T, Kind>,
    {
        let Some(motion) = self.motion else {
            return Ok(false);
        };

        motion.play(playback, runtime)?;
        Ok(true)
    }

    /// Replaces the current animation and immediately finishes it when reduced motion is enabled.
    ///
    /// Returns `Ok(false)` when called before registration.
    pub fn play_or_finish<P, Kind>(
        &self,
        playback: P,
        cx: &mut ComponentUpdateCx<'_>,
    ) -> Result<bool, MotionError>
    where
        P: IntoMotionAnimation<T, Kind>,
    {
        let Some(motion) = self.motion else {
            return Ok(false);
        };

        motion.play(playback, cx.runtime)?;
        finish_if_reduced(motion, cx.reduce_motion(), cx.runtime)?;
        Ok(true)
    }

    /// Replaces the registered motion's current animation and returns its playback ID.
    ///
    /// Returns `Ok(None)` when called before registration.
    pub fn play_tracked<P, Kind>(
        &self,
        playback: P,
        runtime: &mut MotionRuntime,
    ) -> Result<Option<PlaybackId>, MotionError>
    where
        P: IntoMotionAnimation<T, Kind>,
    {
        let Some(motion) = self.motion else {
            return Ok(None);
        };

        motion.play_tracked(playback, runtime).map(Some)
    }

    /// Replaces the current animation, returns its playback ID, and optionally finishes it.
    ///
    /// Returns `Ok(None)` when called before registration.
    pub fn play_tracked_or_finish<P, Kind>(
        &self,
        playback: P,
        cx: &mut ComponentUpdateCx<'_>,
    ) -> Result<Option<PlaybackId>, MotionError>
    where
        P: IntoMotionAnimation<T, Kind>,
    {
        let Some(motion) = self.motion else {
            return Ok(None);
        };

        let playback = motion.play_tracked(playback, cx.runtime)?;
        finish_if_reduced(motion, cx.reduce_motion(), cx.runtime)?;
        Ok(Some(playback))
    }

    /// Borrows the current runtime value when registered.
    pub fn value<'a>(&self, runtime: &'a MotionRuntime) -> Result<Option<&'a T>, MotionError> {
        self.motion
            .map(|motion| motion.value_ref(runtime))
            .transpose()
    }

    /// Borrows the runtime value only when it belongs to `theme_revision`.
    pub fn value_if_current<'a>(
        &self,
        runtime: &'a MotionRuntime,
        theme_revision: ThemeRevision,
    ) -> Result<Option<&'a T>, MotionError> {
        if !self.is_current(theme_revision) {
            return Ok(None);
        }

        self.value(runtime)
    }

    fn register_fresh_if_stale(
        &mut self,
        runtime: &mut MotionRuntime,
        initial: T,
        theme_revision: ThemeRevision,
        timing: Timing,
    ) -> Result<Motion<T>, MotionError> {
        if self.motion.is_some() && self.is_current(theme_revision) {
            return Ok(self.register_with(runtime, initial, theme_revision, timing));
        }

        if let Some(motion) = self.motion.take() {
            self.theme_revision = None;
            motion.remove(runtime)?;
        }

        Ok(self.register_with(runtime, initial, theme_revision, timing))
    }
}

impl<T: Animatable> Default for MotionSlot<T> {
    fn default() -> Self {
        Self::new()
    }
}

fn finish_if_reduced<T: Animatable>(
    motion: Motion<T>,
    reduce_motion: bool,
    runtime: &mut MotionRuntime,
) -> Result<(), MotionError> {
    if reduce_motion {
        motion.finish(runtime)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::component::{ComponentContext, ComponentUpdateCx, ThemeRevision};
    use aura_anim::prelude::*;

    use super::MotionSlot;

    #[test]
    fn transition_is_ignored_before_registration() {
        let mut runtime = MotionRuntime::new();
        let mut motion = MotionSlot::new();

        let changed = motion.transition_to(1.0, &mut runtime).unwrap();

        assert!(!changed);
        assert_eq!(runtime.motion_count(), 0);
        assert_eq!(motion.value(&runtime).unwrap().copied(), None);
    }

    #[test]
    fn register_inserts_motion_only_once() {
        let mut runtime = MotionRuntime::new();
        let mut motion = MotionSlot::new();

        let first = motion.register_with(
            &mut runtime,
            0.0_f32,
            ThemeRevision::default(),
            Timing::linear(100.0),
        );
        let second = motion.register_with(
            &mut runtime,
            1.0_f32,
            ThemeRevision::default(),
            Timing::linear(50.0),
        );

        assert_eq!(first.motion_id(), second.motion_id());
        assert_eq!(runtime.motion_count(), 1);
        assert!(motion.is_registered());
    }

    #[test]
    fn registered_motion_transitions_runtime_value() {
        let mut runtime = MotionRuntime::new();
        let mut motion = MotionSlot::new();

        let _handle = motion.register_with(
            &mut runtime,
            0.0_f32,
            ThemeRevision::default(),
            Timing::linear(100.0),
        );
        let changed = motion.transition_to(1.0, &mut runtime).unwrap();
        runtime.tick(Duration::from_millis(100.0));

        assert!(changed);
        assert_eq!(motion.value(&runtime).unwrap().copied(), Some(1.0));
    }

    #[test]
    fn tween_from_to_registers_before_playing() {
        let mut runtime = MotionRuntime::new();
        let mut motion = MotionSlot::new();

        let changed = motion
            .tween_from_to(
                0.0_f32,
                1.0,
                ThemeRevision::default(),
                Timing::linear(100.0),
                &mut runtime,
            )
            .unwrap();
        runtime.tick(Duration::from_millis(100.0));

        assert!(changed);
        assert_eq!(runtime.motion_count(), 1);
        assert_eq!(motion.value(&runtime).unwrap().copied(), Some(1.0));
    }

    #[test]
    fn registered_motion_can_play_timeline() {
        let mut runtime = MotionRuntime::new();
        let mut motion = MotionSlot::new();

        let _handle = motion.register(&mut runtime, 0.0_f32, ThemeRevision::default());
        let played = motion
            .play(
                Sequence::new(0.0_f32)
                    .then(Tween::between(0.0, 2.0, Timing::linear(100.0)))
                    .then(Tween::between(2.0, 1.0, Timing::linear(100.0))),
                &mut runtime,
            )
            .unwrap();
        runtime.tick(Duration::from_millis(200.0));

        assert!(played);
        assert_eq!(motion.value(&runtime).unwrap().copied(), Some(1.0));
    }

    #[test]
    fn reduced_tween_finishes_registered_motion_immediately() {
        let mut runtime = MotionRuntime::new();
        let mut motion = MotionSlot::new();
        let mut context = ComponentContext::default().with_reduce_motion(true);
        let _ = motion.register(&mut runtime, 0.0_f32, context.theme_revision());
        let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);

        let played = motion
            .tween_to_or_finish(1.0, Timing::linear(100.0), &mut cx)
            .unwrap();

        assert!(played);
        assert_eq!(motion.value(&runtime).unwrap().copied(), Some(1.0));
    }

    #[test]
    fn value_if_current_ignores_stale_theme_revision() {
        let mut runtime = MotionRuntime::new();
        let mut motion = MotionSlot::new();
        let mut context = ComponentContext::default();
        let original = context.theme_revision();
        let _ = motion.register(&mut runtime, 1.0_f32, original);

        assert_eq!(
            motion
                .value_if_current(&runtime, original)
                .unwrap()
                .copied(),
            Some(1.0)
        );

        context
            .patch_theme(|theme| theme.button.standard_filled.idle.bg = "#ddeeff".parse().unwrap());

        assert_eq!(
            motion
                .value_if_current(&runtime, context.theme_revision())
                .unwrap()
                .copied(),
            None
        );
    }

    #[test]
    fn reduced_tracked_play_finishes_with_completed_playback_event() {
        let mut runtime = MotionRuntime::new();
        let mut motion = MotionSlot::new();
        let mut context = ComponentContext::default().with_reduce_motion(true);
        let handle = motion.register(&mut runtime, 0.0_f32, context.theme_revision());

        let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);

        let playback = motion
            .play_tracked_or_finish(Tween::between(0.0, 1.0, Timing::linear(100.0)), &mut cx)
            .unwrap()
            .expect("registered motion should start tracked playback");

        assert_eq!(handle.playback(&runtime).unwrap(), playback);
        assert_eq!(motion.value(&runtime).unwrap().copied(), Some(1.0));
        assert!(runtime.take_events()[0].is_completed_for(playback));
    }

    #[test]
    fn tracked_play_returns_playback_id() {
        let mut runtime = MotionRuntime::new();
        let mut motion = MotionSlot::new();

        let handle = motion.register(&mut runtime, 0.0_f32, ThemeRevision::default());
        let playback = motion
            .play_tracked(
                Tween::between(0.0, 1.0, Timing::linear(100.0)),
                &mut runtime,
            )
            .unwrap()
            .expect("registered motion should start tracked playback");
        runtime.tick(Duration::from_millis(100.0));

        assert_eq!(handle.playback(&runtime).unwrap(), playback);
        assert!(runtime.take_events()[0].is_completed_for(playback));
    }
}
