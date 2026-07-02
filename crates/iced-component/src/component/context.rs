use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

use aura_anim::prelude::MotionRuntime;

use crate::theme::{ThemeContext, ThemeLoadError, ThemePack};

/// Monotonic marker for the theme snapshot carried by a [`ComponentContext`].
///
/// This is a cache-invalidating identity, not a user-visible theme version.
/// Components use it to decide whether a registered runtime motion value still
/// belongs to the theme snapshot used by the current update or view pass.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct ThemeRevision(u64);

impl ThemeRevision {
    fn next() -> Self {
        static NEXT_REVISION: AtomicU64 = AtomicU64::new(1);

        Self(NEXT_REVISION.fetch_add(1, Ordering::Relaxed))
    }
}

/// Shared component inputs that can change while the application is running.
///
/// This is the only source components should use for theme-dependent values.
/// Constructors should store stable configuration only; theme tokens are
/// resolved during `update`, `sync`, `snapshot`, or `view` with the current
/// context.
#[derive(Clone)]
pub struct ComponentContext {
    theme: ThemeContext,
    theme_revision: ThemeRevision,
    reduce_motion: bool,
}

impl ComponentContext {
    /// Creates a component context from explicit inputs.
    #[must_use]
    pub fn new(theme: ThemeContext) -> Self {
        Self {
            theme,
            theme_revision: ThemeRevision::next(),
            reduce_motion: false,
        }
    }

    /// Creates a context from the default Adwaita-like theme.
    #[must_use]
    pub fn adwaita() -> Self {
        Self::new(ThemeContext::adwaita())
    }

    /// Returns the theme context.
    #[must_use]
    pub const fn theme(&self) -> &ThemeContext {
        &self.theme
    }

    /// Returns the revision of the current theme snapshot.
    #[must_use]
    pub const fn theme_revision(&self) -> ThemeRevision {
        self.theme_revision
    }

    /// Replaces the theme context.
    pub fn set_theme(&mut self, theme: ThemeContext) {
        self.theme = theme;
        self.bump_theme_revision();
    }

    /// Replaces the theme with an owned theme pack.
    pub fn set_theme_pack(&mut self, theme: ThemePack) {
        self.theme = ThemeContext::new(theme);
        self.bump_theme_revision();
    }

    /// Loads a theme from a TOML file at the given path.
    pub fn load_theme_from(&mut self, path: impl Into<PathBuf>) -> Result<(), ThemeLoadError> {
        let config = fs::read_to_string(path.into())?;
        let theme_pack = ThemePack::try_from_toml(&config)?;

        self.set_theme_pack(theme_pack);
        Ok(())
    }

    /// Resets this context to the default Adwaita-like theme.
    pub fn reset_theme(&mut self) {
        self.theme = ThemeContext::adwaita();
        self.bump_theme_revision();
    }

    /// Applies local theme token changes to this context.
    pub fn patch_theme(&mut self, patch: impl FnOnce(&mut ThemePack)) {
        self.theme = self.theme.clone().with_patch(patch);
        self.bump_theme_revision();
    }

    /// Returns a context with local theme token changes applied.
    ///
    /// Store scoped contexts that need animated continuity. Recreating a scoped
    /// context every frame produces a fresh [`ThemeRevision`], so existing
    /// motion values will be treated as stale by design.
    #[must_use]
    pub fn scoped_theme(&self, patch: impl FnOnce(&mut ThemePack)) -> Self {
        Self {
            theme: self.theme.scoped(patch),
            theme_revision: ThemeRevision::next(),
            reduce_motion: self.reduce_motion,
        }
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

    fn bump_theme_revision(&mut self) {
        self.theme_revision = ThemeRevision::next();
    }
}

impl Default for ComponentContext {
    fn default() -> Self {
        Self::adwaita()
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
}

#[cfg(test)]
mod tests {
    use spectrum_theme::Color;

    use crate::component::ComponentContext;

    #[test]
    fn scoped_theme_keeps_theme_override() {
        let context = ComponentContext::adwaita();
        let parent_revision = context.theme_revision();
        let scoped_bg = Color::new(221, 238, 255);
        let scoped =
            context.scoped_theme(|theme| theme.button.standard_filled.hover.bg = scoped_bg);

        assert_ne!(scoped.theme_revision(), parent_revision);
        assert_eq!(
            scoped.theme().theme().button.standard_filled.hover.bg,
            scoped_bg
        );
    }

    #[test]
    fn context_can_change_theme_and_reduce_motion_at_runtime() {
        let accent = Color::new(26, 95, 180);
        let mut context = ComponentContext::adwaita();
        let initial_revision = context.theme_revision();

        context.patch_theme(|theme| theme.button.suggested_filled.idle.bg = accent);
        let patched_revision = context.theme_revision();
        context.set_reduce_motion(true);

        assert_ne!(patched_revision, initial_revision);
        assert_eq!(context.theme_revision(), patched_revision);
        assert_eq!(
            context.theme().theme().button.suggested_filled.idle.bg,
            accent
        );
        assert!(context.reduce_motion());
    }

    #[test]
    fn theme_changes_are_scoped_to_one_component_context() {
        let accent = Color::new(26, 95, 180);
        let mut first = ComponentContext::adwaita();
        let second = ComponentContext::adwaita();

        first.patch_theme(|theme| theme.button.suggested_filled.idle.bg = accent);

        assert_eq!(
            first.theme().theme().button.suggested_filled.idle.bg,
            accent
        );
        assert_ne!(
            second.theme().theme().button.suggested_filled.idle.bg,
            accent
        );
    }

    #[test]
    fn theme_replacement_paths_bump_revision() {
        let mut context = ComponentContext::adwaita();
        let initial = context.theme_revision();

        context.set_theme(crate::theme::ThemeContext::adwaita());
        let after_set_theme = context.theme_revision();
        context.set_theme_pack(crate::theme::ThemePack::adwaita());
        let after_set_pack = context.theme_revision();
        context.reset_theme();

        assert_ne!(after_set_theme, initial);
        assert_ne!(after_set_pack, after_set_theme);
        assert_ne!(context.theme_revision(), after_set_pack);
    }
}
