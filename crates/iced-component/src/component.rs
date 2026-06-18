//! Shared component-side motion handles.

use aura_anim_core::{Animatable, Motion, MotionError, MotionRuntime, timing::Timing};

/// Component-owned optional motion handle.
///
/// Components can be constructed before an application has a `MotionRuntime`.
/// Calling [`register`](Self::register) later inserts the animation into the
/// application-owned runtime exactly once.
#[derive(Debug)]
pub struct ComponentMotion<T: Animatable> {
    initial: T,
    timing: Timing,
    motion: Option<Motion<T>>,
}

impl<T: Animatable> ComponentMotion<T> {
    /// Creates an unregistered component motion handle.
    #[must_use]
    pub fn new(initial: T, timing: Timing) -> Self {
        Self {
            initial,
            timing,
            motion: None,
        }
    }

    /// Registers this handle in the runtime if needed and returns the motion.
    pub fn register(&mut self, runtime: &mut MotionRuntime) -> Motion<T> {
        *self
            .motion
            .get_or_insert_with(|| runtime.motion_with(self.initial.clone(), self.timing))
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

    /// Transitions the registered motion toward `target`.
    ///
    /// Returns `Ok(false)` when called before registration.
    pub fn transition_to(
        &self,
        target: T,
        runtime: &mut MotionRuntime,
    ) -> Result<bool, MotionError> {
        let Some(motion) = self.motion else {
            return Ok(false);
        };

        motion.transition_to(target, runtime)?;
        Ok(true)
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
    use aura_anim_core::{
        MotionRuntime,
        timing::{Duration, Timing},
    };

    use super::ComponentMotion;

    #[test]
    fn transition_is_ignored_before_registration() {
        let mut runtime = MotionRuntime::new();
        let motion = ComponentMotion::new(0.0_f32, Timing::linear(100.0));

        let changed = motion.transition_to(1.0, &mut runtime).unwrap();

        assert!(!changed);
        assert_eq!(runtime.motion_count(), 0);
    }

    #[test]
    fn register_inserts_motion_only_once() {
        let mut runtime = MotionRuntime::new();
        let mut motion = ComponentMotion::new(0.0_f32, Timing::linear(100.0));

        let first = motion.register(&mut runtime);
        let second = motion.register(&mut runtime);

        assert_eq!(first.motion_id(), second.motion_id());
        assert_eq!(runtime.motion_count(), 1);
        assert!(motion.is_registered());
    }

    #[test]
    fn registered_motion_transitions_runtime_value() {
        let mut runtime = MotionRuntime::new();
        let mut motion = ComponentMotion::new(0.0_f32, Timing::linear(100.0));

        let _handle = motion.register(&mut runtime);
        let changed = motion.transition_to(1.0, &mut runtime).unwrap();
        runtime.tick(Duration::from_millis(100.0));

        assert!(changed);
        assert_eq!(motion.value(&runtime).unwrap().copied(), Some(1.0));
    }
}
