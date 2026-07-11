//! Runtime context for a selected pair of themed backends.

#[cfg(all(feature = "adwaita", feature = "material"))]
mod defaults;
mod scheme;
#[cfg(all(test, feature = "adwaita", feature = "material"))]
mod tests;

use iced_component_core::anim::MotionRuntime;

use crate::backend::ThemeBackend;

pub use scheme::ColorScheme;

/// Active side of a generic two-backend adapter.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum BackendSelection {
    /// Use the first backend type.
    #[default]
    First,
    /// Use the second backend type.
    Second,
}

impl BackendSelection {
    const fn toggled(self) -> Self {
        match self {
            Self::First => Self::Second,
            Self::Second => Self::First,
        }
    }
}

/// Named selection used by the built-in Adwaita + Material adapter.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ThemeFamily {
    /// Render components from `iced-adwaita`.
    #[default]
    Adwaita,
    /// Render components from `iced-material`.
    Material,
}

impl From<ThemeFamily> for BackendSelection {
    fn from(family: ThemeFamily) -> Self {
        match family {
            ThemeFamily::Adwaita => Self::First,
            ThemeFamily::Material => Self::Second,
        }
    }
}

/// Persistent contexts for exactly two selected themed backends.
pub struct AdapterContext<A, B>
where
    A: ThemeBackend,
    B: ThemeBackend,
{
    selection: BackendSelection,
    first: A::Context,
    second: B::Context,
}

impl<A, B> AdapterContext<A, B>
where
    A: ThemeBackend,
    B: ThemeBackend,
{
    /// Creates an adapter from independently configured backend contexts.
    #[must_use]
    pub fn from_backends(
        selection: impl Into<BackendSelection>,
        first: A::Context,
        second: B::Context,
    ) -> Self {
        Self {
            selection: selection.into(),
            first,
            second,
        }
    }

    /// Returns the active backend side.
    #[must_use]
    pub const fn selection(&self) -> BackendSelection {
        self.selection
    }

    /// Returns the first concrete backend context.
    #[must_use]
    pub const fn first(&self) -> &A::Context {
        &self.first
    }

    /// Returns the second concrete backend context.
    #[must_use]
    pub const fn second(&self) -> &B::Context {
        &self.second
    }

    /// Returns the active backend's color scheme.
    #[must_use]
    pub fn color_scheme(&self) -> ColorScheme {
        match self.selection {
            BackendSelection::First => A::color_scheme(&self.first),
            BackendSelection::Second => B::color_scheme(&self.second),
        }
    }

    /// Returns the active backend's reduced-motion preference.
    #[must_use]
    pub fn reduce_motion(&self) -> bool {
        match self.selection {
            BackendSelection::First => A::reduce_motion(&self.first),
            BackendSelection::Second => B::reduce_motion(&self.second),
        }
    }
}

impl<A, B> Clone for AdapterContext<A, B>
where
    A: ThemeBackend,
    B: ThemeBackend,
    A::Context: Clone,
    B::Context: Clone,
{
    fn clone(&self) -> Self {
        Self::from_backends(self.selection, self.first.clone(), self.second.clone())
    }
}

/// Mutable runtime inputs for a generic backend pair.
pub struct AdapterUpdateCx<'a, A, B>
where
    A: ThemeBackend,
    B: ThemeBackend,
{
    runtime: &'a mut MotionRuntime,
    context: &'a mut AdapterContext<A, B>,
}

