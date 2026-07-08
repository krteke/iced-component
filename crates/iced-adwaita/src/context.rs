use iced_component_core::{
    anim::MotionRuntime,
    component::{ComponentContext, ComponentUpdateCx, ComponentViewCx, StyleRevision},
};

use crate::theme::ThemePack;

/// Shared Adwaita component inputs.
#[derive(Clone)]
pub struct Context {
    core: ComponentContext,
    theme: ThemePack,
}

impl Context {
    /// Creates an Adwaita component context from an explicit theme pack.
    #[must_use]
    pub fn new(theme: ThemePack) -> Self {
        Self {
            core: ComponentContext::new(),
            theme,
        }
    }

    /// Creates the embedded Adwaita light context.
    #[must_use]
    pub fn light() -> Self {
        Self::new(ThemePack::light())
    }

    /// Returns the underlying core component context.
    #[must_use]
    pub const fn core(&self) -> &ComponentContext {
        &self.core
    }

    /// Returns the current Adwaita theme pack.
    #[must_use]
    pub const fn theme(&self) -> &ThemePack {
        &self.theme
    }

    /// Returns the current style revision.
    #[must_use]
    pub const fn style_revision(&self) -> StyleRevision {
        self.core.style_revision()
    }

    /// Returns whether non-essential motion should be reduced.
    #[must_use]
    pub const fn reduce_motion(&self) -> bool {
        self.core.reduce_motion()
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::light()
    }
}

/// Mutable inputs used while applying Adwaita component events.
pub struct UpdateCx<'a> {
    runtime: &'a mut MotionRuntime,
    context: &'a mut Context,
}

impl<'a> UpdateCx<'a> {
    /// Creates an Adwaita update context.
    pub fn new(runtime: &'a mut MotionRuntime, context: &'a mut Context) -> Self {
        Self { runtime, context }
    }

    /// Creates a temporary core update context for shared motion helpers.
    pub fn core(&mut self) -> ComponentUpdateCx<'_> {
        ComponentUpdateCx::new(self.runtime, &mut self.context.core)
    }

    /// Returns the current Adwaita component context.
    #[must_use]
    pub const fn context(&self) -> &Context {
        self.context
    }

    /// Returns the current Adwaita theme pack.
    #[must_use]
    pub const fn theme(&self) -> &ThemePack {
        self.context.theme()
    }

    /// Replaces the Adwaita theme pack and invalidates style-dependent motion.
    pub fn set_theme_pack(&mut self, theme: ThemePack) {
        self.context.theme = theme;
        self.context.core.bump_style_revision();
    }

    /// Applies local theme changes and invalidates style-dependent motion.
    pub fn patch_theme(&mut self, patch: impl FnOnce(&mut ThemePack)) {
        patch(&mut self.context.theme);
        self.context.core.bump_style_revision();
    }

    /// Returns the current style revision.
    #[must_use]
    pub const fn style_revision(&self) -> StyleRevision {
        self.context.style_revision()
    }

    /// Returns whether non-essential motion should be reduced.
    #[must_use]
    pub const fn reduce_motion(&self) -> bool {
        self.context.reduce_motion()
    }

    /// Updates whether non-essential motion should be reduced.
    pub fn set_reduce_motion(&mut self, reduce_motion: bool) {
        self.context.core.set_reduce_motion(reduce_motion);
    }

    /// Toggles the reduced-motion preference.
    pub fn toggle_reduce_motion(&mut self) {
        self.context.core.toggle_reduce_motion();
    }
}

/// Read-only inputs used while rendering Adwaita component views.
pub struct ViewCx<'a> {
    runtime: &'a MotionRuntime,
    context: &'a Context,
}

impl<'a> ViewCx<'a> {
    /// Creates an Adwaita view context.
    #[must_use]
    pub const fn new(runtime: &'a MotionRuntime, context: &'a Context) -> Self {
        Self { runtime, context }
    }

    /// Creates a temporary core view context for shared motion helpers.
    #[must_use]
    pub const fn core(&self) -> ComponentViewCx<'_> {
        ComponentViewCx::new(self.runtime, self.context.core())
    }

    /// Returns the current Adwaita component context.
    #[must_use]
    pub const fn context(&self) -> &Context {
        self.context
    }

    /// Returns the current Adwaita theme pack.
    #[must_use]
    pub const fn theme(&self) -> &ThemePack {
        self.context.theme()
    }

    /// Returns the current style revision.
    #[must_use]
    pub const fn style_revision(&self) -> StyleRevision {
        self.context.style_revision()
    }

    /// Returns whether non-essential motion should be reduced.
    #[must_use]
    pub const fn reduce_motion(&self) -> bool {
        self.context.reduce_motion()
    }
}

#[cfg(test)]
mod tests {
    use iced_component_core::anim::MotionRuntime;
    use spectrum_theme::Color;

    use super::{Context, ThemePack, UpdateCx, ViewCx};

    #[test]
    fn replacing_theme_bumps_style_revision() {
        let mut runtime = MotionRuntime::new();
        let mut context = Context::light();
        let revision = context.style_revision();

        UpdateCx::new(&mut runtime, &mut context).set_theme_pack(ThemePack::light());

        assert_ne!(context.style_revision(), revision);
    }

    #[test]
    fn patching_theme_updates_theme_and_revision() {
        let mut runtime = MotionRuntime::new();
        let mut context = Context::light();
        let revision = context.style_revision();

        UpdateCx::new(&mut runtime, &mut context).patch_theme(|theme| {
            theme.spinner.color = Color::new(1, 2, 3);
        });

        assert_eq!(context.theme().spinner.color, Color::new(1, 2, 3));
        assert_ne!(context.style_revision(), revision);
    }

    #[test]
    fn reduce_motion_is_owned_by_core_context() {
        let mut runtime = MotionRuntime::new();
        let mut context = Context::light();

        UpdateCx::new(&mut runtime, &mut context).set_reduce_motion(true);

        assert!(context.reduce_motion());
        assert!(context.core().reduce_motion());
    }

    #[test]
    fn update_context_exposes_core_update_inputs() {
        let mut runtime = MotionRuntime::new();
        let mut context = Context::light();
        let mut cx = UpdateCx::new(&mut runtime, &mut context);

        cx.set_reduce_motion(true);
        let core = cx.core();

        assert!(core.reduce_motion());
    }

    #[test]
    fn view_context_exposes_core_view_inputs() {
        let runtime = MotionRuntime::new();
        let context = Context::light();
        let cx = ViewCx::new(&runtime, &context);

        assert_eq!(
            cx.core().context().style_revision(),
            context.style_revision()
        );
    }
}
