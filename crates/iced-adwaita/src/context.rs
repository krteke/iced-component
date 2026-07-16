mod transition;

use iced_component_core::{
    anim::MotionRuntime,
    component::{
        ComponentContext, ComponentUpdateCx, ComponentViewCx, StyleChange, StyleRevision,
        animation::AnimationOverrides,
    },
};

use crate::theme::{ThemeLoadError, ThemePack};

pub use transition::{StyleTransition, StyleTransitionBuilder};
use transition::{StyleTransitionSnapshot, StyleTransitionState};

/// The theme mode for the Adwaita component context.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ThemeMode {
    /// The dark theme mode.
    Dark,
    /// The light theme mode.
    Light,
}

/// The theme for the Adwaita component context.
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Theme {
    mode: ThemeMode,
    light: ThemePack,
    dark: ThemePack,
}

impl Theme {
    /// Creates a new theme from the given theme mode.
    #[must_use]
    pub fn new(mode: ThemeMode) -> Self {
        Self::from_packs(mode, ThemePack::light(), ThemePack::dark())
    }

    /// Creates a theme from explicit light and dark packs.
    #[must_use]
    pub const fn from_packs(mode: ThemeMode, light: ThemePack, dark: ThemePack) -> Self {
        Self { mode, light, dark }
    }

    /// Creates a new dark theme.
    #[must_use]
    pub fn dark() -> Self {
        Self::new(ThemeMode::Dark)
    }

    /// Creates a new light theme.
    #[must_use]
    pub fn light() -> Self {
        Self::new(ThemeMode::Light)
    }

    /// Returns a reference to the theme pack.
    #[must_use]
    pub const fn pack(&self) -> &ThemePack {
        self.pack_for(self.mode)
    }

    /// Returns the theme mode.
    #[must_use]
    pub const fn mode(&self) -> ThemeMode {
        self.mode
    }

    /// Returns a reference to the pack for a specific mode.
    #[must_use]
    pub const fn pack_for(&self, mode: ThemeMode) -> &ThemePack {
        match mode {
            ThemeMode::Light => &self.light,
            ThemeMode::Dark => &self.dark,
        }
    }

    /// Returns a mutable reference to the pack for a specific mode.
    pub fn pack_for_mut(&mut self, mode: ThemeMode) -> &mut ThemePack {
        match mode {
            ThemeMode::Light => &mut self.light,
            ThemeMode::Dark => &mut self.dark,
        }
    }

    /// Replaces the pack for a specific mode.
    pub fn set_pack(&mut self, mode: ThemeMode, pack: ThemePack) {
        *self.pack_for_mut(mode) = pack;
    }

    /// Loads and replaces the pack for a specific mode from TOML.
    pub fn set_pack_from_toml(
        &mut self,
        mode: ThemeMode,
        input: &str,
    ) -> Result<(), ThemeLoadError> {
        self.set_pack(mode, ThemePack::try_from_toml(input)?);
        Ok(())
    }

    fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
        };
    }
}

/// Shared Adwaita component inputs.
#[derive(Clone)]
pub struct Context {
    core: ComponentContext,
    theme: Theme,
    style_transition: StyleTransitionState,
}

impl Context {
    /// Creates an Adwaita component context for a theme mode.
    #[must_use]
    pub fn new(mode: ThemeMode) -> Self {
        Self::from_theme(Theme::new(mode))
    }

    /// Creates an Adwaita component context from an explicit theme.
    #[must_use]
    pub fn from_theme(theme: Theme) -> Self {
        Self {
            core: ComponentContext::new(),
            theme,
            style_transition: StyleTransitionState::new(),
        }
    }

    /// Creates the embedded Adwaita light context.
    #[must_use]
    pub fn light() -> Self {
        Self::new(ThemeMode::Light)
    }

    /// Creates the embedded Adwaita dark context.
    #[must_use]
    pub fn dark() -> Self {
        Self::new(ThemeMode::Dark)
    }

    /// Returns the underlying core component context.
    #[must_use]
    pub const fn core(&self) -> &ComponentContext {
        &self.core
    }

    /// Returns the current Adwaita theme pack.
    #[must_use]
    pub const fn theme(&self) -> &Theme {
        &self.theme
    }

    /// Returns sparse application-level component animation overrides.
    #[must_use]
    pub const fn animation_overrides(&self) -> &AnimationOverrides {
        self.core.animation_overrides()
    }

