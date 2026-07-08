use std::sync::atomic::{AtomicU64, Ordering};

use aura_anim::prelude::MotionRuntime;

/// Monotonic marker for the style snapshot carried by a [`ComponentContext`].
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct StyleRevision(u64);

impl StyleRevision {
    fn next() -> Self {
        static NEXT_REVISION: AtomicU64 = AtomicU64::new(1);

        Self(NEXT_REVISION.fetch_add(1, Ordering::Relaxed))
    }
}

/// Shared component inputs that can change while the application is running.
#[derive(Clone)]
pub struct ComponentContext {
    style_revision: StyleRevision,
    reduce_motion: bool,
}

impl ComponentContext {
    /// Creates a component context.
    #[must_use]
    pub fn new() -> Self {
        Self {
            style_revision: StyleRevision::next(),
            reduce_motion: false,
        }
    }

    /// Returns the revision of the current style snapshot.
    #[must_use]
    pub const fn style_revision(&self) -> StyleRevision {
        self.style_revision
    }

    /// Returns whether non-essential animation should be reduced.
    #[must_use]
    pub const fn reduce_motion(&self) -> bool {
        self.reduce_motion
    }

    /// Updates whether non-essential animation should be reduced.
    pub fn set_reduce_motion(&mut self, reduce_motion: bool) {
        self.reduce_motion = reduce_motion;
    }

    /// Returns a context with a different reduced-motion preference.
    #[must_use]
    pub const fn with_reduce_motion(mut self, reduce_motion: bool) -> Self {
        self.reduce_motion = reduce_motion;
        self
    }

    /// Toggles the reduced-motion preference.
    pub fn toggle_reduce_motion(&mut self) {
        self.reduce_motion = !self.reduce_motion;
    }

    /// Marks externally owned style/theme state as changed.
    pub fn bump_style_revision(&mut self) {
        self.style_revision = StyleRevision::next();
    }
}

impl Default for ComponentContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Mutable inputs used while applying component events.
pub struct ComponentUpdateCx<'a> {
    /// Application-owned animation runtime.
    pub runtime: &'a mut MotionRuntime,
    /// Mutable component context.
    pub context: &'a mut ComponentContext,
}

impl<'a> ComponentUpdateCx<'a> {
    /// Creates an update context from the application runtime and component context.
    pub fn new(runtime: &'a mut MotionRuntime, context: &'a mut ComponentContext) -> Self {
        Self { runtime, context }
    }

    /// Returns the current component context.
    #[must_use]
    pub const fn context(&self) -> &ComponentContext {
        self.context
    }

    /// Returns the mutable component context.
    pub fn context_mut(&mut self) -> &mut ComponentContext {
        self.context
    }

    /// Returns whether motion is reduced.
    #[must_use]
    pub fn reduce_motion(&self) -> bool {
        self.context.reduce_motion
    }
}

/// Read-only inputs used while rendering component views.
pub struct ComponentViewCx<'a> {
    /// Application-owned animation runtime.
    pub runtime: &'a MotionRuntime,
    /// Component context snapshot used by view resolution.
    pub context: &'a ComponentContext,
}

impl<'a> ComponentViewCx<'a> {
    /// Creates a view context from the application runtime and component context.
    #[must_use]
    pub const fn new(runtime: &'a MotionRuntime, context: &'a ComponentContext) -> Self {
        Self { runtime, context }
    }

    /// Returns the component context.
    #[must_use]
    pub const fn context(&self) -> &ComponentContext {
        self.context
    }

    /// Returns whether motion is reduced.
    #[must_use]
    pub fn reduce_motion(&self) -> bool {
        self.context.reduce_motion
    }
}

#[cfg(test)]
mod tests {
    use crate::component::ComponentContext;

    #[test]
    fn context_can_track_style_revision_and_reduce_motion() {
        let mut context = ComponentContext::new();
        let initial_revision = context.style_revision();

        context.bump_style_revision();
        let patched_revision = context.style_revision();
        context.set_reduce_motion(true);

        assert_ne!(patched_revision, initial_revision);
        assert_eq!(context.style_revision(), patched_revision);
        assert!(context.reduce_motion());
    }

    #[test]
    fn style_revision_changes_are_scoped_to_one_component_context() {
        let mut first = ComponentContext::new();
        let second = ComponentContext::new();
        let second_revision = second.style_revision();

        first.bump_style_revision();

        assert_ne!(first.style_revision(), second_revision);
        assert_eq!(second.style_revision(), second_revision);
    }
}
