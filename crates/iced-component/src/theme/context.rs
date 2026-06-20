use super::pack::{ThemePack, with_theme_pack};

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

    /// Creates a context from the current thread-local theme.
    #[must_use]
    pub fn current() -> Self {
        with_theme_pack(Self::from_theme)
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

/// Reads a snapshot of the current thread-local theme context.
pub fn with_theme_context<R>(read: impl FnOnce(&ThemeContext) -> R) -> R {
    let context = ThemeContext::current();
    read(&context)
}

#[cfg(test)]
mod tests {
    use super::{ThemeContext, with_theme_context};
    use crate::{Color, ThemePack, set_theme_pack};

    #[test]
    fn current_context_reads_thread_local_theme() {
        let accent = Color::new(26, 95, 180);
        let mut theme = ThemePack::adwaita();
        theme.button.primary.bg = accent;

        set_theme_pack(theme);

        with_theme_context(|context| assert_eq!(context.theme().button.primary.bg, accent));
        set_theme_pack(ThemePack::adwaita());
    }

    #[test]
    fn scoped_context_does_not_mutate_parent() {
        let parent = ThemeContext::from_theme(&ThemePack::adwaita());
        let scoped_bg = Color::new(221, 238, 255);
        let scoped = parent.scoped(|theme| theme.button.standard.hover.bg = scoped_bg);

        assert_ne!(parent.theme().button.standard.hover.bg, scoped_bg);
        assert_eq!(scoped.theme().button.standard.hover.bg, scoped_bg);
    }
}
