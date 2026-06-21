use crate::{
    motion::{MotionPreferences, MotionTokens},
    theme::{ThemeContext, ThemePack},
};

/// Shared read-only inputs used while updating or rendering components.
#[derive(Clone)]
pub struct ComponentContext {
    theme: ThemeContext,
    motion_tokens: MotionTokens,
    motion_preferences: MotionPreferences,
}

impl ComponentContext {
    /// Creates a component context from explicit inputs.
    #[must_use]
    pub const fn new(
        theme: ThemeContext,
        motion_tokens: MotionTokens,
        motion_preferences: MotionPreferences,
    ) -> Self {
        Self {
            theme,
            motion_tokens,
            motion_preferences,
        }
    }

    /// Creates a context from the current theme and default motion settings.
    #[must_use]
    pub fn current() -> Self {
        Self::new(
            ThemeContext::current(),
            MotionTokens::default(),
            MotionPreferences::default(),
        )
    }

    /// Returns the theme context.
    #[must_use]
    pub const fn theme(&self) -> &ThemeContext {
        &self.theme
    }

    /// Returns motion duration tokens.
    #[must_use]
    pub const fn motion_tokens(&self) -> MotionTokens {
        self.motion_tokens
    }

    /// Returns shared motion preferences.
    #[must_use]
    pub const fn motion_preferences(&self) -> &MotionPreferences {
        &self.motion_preferences
    }

    /// Returns a context with local theme token changes applied.
    #[must_use]
    pub fn scoped_theme(&self, patch: impl FnOnce(&mut ThemePack)) -> Self {
        Self {
            theme: self.theme.scoped(patch),
            motion_tokens: self.motion_tokens,
            motion_preferences: self.motion_preferences.clone(),
        }
    }

    /// Returns a context with different motion tokens.
    #[must_use]
    pub const fn with_motion_tokens(mut self, motion_tokens: MotionTokens) -> Self {
        self.motion_tokens = motion_tokens;
        self
    }

    /// Returns a context with different motion preferences.
    #[must_use]
    pub fn with_motion_preferences(mut self, motion_preferences: MotionPreferences) -> Self {
        self.motion_preferences = motion_preferences;
        self
    }
}

impl Default for ComponentContext {
    fn default() -> Self {
        Self::current()
    }
}

#[cfg(test)]
mod tests {
    use aura_anim_core::timing::Duration;
    use spectrum_theme::Color;

    use crate::{
        component::ComponentContext,
        motion::{MotionPreferences, MotionSpeed, MotionTokens},
    };

    #[test]
    fn scoped_theme_keeps_motion_inputs() {
        let context = ComponentContext::current();
        let scoped_bg = Color::new(221, 238, 255);
        let scoped = context.scoped_theme(|theme| theme.button.standard.hover.bg = scoped_bg);

        assert_eq!(scoped.theme().theme().button.standard.hover.bg, scoped_bg);
        assert_eq!(scoped.motion_tokens(), context.motion_tokens());
        assert!(
            scoped
                .motion_preferences()
                .is_shared_with(context.motion_preferences())
        );
    }

    #[test]
    fn context_can_override_motion_inputs() {
        let (preferences, _controller) = MotionPreferences::new(true);
        let tokens = MotionTokens {
            fast: Duration::from_millis(40.0),
            normal: Duration::from_millis(80.0),
            slow: Duration::from_millis(120.0),
        };

        let context = ComponentContext::current()
            .with_motion_tokens(tokens)
            .with_motion_preferences(preferences);

        assert_eq!(
            context.motion_tokens().duration(MotionSpeed::Fast),
            Duration::from_millis(40.0)
        );
        assert!(context.motion_preferences().reduce_motion());
    }
}
