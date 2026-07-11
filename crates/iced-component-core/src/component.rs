//! Shared component-side motion slots.

mod context;
mod macros;

/// Typed application-level animation overrides.
pub mod animation;
/// Theme-independent button interaction protocol and state.
pub mod button;

use aura_anim::{
    core::{runtime::PlaybackId, traits::Animatable},
    prelude::{Animation, Motion, MotionError, MotionRuntime, Timing, tween_to},
};
pub use context::{
    ComponentContext, ComponentUpdateCx, ComponentViewCx, StyleChange, StyleRevision,
};

/// Component-owned optional motion slot.
///
/// Components can be constructed before an application has a `MotionRuntime`.
/// Calling [`register`](Self::register) later inserts the animation into the
/// application-owned runtime exactly once.
///
/// The slot records the [`StyleRevision`] associated with the registered
/// runtime value. Rendering code should use [`value_if_current`](Self::value_if_current)
/// and fall back to freshly resolved style values when the slot is stale.
/// Interaction paths only animate after explicit registration. Before
/// registration, component state can still jump to its resolved final value.
#[derive(Debug)]
pub struct MotionSlot<T: Animatable> {
    motion: Option<Motion<T>>,
    style_revision: Option<StyleRevision>,
}

impl<T: Animatable> MotionSlot<T> {
    /// Creates an unregistered component motion slot.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            motion: None,
            style_revision: None,
        }
    }

    /// Registers this slot in the runtime if needed and returns the motion.
    pub fn register(
        &mut self,
        runtime: &mut MotionRuntime,
        initial: T,
        style_revision: StyleRevision,
    ) -> Motion<T> {
        self.register_with(runtime, initial, style_revision, Timing::default())
    }

    /// Registers this slot with a fallback timing for direct `transition_to` calls.
    pub fn register_with(
        &mut self,
        runtime: &mut MotionRuntime,
        initial: T,
        style_revision: StyleRevision,
        timing: Timing,
    ) -> Motion<T> {
        if let Some(motion) = self.motion {
            trace_slot::<T>("register_reuse", Some(motion), self.style_revision, None);
            return motion;
        }

        let motion = runtime.motion_with(initial, timing);
        self.motion = Some(motion);
        self.style_revision = Some(style_revision);
        trace_slot::<T>("register", Some(motion), self.style_revision, None);
        motion
    }

    /// Plays a tween toward `target`.
    ///
    /// Finishing the real animation keeps playback completion semantics owned by
    /// `aura-anim`, including direction, iteration, spring, and sequence rules.
    ///
    /// Returns `Ok(false)` before explicit registration.
    pub fn tween(
        &mut self,
        target: T,
        timing: Timing,
        cx: &mut ComponentUpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        let Some(motion) = self.motion else {
            trace_slot::<T>(
                "tween_skipped_unregistered",
                None,
                self.style_revision,
                Some(cx.reduce_motion()),
            );
            return Ok(false);
        };

        motion.play(tween_to(target, timing), cx.runtime)?;
        finish_if_reduced(motion, cx.reduce_motion(), cx.runtime)?;
        self.style_revision = Some(cx.context().style_revision());
        trace_slot::<T>(
            "tween",
            Some(motion),
            self.style_revision,
            Some(cx.reduce_motion()),
        );
        Ok(true)
    }

    /// Plays an arbitrary animation on an explicitly registered slot.
    ///
    /// Returns `Ok(false)` before registration.
    pub fn play<A>(
        &mut self,
        playback: A,
        cx: &mut ComponentUpdateCx<'_>,
    ) -> Result<bool, MotionError>
    where
        A: Animation<T>,
    {
        let Some(motion) = self.motion else {
            trace_slot::<T>(
                "play_skipped_unregistered",
                None,
                self.style_revision,
                Some(cx.reduce_motion()),
            );
            return Ok(false);
        };

        motion.play(playback, cx.runtime)?;
        finish_if_reduced(motion, cx.reduce_motion(), cx.runtime)?;
        self.style_revision = Some(cx.context().style_revision());
        trace_slot::<T>(
            "play",
            Some(motion),
            self.style_revision,
            Some(cx.reduce_motion()),
        );
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

    /// Returns whether the registered motion value belongs to `style_revision`.
    #[must_use]
    pub fn is_current(&self, style_revision: StyleRevision) -> bool {
        self.style_revision == Some(style_revision)
    }

    /// Returns the style revision associated with the registered motion value.
    #[must_use]
    pub const fn style_revision(&self) -> Option<StyleRevision> {
        self.style_revision
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
            trace_slot::<T>(
                "transition_to_skipped_unregistered",
                None,
                self.style_revision,
                None,
            );
            return Ok(false);
        };

        motion.transition_to(target, runtime)?;
        trace_slot::<T>("transition_to", Some(motion), self.style_revision, None);
        Ok(true)
    }

    /// Plays an arbitrary tracked animation on an explicitly registered slot.
    ///
    /// Returns `Ok(None)` when called before registration.
    pub fn play_tracked<A>(
        &mut self,
        playback: A,
        cx: &mut ComponentUpdateCx<'_>,
    ) -> Result<Option<PlaybackId>, MotionError>
    where
        A: Animation<T>,
    {
        let Some(motion) = self.motion else {
            trace_slot::<T>(
                "play_tracked_skipped_unregistered",
                None,
                self.style_revision,
                Some(cx.reduce_motion()),
            );
            return Ok(None);
        };

        let playback = motion.play_tracked(playback, cx.runtime)?;
        finish_if_reduced(motion, cx.reduce_motion(), cx.runtime)?;
        self.style_revision = Some(cx.context().style_revision());
        trace_slot::<T>(
            "play_tracked",
            Some(motion),
            self.style_revision,
            Some(cx.reduce_motion()),
        );
        Ok(Some(playback))
    }

    /// Borrows the current runtime value when registered.
    pub fn value<'a>(&self, runtime: &'a MotionRuntime) -> Result<Option<&'a T>, MotionError> {
        self.motion
            .map(|motion| motion.value_ref(runtime))
            .transpose()
    }

    /// Borrows the runtime value only when it belongs to `style_revision`.
    pub fn value_if_current<'a>(
        &self,
        runtime: &'a MotionRuntime,
        style_revision: StyleRevision,
    ) -> Result<Option<&'a T>, MotionError> {
        if !self.is_current(style_revision) {
            return Ok(None);
        }

        self.value(runtime)
    }
}

