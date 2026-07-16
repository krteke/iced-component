use aura_anim::prelude::Timing;
use iced_component_core::component::animation::AnimationOverrides;

/// Configurable animation timings for Material loading indicator theme colors.
#[derive(Clone, Copy, Debug)]
pub struct LoadingIndicatorAnimations {
    /// Timing used when synchronizing colors after a theme change.
    pub theme_change: Timing,
}

impl LoadingIndicatorAnimations {
    /// Creates a loading indicator animation configuration.
    #[must_use]
    pub const fn new(theme_change: Timing) -> Self {
        Self { theme_change }
    }

    pub(crate) fn resolve(overrides: &AnimationOverrides) -> Self {
        overrides.get::<Self>().copied().unwrap_or_default()
    }
}

impl Default for LoadingIndicatorAnimations {
    fn default() -> Self {
        Self::new(Timing::ease_out(200.0))
    }
}