    /// Returns the configured transition for same-theme style changes.
    #[must_use]
    pub const fn style_transition(&self) -> &StyleTransition {
        self.style_transition.config()
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

    /// Returns the application-owned motion runtime.
    #[must_use]
    pub(crate) fn runtime(&self) -> &MotionRuntime {
        self.runtime
    }

    /// Returns the current Adwaita component context.
    #[must_use]
    pub const fn context(&self) -> &Context {
        self.context
    }

    /// Returns the current Adwaita theme pack.
    #[must_use]
    pub const fn theme(&self) -> &Theme {
        self.context.theme()
    }

    /// Toggles the theme mode between dark and light.
    pub fn toggle_theme(&mut self) -> StyleChange {
        let previous_mode = self.context.theme.mode;
        let previous_revision = self.context.style_revision();
        let previous_pack = self.context.theme.pack().clone();
        self.context.theme.toggle_mode();
        let change = self.context.core.bump_style_revision();
        self.start_style_transition(previous_pack);
        trace_theme_change(
            "toggle_theme",
            previous_mode,
            self.context.theme.mode,
            previous_revision,
            self.context.style_revision(),
        );
        change
    }

    /// Replaces the Adwaita theme pack and invalidates style-dependent motion.
    pub fn set_theme(&mut self, theme: Theme) -> StyleChange {
        let previous_mode = self.context.theme.mode;
        let previous_revision = self.context.style_revision();
        let previous_pack = self.context.theme.pack().clone();
        let current_mode = theme.mode;
        self.context.theme = theme;
        let change = self.context.core.bump_style_revision();
        self.start_style_transition(previous_pack);
        trace_theme_change(
            "set_theme",
            previous_mode,
            current_mode,
            previous_revision,
            self.context.style_revision(),
        );
        change
    }

    /// Applies local theme changes and invalidates style-dependent motion.
    pub fn patch_theme(&mut self, patch: impl FnOnce(&mut ThemePack)) -> StyleChange {
        let mode = self.context.theme.mode;
        let previous_revision = self.context.style_revision();
        let previous_pack = self.context.theme.pack().clone();
        patch(self.context.theme.pack_for_mut(mode));
        let change = self.context.core.bump_style_revision();
        self.start_style_transition(previous_pack);

        trace_theme_change(
            "patch_theme",
            mode,
            mode,
            previous_revision,
            self.context.style_revision(),
        );
        change
    }

    /// Applies local theme changes for a specific mode.
    pub fn patch_theme_for(
        &mut self,
        mode: ThemeMode,
        patch: impl FnOnce(&mut ThemePack),
    ) -> StyleChange {
        let previous_revision = self.context.style_revision();
        let previous_pack =
            (mode == self.context.theme.mode).then(|| self.context.theme.pack().clone());
        patch(self.context.theme.pack_for_mut(mode));
        let change = self.context.core.bump_style_revision();
        if let Some(previous_pack) = previous_pack {
            self.start_style_transition(previous_pack);
        }

        trace_theme_change(
            "patch_theme_for",
            self.context.theme.mode,
            self.context.theme.mode,
            previous_revision,
            self.context.style_revision(),
        );
        change
    }

    /// Replaces the pack for a specific mode and invalidates style-dependent motion.
    pub fn set_theme_pack(&mut self, mode: ThemeMode, pack: ThemePack) -> StyleChange {
        let previous_revision = self.context.style_revision();
        let previous_pack =
            (mode == self.context.theme.mode).then(|| self.context.theme.pack().clone());
        self.context.theme.set_pack(mode, pack);
        let change = self.context.core.bump_style_revision();
        if let Some(previous_pack) = previous_pack {
            self.start_style_transition(previous_pack);
        }

        trace_theme_change(
            "set_theme_pack",
            self.context.theme.mode,
            self.context.theme.mode,
            previous_revision,
            self.context.style_revision(),
        );
        change
    }

    /// Loads and replaces the pack for a specific mode from TOML.
    pub fn set_theme_pack_from_toml(
        &mut self,
        mode: ThemeMode,
        input: &str,
    ) -> Result<StyleChange, ThemeLoadError> {
        let pack = ThemePack::try_from_toml(input)?;
        Ok(self.set_theme_pack(mode, pack))
    }

    /// Returns sparse application-level component animation overrides.
    #[must_use]
    pub const fn animation_overrides(&self) -> &AnimationOverrides {
        self.context.animation_overrides()
    }

    /// Installs or replaces one typed component animation override.
    pub fn set_animation_override<T: 'static>(&mut self, animations: T) {
        self.context.core.animation_overrides_mut().set(animations);
    }

    /// Removes one typed component animation override.
    pub fn remove_animation_override<T: 'static>(&mut self) -> bool {
        self.context.core.animation_overrides_mut().remove::<T>()
    }

    /// Replaces the transition used for subsequent same-theme style changes.
    pub fn set_style_transition(&mut self, transition: StyleTransition) {
        self.context.style_transition.set_config(transition);
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
        if reduce_motion {
            self.context.style_transition.finish(self.runtime);
        }
    }

    /// Toggles the reduced-motion preference.
    pub fn toggle_reduce_motion(&mut self) {
        self.context.core.toggle_reduce_motion();
        if self.context.reduce_motion() {
            self.context.style_transition.finish(self.runtime);
        }
    }

    pub(crate) fn style_transition(&self) -> Option<StyleTransitionSnapshot<'_>> {
        self.context.style_transition.snapshot(self.runtime)
    }

    fn start_style_transition(&mut self, previous_pack: ThemePack) {
        let reduce_motion = self.context.reduce_motion();
        self.context
            .style_transition
            .start(previous_pack, self.runtime, reduce_motion);
    }
}