impl<T: Animatable> Default for MotionSlot<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "tracing")]
fn trace_slot<T: Animatable>(
    action: &'static str,
    motion: Option<Motion<T>>,
    _style_revision: Option<StyleRevision>,
    _reduce_motion: Option<bool>,
) {
    let motion_id = motion.map(Motion::motion_id);
    let motion_type = core::any::type_name::<T>()
        .rsplit("::")
        .next()
        .unwrap_or("unknown");

    tracing::trace!(
        target: "iced_component_core::motion_slot",
        action,
        motion_type,
        ?motion_id,
        "motion slot"
    );
}

#[cfg(not(feature = "tracing"))]
fn trace_slot<T: Animatable>(
    _action: &'static str,
    _motion: Option<Motion<T>>,
    _style_revision: Option<StyleRevision>,
    _reduce_motion: Option<bool>,
) {
}

fn finish_if_reduced<T: Animatable>(
    motion: Motion<T>,
    reduce_motion: bool,
    runtime: &mut MotionRuntime,
) -> Result<(), MotionError> {
    if reduce_motion {
        trace_slot::<T>("finish_reduced_motion", Some(motion), None, Some(true));
        motion.finish(runtime)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::component::{ComponentContext, ComponentUpdateCx, StyleRevision};
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
            StyleRevision::default(),
            Timing::linear(100.0),
        );
        let second = motion.register_with(
            &mut runtime,
            1.0_f32,
            StyleRevision::default(),
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
            StyleRevision::default(),
            Timing::linear(100.0),
        );
        let changed = motion.transition_to(1.0, &mut runtime).unwrap();
        runtime.tick(Duration::from_millis(100.0));

        assert!(changed);
        assert_eq!(motion.value(&runtime).unwrap().copied(), Some(1.0));
    }

    #[test]
    fn tween_is_ignored_before_registration() {
        let mut runtime = MotionRuntime::new();
        let mut context = ComponentContext::default();
        let mut motion = MotionSlot::new();

        let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
        let changed = motion
            .tween(1.0_f32, Timing::linear(100.0), &mut cx)
            .unwrap();

        assert!(!changed);
        assert_eq!(runtime.motion_count(), 0);
        assert_eq!(motion.value(&runtime).unwrap().copied(), None);
    }

    #[test]
    fn registered_motion_can_play_timeline() {
        let mut runtime = MotionRuntime::new();
        let mut context = ComponentContext::default();
        let mut motion = MotionSlot::new();

        let _handle = motion.register(&mut runtime, 0.0_f32, context.style_revision());
        let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
        let played = motion
            .play(
                Sequence::new(0.0_f32)
                    .then(Tween::between(0.0, 2.0, Timing::linear(100.0)))
                    .then(Tween::between(2.0, 1.0, Timing::linear(100.0))),
                &mut cx,
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
        let _ = motion.register(&mut runtime, 0.0_f32, context.style_revision());
        let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);

        let played = motion.tween(1.0, Timing::linear(100.0), &mut cx).unwrap();

        assert!(played);
        assert_eq!(motion.value(&runtime).unwrap().copied(), Some(1.0));
    }

    #[test]
    fn value_if_current_ignores_stale_style_revision() {
        let mut runtime = MotionRuntime::new();
        let mut motion = MotionSlot::new();
        let mut context = ComponentContext::default();
        let original = context.style_revision();
        let _ = motion.register(&mut runtime, 1.0_f32, original);

        assert_eq!(
            motion
                .value_if_current(&runtime, original)
                .unwrap()
                .copied(),
            Some(1.0)
        );

        context.bump_style_revision();

        assert_eq!(
            motion
                .value_if_current(&runtime, context.style_revision())
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
        let handle = motion.register(&mut runtime, 0.0_f32, context.style_revision());

        let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);

        let playback = motion
            .play_tracked(Tween::between(0.0, 1.0, Timing::linear(100.0)), &mut cx)
            .unwrap()
            .expect("registered motion should start tracked playback");

        assert_eq!(handle.playback(&runtime).unwrap(), playback);
        assert_eq!(motion.value(&runtime).unwrap().copied(), Some(1.0));
        assert!(runtime.take_events()[0].is_completed_for(playback));
    }

    #[test]
    fn tracked_play_returns_playback_id() {
        let mut runtime = MotionRuntime::new();
        let mut context = ComponentContext::default();
        let mut motion = MotionSlot::new();

        let handle = motion.register(&mut runtime, 0.0_f32, context.style_revision());
        let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
        let playback = motion
            .play_tracked(Tween::between(0.0, 1.0, Timing::linear(100.0)), &mut cx)
            .unwrap()
            .expect("registered motion should start tracked playback");
        runtime.tick(Duration::from_millis(100.0));

        assert_eq!(handle.playback(&runtime).unwrap(), playback);
        assert!(runtime.take_events()[0].is_completed_for(playback));
    }
}
