//! Animation provider presets and runtime animation context.

use std::{fmt, sync::Arc};

use aura_anim::core::traits::BoxAnimation;

use crate::button::{ButtonAnimationProvider, ButtonMotion, ButtonMotionTransition};

pub mod adwaita;

/// Canonical built-in Adwaita-like button animation provider.
pub use adwaita::AdwaitaButtonAnimationProvider;

/// Shared animation providers used by component update paths.
///
/// Components should read providers from this context instead of storing their
/// own provider slots. This keeps one animation theme attached to the
/// [`crate::component::ComponentContext`] used for the current update.
#[derive(Clone)]
pub struct AnimationContext {
    button: ButtonAnimationProviderSlot,
}

impl AnimationContext {
    /// Creates an animation context from explicit component providers.
    #[must_use]
    pub const fn new(button: ButtonAnimationProviderSlot) -> Self {
        Self { button }
    }

    /// Creates the default Adwaita-like animation context.
    #[must_use]
    pub fn adwaita() -> Self {
        Self::new(ButtonAnimationProviderSlot::new(
            AdwaitaButtonAnimationProvider,
        ))
    }

    /// Returns the button animation provider slot.
    #[must_use]
    pub const fn button(&self) -> &ButtonAnimationProviderSlot {
        &self.button
    }

    /// Returns the mutable button animation provider slot.
    pub fn button_mut(&mut self) -> &mut ButtonAnimationProviderSlot {
        &mut self.button
    }

    /// Returns this context with a different button animation provider.
    #[must_use]
    pub fn with_button_provider(mut self, provider: impl ButtonAnimationProvider) -> Self {
        self.set_button_provider(provider);
        self
    }

    /// Replaces the button animation provider.
    pub fn set_button_provider(&mut self, provider: impl ButtonAnimationProvider) {
        self.button = ButtonAnimationProviderSlot::new(provider);
    }
}

impl Default for AnimationContext {
    fn default() -> Self {
        Self::adwaita()
    }
}

/// Shared slot for one button animation provider.
#[derive(Clone)]
pub struct ButtonAnimationProviderSlot {
    provider: Arc<dyn ButtonAnimationProvider>,
}

impl ButtonAnimationProviderSlot {
    /// Creates a provider slot from a concrete provider.
    #[must_use]
    pub fn new(provider: impl ButtonAnimationProvider) -> Self {
        Self {
            provider: Arc::new(provider),
        }
    }

    /// Returns the stored button animation provider.
    #[must_use]
    pub fn provider(&self) -> &dyn ButtonAnimationProvider {
        self.provider.as_ref()
    }

    /// Builds one button animation for the resolved transition.
    #[must_use]
    pub fn build(&self, transition: &ButtonMotionTransition) -> BoxAnimation<ButtonMotion> {
        (self.provider().button_animation(transition))(*transition)
    }
}

impl fmt::Debug for ButtonAnimationProviderSlot {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ButtonAnimationProviderSlot { .. }")
    }
}
