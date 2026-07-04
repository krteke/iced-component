//! Animation provider presets and runtime animation context.

use std::{fmt, sync::Arc};

use aura_anim::core::traits::BoxAnimation;

use crate::{
    button::{ButtonAnimationProvider, ButtonMotion, ButtonMotionTransition},
    surface::{SurfaceAnimationProvider, SurfaceMotion, SurfaceMotionTransition},
};

pub mod adwaita;

/// Canonical built-in Adwaita-like animation providers.
pub use adwaita::{AdwaitaButtonAnimationProvider, AdwaitaSurfaceAnimationProvider};

/// Shared animation providers used by component update paths.
///
/// Components should read providers from this context instead of storing their
/// own provider slots. This keeps one animation theme attached to the
/// [`crate::component::ComponentContext`] used for the current update.
#[derive(Clone)]
pub struct AnimationContext {
    button: ButtonAnimationProviderSlot,
    surface: SurfaceAnimationProviderSlot,
}

impl AnimationContext {
    /// Creates an animation context from explicit component providers.
    #[must_use]
    pub const fn new(
        button: ButtonAnimationProviderSlot,
        surface: SurfaceAnimationProviderSlot,
    ) -> Self {
        Self { button, surface }
    }

    /// Creates the default Adwaita-like animation context.
    #[must_use]
    pub fn adwaita() -> Self {
        Self::new(
            ButtonAnimationProviderSlot::new(AdwaitaButtonAnimationProvider),
            SurfaceAnimationProviderSlot::new(AdwaitaSurfaceAnimationProvider),
        )
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

    /// Returns the surface animation provider slot.
    #[must_use]
    pub const fn surface(&self) -> &SurfaceAnimationProviderSlot {
        &self.surface
    }

    /// Returns the mutable surface animation provider slot.
    pub fn surface_mut(&mut self) -> &mut SurfaceAnimationProviderSlot {
        &mut self.surface
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

    /// Returns this context with a different surface animation provider.
    #[must_use]
    pub fn with_surface_provider(mut self, provider: impl SurfaceAnimationProvider) -> Self {
        self.set_surface_provider(provider);
        self
    }

    /// Replaces the surface animation provider.
    pub fn set_surface_provider(&mut self, provider: impl SurfaceAnimationProvider) {
        self.surface = SurfaceAnimationProviderSlot::new(provider);
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

/// Shared slot for one surface animation provider.
#[derive(Clone)]
pub struct SurfaceAnimationProviderSlot {
    provider: Arc<dyn SurfaceAnimationProvider>,
}

impl SurfaceAnimationProviderSlot {
    /// Creates a provider slot from a concrete provider.
    #[must_use]
    pub fn new(provider: impl SurfaceAnimationProvider) -> Self {
        Self {
            provider: Arc::new(provider),
        }
    }

    /// Returns the stored surface animation provider.
    #[must_use]
    pub fn provider(&self) -> &dyn SurfaceAnimationProvider {
        self.provider.as_ref()
    }

    /// Builds one surface animation for the resolved transition.
    #[must_use]
    pub fn build(&self, transition: &SurfaceMotionTransition) -> BoxAnimation<SurfaceMotion> {
        (self.provider().surface_animation(transition))(*transition)
    }
}

impl fmt::Debug for SurfaceAnimationProviderSlot {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SurfaceAnimationProviderSlot { .. }")
    }
}