impl<'a, A, B> AdapterUpdateCx<'a, A, B>
where
    A: ThemeBackend,
    B: ThemeBackend,
{
    /// Creates an adapter update context.
    pub fn new(runtime: &'a mut MotionRuntime, context: &'a mut AdapterContext<A, B>) -> Self {
        Self { runtime, context }
    }

    /// Returns the active backend side.
    #[must_use]
    pub const fn selection(&self) -> BackendSelection {
        self.context.selection
    }

    /// Returns the adapter context.
    #[must_use]
    pub const fn context(&self) -> &AdapterContext<A, B> {
        self.context
    }

    /// Selects a backend without starting a cross-theme transition.
    pub fn set_selection(&mut self, selection: BackendSelection) -> bool {
        let changed = self.context.selection != selection;
        self.context.selection = selection;
        changed
    }

    /// Selects the other backend without starting a transition.
    pub fn toggle_selection(&mut self) -> BackendSelection {
        let selection = self.context.selection.toggled();
        self.context.selection = selection;
        selection
    }

    /// Returns the active backend's color scheme.
    #[must_use]
    pub fn color_scheme(&self) -> ColorScheme {
        self.context.color_scheme()
    }

    /// Applies one color scheme to both selected backend contexts.
    pub fn set_color_scheme(&mut self, color_scheme: ColorScheme) -> bool {
        let first = A::set_color_scheme(&mut self.first(), color_scheme);
        let second = B::set_color_scheme(&mut self.second(), color_scheme);
        first || second
    }

    /// Toggles the color scheme of both selected backend contexts.
    pub fn toggle_color_scheme(&mut self) -> ColorScheme {
        let color_scheme = self.color_scheme().toggled();
        let _ = self.set_color_scheme(color_scheme);
        color_scheme
    }

    /// Returns whether the active backend reduces non-essential motion.
    #[must_use]
    pub fn reduce_motion(&self) -> bool {
        self.context.reduce_motion()
    }

    /// Applies reduced motion to both selected backend contexts.
    pub fn set_reduce_motion(&mut self, reduce_motion: bool) {
        A::set_reduce_motion(&mut self.first(), reduce_motion);
        B::set_reduce_motion(&mut self.second(), reduce_motion);
    }

    /// Toggles reduced motion for both selected backend contexts.
    pub fn toggle_reduce_motion(&mut self) -> bool {
        let reduce_motion = !self.reduce_motion();
        self.set_reduce_motion(reduce_motion);
        reduce_motion
    }

    /// Creates the first backend's concrete update context.
    pub fn first(&mut self) -> A::UpdateCx<'_> {
        A::update_cx(self.runtime, &mut self.context.first)
    }

    /// Creates the second backend's concrete update context.
    pub fn second(&mut self) -> B::UpdateCx<'_> {
        B::update_cx(self.runtime, &mut self.context.second)
    }
}

/// Read-only runtime inputs for a generic backend pair.
pub struct AdapterViewCx<'a, A, B>
where
    A: ThemeBackend,
    B: ThemeBackend,
{
    runtime: &'a MotionRuntime,
    context: &'a AdapterContext<A, B>,
}

impl<'a, A, B> AdapterViewCx<'a, A, B>
where
    A: ThemeBackend,
    B: ThemeBackend,
{
    /// Creates an adapter view context.
    #[must_use]
    pub const fn new(runtime: &'a MotionRuntime, context: &'a AdapterContext<A, B>) -> Self {
        Self { runtime, context }
    }

    /// Returns the active backend side.
    #[must_use]
    pub const fn selection(&self) -> BackendSelection {
        self.context.selection
    }

    /// Returns the adapter context.
    #[must_use]
    pub const fn context(&self) -> &AdapterContext<A, B> {
        self.context
    }

    /// Returns the active backend's color scheme.
    #[must_use]
    pub fn color_scheme(&self) -> ColorScheme {
        self.context.color_scheme()
    }

    /// Returns whether the active backend reduces non-essential motion.
    #[must_use]
    pub fn reduce_motion(&self) -> bool {
        self.context.reduce_motion()
    }

    /// Creates the first backend's concrete view context.
    #[must_use]
    pub fn first(&self) -> A::ViewCx<'_> {
        A::view_cx(self.runtime, &self.context.first)
    }

    /// Creates the second backend's concrete view context.
    #[must_use]
    pub fn second(&self) -> B::ViewCx<'_> {
        B::view_cx(self.runtime, &self.context.second)
    }
}

#[cfg(all(feature = "adwaita", feature = "material"))]
pub use defaults::{Context, UpdateCx, ViewCx};
