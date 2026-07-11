//! Shared adapter context for selecting a themed component backend.

use iced_component_core::anim::MotionRuntime;

/// The themed component family rendered by an adapter.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ThemeFamily {
    /// Render components from `iced-adwaita`.
    #[default]
    Adwaita,
    /// Render components from `iced-material`.
    Material,
}

impl ThemeFamily {
    const fn toggled(self) -> Self {
        match self {
            Self::Adwaita => Self::Material,
            Self::Material => Self::Adwaita,
        }
    }
}

/// Theme contexts retained by the adapter.
///
/// Both contexts remain alive across a family switch. This preserves runtime
/// theme edits while the adapter changes the rendered component implementation.
#[derive(Clone)]
pub struct Context {
    family: ThemeFamily,
    adwaita: iced_adwaita::Context,
    material: iced_material::context::Context,
}

impl Context {
    /// Creates a light context with the selected component family.
    #[must_use]
    pub fn new(family: ThemeFamily) -> Self {
        Self {
            family,
            adwaita: iced_adwaita::Context::light(),
            material: iced_material::context::Context::light(),
        }
    }

    /// Returns the active component family.
    #[must_use]
    pub const fn family(&self) -> ThemeFamily {
        self.family
    }

    /// Returns the retained Adwaita context.
    #[must_use]
    pub const fn adwaita(&self) -> &iced_adwaita::Context {
        &self.adwaita
    }

    /// Returns the retained Material context.
    #[must_use]
    pub const fn material(&self) -> &iced_material::context::Context {
        &self.material
    }

    /// Returns the adapter-wide reduced-motion preference.
    #[must_use]
    pub fn reduce_motion(&self) -> bool {
        debug_assert_eq!(self.adwaita.reduce_motion(), self.material.reduce_motion());
        self.adwaita.reduce_motion()
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new(ThemeFamily::default())
    }
}

/// Mutable runtime inputs used by adapter components.
pub struct UpdateCx<'a> {
    runtime: &'a mut MotionRuntime,
    context: &'a mut Context,
}

impl<'a> UpdateCx<'a> {
    /// Creates an adapter update context.
    pub fn new(runtime: &'a mut MotionRuntime, context: &'a mut Context) -> Self {
        Self { runtime, context }
    }

    /// Returns the active component family.
    #[must_use]
    pub const fn family(&self) -> ThemeFamily {
        self.context.family
    }

    /// Selects a component family without starting a cross-theme transition.
    pub fn set_family(&mut self, family: ThemeFamily) -> bool {
        let changed = self.context.family != family;
        self.context.family = family;
        changed
    }

    /// Selects the other component family without a transition.
    pub fn toggle_family(&mut self) -> ThemeFamily {
        let family = self.context.family.toggled();
        self.context.family = family;
        family
    }

    /// Returns whether non-essential motion is reduced.
    #[must_use]
    pub fn reduce_motion(&self) -> bool {
        self.context.reduce_motion()
    }

    /// Updates reduced motion for every retained themed context.
    pub fn set_reduce_motion(&mut self, reduce_motion: bool) {
        self.adwaita().set_reduce_motion(reduce_motion);
        self.material().set_reduce_motion(reduce_motion);
    }

    /// Toggles reduced motion for every retained themed context.
    pub fn toggle_reduce_motion(&mut self) -> bool {
        let reduce_motion = !self.reduce_motion();
        self.set_reduce_motion(reduce_motion);
        reduce_motion
    }

    pub(crate) fn adwaita(&mut self) -> iced_adwaita::context::UpdateCx<'_> {
        iced_adwaita::context::UpdateCx::new(self.runtime, &mut self.context.adwaita)
    }

    pub(crate) fn material(&mut self) -> iced_material::context::UpdateCx<'_> {
        iced_material::context::UpdateCx::new(self.runtime, &mut self.context.material)
    }
}

/// Read-only runtime inputs used by adapter views.
pub struct ViewCx<'a> {
    runtime: &'a MotionRuntime,
    context: &'a Context,
}

impl<'a> ViewCx<'a> {
    /// Creates an adapter view context.
    #[must_use]
    pub const fn new(runtime: &'a MotionRuntime, context: &'a Context) -> Self {
        Self { runtime, context }
    }

    /// Returns the active component family.
    #[must_use]
    pub const fn family(&self) -> ThemeFamily {
        self.context.family
    }

    /// Returns whether non-essential motion is reduced.
    #[must_use]
    pub fn reduce_motion(&self) -> bool {
        self.context.reduce_motion()
    }

    pub(crate) const fn adwaita(&self) -> iced_adwaita::context::ViewCx<'_> {
        iced_adwaita::context::ViewCx::new(self.runtime, &self.context.adwaita)
    }

    pub(crate) const fn material(&self) -> iced_material::context::ViewCx<'_> {
        iced_material::context::ViewCx::new(self.runtime, &self.context.material)
    }
}

#[cfg(test)]
mod tests {
    use iced_component_core::anim::MotionRuntime;

    use super::{Context, ThemeFamily, UpdateCx};

    #[test]
    fn family_switch_is_direct_and_preserves_theme_revisions() {
        let mut runtime = MotionRuntime::new();
        let mut context = Context::default();
        let adwaita_revision = context.adwaita().style_revision();
        let material_revision = context.material().style_revision();

        let changed = UpdateCx::new(&mut runtime, &mut context).set_family(ThemeFamily::Material);

        assert!(changed);
        assert_eq!(context.family(), ThemeFamily::Material);
        assert_eq!(context.adwaita().style_revision(), adwaita_revision);
        assert_eq!(context.material().style_revision(), material_revision);
        assert_eq!(runtime.motion_count(), 0);
    }

    #[test]
    fn reduced_motion_is_synchronized_across_backends() {
        let mut runtime = MotionRuntime::new();
        let mut context = Context::default();

        UpdateCx::new(&mut runtime, &mut context).set_reduce_motion(true);

        assert!(context.reduce_motion());
        assert!(context.adwaita().reduce_motion());
        assert!(context.material().reduce_motion());
    }
}
