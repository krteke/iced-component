use iced_component_core::{
    anim::MotionRuntime,
    component::{ComponentContext, ComponentUpdateCx, ComponentViewCx, StyleChange, StyleRevision},
};
use spectrum_theme::{Color, config::TomlThemeSource};

use crate::theme::{
    MATERIAL_DARK_TOML, MATERIAL_LIGHT_TOML, ThemeLoadError, ThemePack, ThemeSeedError,
};

/// Material color-scheme mode supplied by `spectrum-theme`.
pub use spectrum_theme::ThemeMode;

#[derive(Clone)]
struct ThemeSlot {
    pack: ThemePack,
    source: Option<TomlThemeSource>,
}

impl ThemeSlot {
    fn direct(pack: ThemePack) -> Self {
        Self { pack, source: None }
    }

    fn try_from_toml(mode: ThemeMode, input: &str) -> Result<Self, ThemeLoadError> {
        let source = TomlThemeSource::parse(input)?;
        if source.mode() != mode {
            return Err(ThemeLoadError::ModeMismatch {
                expected: mode,
                actual: source.mode(),
            });
        }
        let pack = ThemePack::try_from_toml_source(&source)?;

        Ok(Self {
            pack,
            source: Some(source),
        })
    }

    fn try_reseed(&self, mode: ThemeMode, seed: Color) -> Result<Self, ThemeSeedError> {
        let source = self
            .source
            .as_ref()
            .ok_or(ThemeSeedError::SourceDetached { mode })?
            .clone()
            .with_seed(seed);
        let pack = ThemePack::try_from_toml_source(&source)?;

        Ok(Self {
            pack,
            source: Some(source),
        })
    }

    fn detach(&mut self) {
        self.source = None;
    }

    const fn is_source_backed(&self) -> bool {
        self.source.is_some()
    }
}

/// Mutable pair of Material light and dark theme packs.
#[derive(Clone)]
pub struct Theme {
    mode: ThemeMode,
    light: ThemeSlot,
    dark: ThemeSlot,
}

impl Theme {
    /// Creates a source-backed theme with the embedded Material 3 packs.
    #[must_use]
    pub fn new(mode: ThemeMode) -> Self {
        Self::try_from_toml(mode, MATERIAL_LIGHT_TOML, MATERIAL_DARK_TOML)
            .expect("embedded Material theme sources are valid")
    }

    /// Creates a source-backed theme from light and dark TOML inputs.
    pub fn try_from_toml(mode: ThemeMode, light: &str, dark: &str) -> Result<Self, ThemeLoadError> {
        Ok(Self {
            mode,
            light: ThemeSlot::try_from_toml(ThemeMode::Light, light)?,
            dark: ThemeSlot::try_from_toml(ThemeMode::Dark, dark)?,
        })
    }

    /// Creates a theme from direct resolved packs.
    ///
    /// Direct packs deliberately do not retain TOML provenance and therefore
    /// cannot later be rebuilt with [`UpdateCx::set_seed`].
    #[must_use]
    pub fn from_packs(mode: ThemeMode, light: ThemePack, dark: ThemePack) -> Self {
        Self {
            mode,
            light: ThemeSlot::direct(light),
            dark: ThemeSlot::direct(dark),
        }
    }

    /// Creates a source-backed light theme.
    #[must_use]
    pub fn light() -> Self {
        Self::new(ThemeMode::Light)
    }

    /// Creates a source-backed dark theme.
    #[must_use]
    pub fn dark() -> Self {
        Self::new(ThemeMode::Dark)
    }

    /// Returns the currently active theme pack.
    #[must_use]
    pub const fn pack(&self) -> &ThemePack {
        self.pack_for(self.mode)
    }

    /// Returns the active theme mode.
    #[must_use]
    pub const fn mode(&self) -> ThemeMode {
        self.mode
    }

    /// Returns a pack for a specific mode.
    #[must_use]
    pub const fn pack_for(&self, mode: ThemeMode) -> &ThemePack {
        match mode {
            ThemeMode::Dark => &self.dark.pack,
            ThemeMode::Light => &self.light.pack,
        }
    }

    /// Returns whether a mode can be rebuilt from TOML with a new seed.
    #[must_use]
    pub const fn is_seed_backed(&self, mode: ThemeMode) -> bool {
        match mode {
            ThemeMode::Dark => self.dark.is_source_backed(),
            ThemeMode::Light => self.light.is_source_backed(),
        }
    }

    fn slot_for_mut(&mut self, mode: ThemeMode) -> &mut ThemeSlot {
        match mode {
            ThemeMode::Dark => &mut self.dark,
            ThemeMode::Light => &mut self.light,
        }
    }

    fn set_pack(&mut self, mode: ThemeMode, pack: ThemePack) {
        *self.slot_for_mut(mode) = ThemeSlot::direct(pack);
    }

