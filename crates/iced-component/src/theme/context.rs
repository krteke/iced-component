use super::pack::ThemePack;

/// Theme snapshot used while resolving component styles.
#[derive(Clone)]
pub struct ThemeContext {
    theme: ThemePack,
}

impl ThemeContext {
    /// Creates a context from an owned theme pack.
    #[must_use]
    pub const fn new(theme: ThemePack) -> Self {
        Self { theme }
    }

    /// Creates a context by cloning a theme pack.
    #[must_use]
    pub fn from_theme(theme: &ThemePack) -> Self {
        Self {
            theme: theme.clone(),
        }
    }

    /// Creates a context from the default Adwaita-like theme.
    #[must_use]
    pub fn adwaita() -> Self {
        Self::new(ThemePack::adwaita())
    }

    /// Returns the resolved theme snapshot.
    #[must_use]
    pub const fn theme(&self) -> &ThemePack {
        &self.theme
    }

    /// Returns a scoped context with local token changes applied.
    #[must_use]
    pub fn scoped(&self, patch: impl FnOnce(&mut ThemePack)) -> Self {
        let mut theme = self.theme.clone();
        patch(&mut theme);

        Self { theme }
    }

    /// Applies local token changes to this context.
    #[must_use]
    pub fn with_patch(mut self, patch: impl FnOnce(&mut ThemePack)) -> Self {
        patch(&mut self.theme);
        self
    }

    /// Consumes the context and returns its theme pack.
    #[must_use]
    pub fn into_theme(self) -> ThemePack {
        self.theme
    }
}

#[cfg(test)]
mod tests {
    use spectrum_theme::Color;

    use crate::theme::ThemePack;

    use super::ThemeContext;

    #[test]
    fn scoped_context_does_not_mutate_parent() {
        let parent = ThemeContext::from_theme(&ThemePack::adwaita());
        let scoped_bg = Color::new(221, 238, 255);
        let scoped = parent.scoped(|theme| theme.button.standard_filled.hover.bg = scoped_bg);

        assert_ne!(parent.theme().button.standard_filled.hover.bg, scoped_bg);
        assert_eq!(scoped.theme().button.standard_filled.hover.bg, scoped_bg);
    }

    #[test]
    fn adwaita_context_uses_default_theme() {
        let context = ThemeContext::adwaita();

        assert_eq!(context.theme().app.bg, ThemePack::adwaita().app.bg);
    }
}