#[cfg(feature = "tracing")]
fn trace_theme_change(
    action: &'static str,
    previous_mode: ThemeMode,
    current_mode: ThemeMode,
    previous_revision: StyleRevision,
    current_revision: StyleRevision,
) {
    tracing::debug!(
        target: "iced_adwaita::context",
        action,
        ?previous_mode,
        ?current_mode,
        ?previous_revision,
        ?current_revision,
        "adwaita theme changed"
    );
}

#[cfg(not(feature = "tracing"))]
fn trace_theme_change(
    _action: &'static str,
    _previous_mode: ThemeMode,
    _current_mode: ThemeMode,
    _previous_revision: StyleRevision,
    _current_revision: StyleRevision,
) {
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

    /// Returns the application-owned motion runtime.
    #[must_use]
    pub(crate) const fn runtime(&self) -> &MotionRuntime {
        self.runtime
    }

    /// Returns the current Adwaita component context.
    #[must_use]
    pub const fn context(&self) -> &Context {
        self.context
    }

    /// Returns the current Adwaita theme pack.
    #[must_use]
    pub const fn theme(&self) -> &Theme {
        &self.context.theme
    }

    /// Returns the current theme mode.
    #[must_use]
    pub const fn theme_mode(&self) -> ThemeMode {
        self.context.theme.mode()
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

    /// Returns sparse application-level component animation overrides.
    #[must_use]
    pub const fn animation_overrides(&self) -> &AnimationOverrides {
        self.context.animation_overrides()
    }

    pub(crate) fn style_transition(&self) -> Option<StyleTransitionSnapshot<'_>> {
        self.context.style_transition.snapshot(self.runtime)
    }
}

#[cfg(test)]
mod tests {
    use iced_component_core::anim::MotionRuntime;
    use spectrum_theme::Color;

    use crate::{context::Theme, theme::ThemePack};

    use super::{Context, ThemeMode, UpdateCx, ViewCx};

    #[test]
    fn replacing_theme_bumps_style_revision() {
        let mut runtime = MotionRuntime::new();
        let mut context = Context::light();
        let revision = context.style_revision();

        UpdateCx::new(&mut runtime, &mut context).set_theme(Theme::dark());

        assert_ne!(context.style_revision(), revision);
    }

    #[test]
    fn patching_theme_updates_theme_and_revision() {
        let mut runtime = MotionRuntime::new();
        let mut context = Context::light();
        let revision = context.style_revision();

        UpdateCx::new(&mut runtime, &mut context).patch_theme(|theme| {
            theme.spinner.foreground = Color::new(1, 2, 3);
        });

        assert_eq!(
            context.theme().pack().spinner.foreground,
            Color::new(1, 2, 3)
        );
        assert_ne!(context.style_revision(), revision);
    }

    #[test]
    fn toggling_theme_preserves_patched_mode_packs() {
        let mut runtime = MotionRuntime::new();
        let mut context = Context::light();

        UpdateCx::new(&mut runtime, &mut context).patch_theme(|theme| {
            theme.spinner.foreground = Color::new(1, 2, 3);
        });
        {
            let mut cx = UpdateCx::new(&mut runtime, &mut context);
            cx.toggle_theme();
            cx.toggle_theme();
        }

        assert_eq!(context.theme().mode(), ThemeMode::Light);
        assert_eq!(
            context.theme().pack().spinner.foreground,
            Color::new(1, 2, 3)
        );
    }

    #[test]
    fn replacing_inactive_mode_pack_is_used_after_toggle() {
        let mut runtime = MotionRuntime::new();
        let mut context = Context::light();
        let mut dark = ThemePack::dark();
        dark.spinner.foreground = Color::new(9, 8, 7);

        {
            let mut cx = UpdateCx::new(&mut runtime, &mut context);
            cx.set_theme_pack(ThemeMode::Dark, dark);
            cx.toggle_theme();
        }

        assert_eq!(context.theme().mode(), ThemeMode::Dark);
        assert_eq!(
            context.theme().pack().spinner.foreground,
            Color::new(9, 8, 7)
        );
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
    fn reduced_motion_finishes_context_style_transition() {
        let mut runtime = MotionRuntime::new();
        let mut context = Context::light();

        {
            let mut update = UpdateCx::new(&mut runtime, &mut context);
            update.set_reduce_motion(true);
            update.toggle_theme();
        }

        assert!(ViewCx::new(&runtime, &context).style_transition().is_none());
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