    fn patch_pack(&mut self, mode: ThemeMode, patch: impl FnOnce(&mut ThemePack)) {
        let slot = self.slot_for_mut(mode);
        slot.detach();
        patch(&mut slot.pack);
    }

    fn try_set_pack_from_toml(
        &mut self,
        mode: ThemeMode,
        input: &str,
    ) -> Result<(), ThemeLoadError> {
        *self.slot_for_mut(mode) = ThemeSlot::try_from_toml(mode, input)?;
        Ok(())
    }

    fn try_set_seed(&mut self, seed: Color) -> Result<(), ThemeSeedError> {
        let light = self.light.try_reseed(ThemeMode::Light, seed)?;
        let dark = self.dark.try_reseed(ThemeMode::Dark, seed)?;

        self.light = light;
        self.dark = dark;
        Ok(())
    }

    fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            ThemeMode::Dark => ThemeMode::Light,
            ThemeMode::Light => ThemeMode::Dark,
        };
    }
}

/// Shared Material component inputs.
#[derive(Clone)]
pub struct Context {
    core: ComponentContext,
    theme: Theme,
}

impl Context {
    /// Creates a Material context with the embedded scheme for `mode`.
    #[must_use]
    pub fn new(mode: ThemeMode) -> Self {
        Self::from_theme(Theme::new(mode))
    }

    /// Creates a Material context from an explicit theme.
    #[must_use]
    pub fn from_theme(theme: Theme) -> Self {
        Self {
            core: ComponentContext::new(),
            theme,
        }
    }

    /// Creates a Material light context.
    #[must_use]
    pub fn light() -> Self {
        Self::new(ThemeMode::Light)
    }

    /// Creates a Material dark context.
    #[must_use]
    pub fn dark() -> Self {
        Self::new(ThemeMode::Dark)
    }

    /// Returns the shared component context.
    #[must_use]
    pub const fn core(&self) -> &ComponentContext {
        &self.core
    }

    /// Returns the current Material theme.
    #[must_use]
    pub const fn theme(&self) -> &Theme {
        &self.theme
    }

    /// Returns the revision used to invalidate stale style motion.
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

/// Mutable inputs used while applying Material component events.
pub struct UpdateCx<'a> {
    runtime: &'a mut MotionRuntime,
    context: &'a mut Context,
}

impl<'a> UpdateCx<'a> {
    /// Creates a Material update context.
    pub fn new(runtime: &'a mut MotionRuntime, context: &'a mut Context) -> Self {
        Self { runtime, context }
    }

    /// Creates a temporary core update context for shared motion helpers.
    pub fn core(&mut self) -> ComponentUpdateCx<'_> {
        ComponentUpdateCx::new(self.runtime, &mut self.context.core)
    }

    /// Returns the current Material context.
    #[must_use]
    pub const fn context(&self) -> &Context {
        self.context
    }

    /// Returns the current Material theme.
    #[must_use]
    pub const fn theme(&self) -> &Theme {
        &self.context.theme
    }

    /// Toggles between the configured light and dark packs.
    pub fn toggle_theme(&mut self) -> StyleChange {
        self.context.theme.toggle_mode();
        self.context.core.bump_style_revision()
    }

    /// Rebuilds both source-backed packs using one Material seed.
    pub fn set_seed(&mut self, seed: Color) -> Result<StyleChange, ThemeSeedError> {
        self.context.theme.try_set_seed(seed)?;
        Ok(self.context.core.bump_style_revision())
    }

    /// Replaces both configured theme packs.
    pub fn set_theme(&mut self, theme: Theme) -> StyleChange {
        self.context.theme = theme;
        self.context.core.bump_style_revision()
    }

    /// Mutates the active resolved pack and detaches its TOML source.
    pub fn patch_theme(&mut self, patch: impl FnOnce(&mut ThemePack)) -> StyleChange {
        let mode = self.context.theme.mode();
        self.context.theme.patch_pack(mode, patch);
        self.context.core.bump_style_revision()
    }

    /// Mutates one resolved pack and detaches its TOML source.
    pub fn patch_theme_for(
        &mut self,
        mode: ThemeMode,
        patch: impl FnOnce(&mut ThemePack),
    ) -> StyleChange {
        self.context.theme.patch_pack(mode, patch);
        self.context.core.bump_style_revision()
    }

    /// Replaces one resolved pack and detaches its TOML source.
    pub fn set_theme_pack(&mut self, mode: ThemeMode, pack: ThemePack) -> StyleChange {
        self.context.theme.set_pack(mode, pack);
        self.context.core.bump_style_revision()
    }

    /// Loads and installs one source-backed theme pack from TOML.
    pub fn set_theme_pack_from_toml(
        &mut self,
        mode: ThemeMode,
        input: &str,
    ) -> Result<StyleChange, ThemeLoadError> {
        self.context.theme.try_set_pack_from_toml(mode, input)?;
        Ok(self.context.core.bump_style_revision())
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

/// Read-only inputs used while rendering Material component views.
pub struct ViewCx<'a> {
    runtime: &'a MotionRuntime,
    context: &'a Context,
}

