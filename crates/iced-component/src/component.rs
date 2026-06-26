//! Shared component-side motion slots.

mod context;

use aura_anim::{
    core::{
        runtime::PlaybackId,
        traits::{Animatable, IntoMotionAnimation},
    },
    prelude::{Motion, MotionError, MotionRuntime, Timing, tween_to},
};
pub use context::{ComponentContext, ComponentUpdateCx, ComponentViewCx};

/// Component-owned optional motion slot.
///
/// Components can be constructed before an application has a `MotionRuntime`.
/// Calling [`register`](Self::register) later inserts the animation into the
/// application-owned runtime exactly once.
#[derive(Debug)]
pub struct MotionSlot<T: Animatable> {
    initial: T,
    motion: Option<Motion<T>>,
}

impl<T: Animatable> MotionSlot<T> {
    /// Creates an unregistered component motion slot.
    #[must_use]
    pub fn new(initial: T) -> Self {
        Self {
            initial,
            motion: None,
        }
    }

    /// Registers this slot in the runtime if needed and returns the motion.
    pub fn register(&mut self, runtime: &mut MotionRuntime) -> Motion<T> {
        self.register_with(runtime, Timing::default())
    }

    /// Registers this slot with a fallback timing for direct `transition_to` calls.
    pub fn register_with(&mut self, runtime: &mut MotionRuntime, timing: Timing) -> Motion<T> {
        *self
            .motion
            .get_or_insert_with(|| runtime.motion_with(self.initial.clone(), timing))
    }

    /// Plays a tween toward `target` using a timing resolved at call time.
    ///
    /// Before registration this only updates the fallback value and returns
    /// `Ok(false)`.
    pub fn tween_to(
        &mut self,
        target: T,
        timing: Timing,
        runtime: &mut MotionRuntime,
    ) -> Result<bool, MotionError> {
        let Some(motion) = self.motion else {
            self.initial = target;
            return Ok(false);
        };

        motion.play(tween_to(target, timing), runtime)?;
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

    /// Returns the initial fallback value used before registration.
    #[must_use]
    pub const fn initial(&self) -> &T {
        &self.initial
    }

    /// Replaces the initial fallback value.
    pub fn set_initial(&mut self, initial: T) {
        self.initial = initial;
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
            self.initial = target;
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

    /// Borrows the current runtime value when registered.
    pub fn value<'a>(&self, runtime: &'a MotionRuntime) -> Result<Option<&'a T>, MotionError> {
        self.motion
            .map(|motion| motion.value_ref(runtime))
            .transpose()
    }
}

#[cfg(test)]
mod tests {
    use aura_anim::prelude::*;
    use float_cmp::assert_approx_eq;

    use super::MotionSlot;

    #[test]
    fn transition_is_ignored_before_registration() {
        let mut runtime = MotionRuntime::new();
        let mut motion = MotionSlot::new(0.0_f32);

        let changed = motion.transition_to(1.0, &mut runtime).unwrap();

        assert!(!changed);
        assert_eq!(runtime.motion_count(), 0);
        assert_approx_eq!(f32, *motion.initial(), 1.0);
    }

    #[test]
    fn register_inserts_motion_only_once() {
        let mut runtime = MotionRuntime::new();
        let mut motion = MotionSlot::new(0.0_f32);

        let first = motion.register_with(&mut runtime, Timing::linear(100.0));
        let second = motion.register_with(&mut runtime, Timing::linear(50.0));

        assert_eq!(first.motion_id(), second.motion_id());
        assert_eq!(runtime.motion_count(), 1);
        assert!(motion.is_registered());
    }

    #[test]
    fn registered_motion_transitions_runtime_value() {
        let mut runtime = MotionRuntime::new();
        let mut motion = MotionSlot::new(0.0_f32);

        let _handle = motion.register_with(&mut runtime, Timing::linear(100.0));
        let changed = motion.transition_to(1.0, &mut runtime).unwrap();
        runtime.tick(Duration::from_millis(100.0));

        assert!(changed);
        assert_eq!(motion.value(&runtime).unwrap().copied(), Some(1.0));
    }

    #[test]
    fn registered_motion_can_play_timeline() {
        let mut runtime = MotionRuntime::new();
        let mut motion = MotionSlot::new(0.0_f32);

        let _handle = motion.register(&mut runtime);
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
    fn tracked_play_returns_playback_id() {
        let mut runtime = MotionRuntime::new();
        let mut motion = MotionSlot::new(0.0_f32);

        let handle = motion.register(&mut runtime);
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