impl<'a> ViewCx<'a> {
    /// Creates a Material view context.
    #[must_use]
    pub const fn new(runtime: &'a MotionRuntime, context: &'a Context) -> Self {
        Self { runtime, context }
    }

    /// Creates a temporary core view context for shared motion helpers.
    #[must_use]
    pub const fn core(&self) -> ComponentViewCx<'_> {
        ComponentViewCx::new(self.runtime, self.context.core())
    }

    /// Returns the current Material context.
    #[must_use]
    pub const fn context(&self) -> &Context {
        self.context
    }

    /// Returns the current Material theme.
    #[must_use]
    pub const fn theme(&self) -> &Theme {
        self.context.theme()
    }

    /// Returns the active theme mode.
    #[must_use]
    pub const fn theme_mode(&self) -> ThemeMode {
        self.context.theme().mode()
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

    use crate::theme::{ThemeLoadError, ThemeSeedError};

    use super::{Context, ThemeMode, UpdateCx, ViewCx};

    #[test]
    fn reseeding_updates_both_source_backed_packs() {
        let mut runtime = MotionRuntime::new();
        let mut context = Context::light();
        let light_before = context
            .theme()
            .pack_for(ThemeMode::Light)
            .palette
            .primary
            .color;
        let dark_before = context
            .theme()
            .pack_for(ThemeMode::Dark)
            .palette
            .primary
            .color;

        UpdateCx::new(&mut runtime, &mut context)
            .set_seed(Color::new(0, 90, 220))
            .unwrap();

        assert_ne!(
            context
                .theme()
                .pack_for(ThemeMode::Light)
                .palette
                .primary
                .color,
            light_before
        );
        assert_ne!(
            context
                .theme()
                .pack_for(ThemeMode::Dark)
                .palette
                .primary
                .color,
            dark_before
        );
    }

    #[test]
    fn direct_patch_detaches_the_pack_before_reseeding() {
        let mut runtime = MotionRuntime::new();
        let mut context = Context::light();

        UpdateCx::new(&mut runtime, &mut context).patch_theme(|theme| {
            theme.palette.primary.color = Color::new(1, 2, 3);
        });

        let error = UpdateCx::new(&mut runtime, &mut context)
            .set_seed(Color::new(0, 90, 220))
            .unwrap_err();

        assert!(matches!(
            error,
            ThemeSeedError::SourceDetached {
                mode: ThemeMode::Light
            }
        ));
        assert_eq!(
            context.theme().pack().palette.primary.color,
            Color::new(1, 2, 3)
        );
    }

    #[test]
    fn installing_toml_restores_seed_support_for_a_direct_pack() {
        let mut runtime = MotionRuntime::new();
        let mut context = Context::light();
        let pack = context.theme().pack_for(ThemeMode::Light).clone();

        UpdateCx::new(&mut runtime, &mut context).set_theme_pack(ThemeMode::Light, pack);
        UpdateCx::new(&mut runtime, &mut context)
            .set_theme_pack_from_toml(ThemeMode::Light, crate::theme::MATERIAL_LIGHT_TOML)
            .unwrap();

        assert!(context.theme().is_seed_backed(ThemeMode::Light));
    }

    #[test]
    fn theme_rejects_toml_with_the_wrong_mode_for_a_slot() {
        let result = super::Theme::try_from_toml(
            ThemeMode::Light,
            crate::theme::MATERIAL_DARK_TOML,
            crate::theme::MATERIAL_DARK_TOML,
        );
        let Err(error) = result else {
            panic!("dark TOML must not be accepted for the light slot");
        };

        assert!(matches!(
            error,
            ThemeLoadError::ModeMismatch {
                expected: ThemeMode::Light,
                actual: ThemeMode::Dark,
            }
        ));
    }

    #[test]
    fn theme_changes_bump_the_style_revision() {
        let mut runtime = MotionRuntime::new();
        let mut context = Context::light();
        let revision = context.style_revision();

        UpdateCx::new(&mut runtime, &mut context).toggle_theme();

        assert_ne!(context.style_revision(), revision);
    }

    #[test]
    fn view_context_only_exposes_read_inputs() {
        let runtime = MotionRuntime::new();
        let context = Context::dark();
        let cx = ViewCx::new(&runtime, &context);

        assert_eq!(cx.theme_mode(), ThemeMode::Dark);
        assert_eq!(cx.style_revision(), context.style_revision());
    }
}
