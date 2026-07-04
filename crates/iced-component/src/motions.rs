//! Animation provider presets and runtime animation context.

use std::{fmt, sync::Arc};

use aura_anim::core::traits::BoxAnimation;

use crate::{
    button::{ButtonAnimationProvider, ButtonMotion, ButtonMotionTransition},
    surface::{SurfaceAnimationProvider, SurfaceMotion, SurfaceMotionTransition},
};

pub mod adwaita;

/// Canonical built-in Adwaita-like animation providers.
pub use adwaita::{
    AdwaitaButtonAnimationProvider, AdwaitaMotionProviders, AdwaitaSurfaceAnimationProvider,
};

/// A complete set of component animation providers.
pub trait MotionProviderSet {
    /// Converts this provider set into concrete animation providers.
    fn into_animation_providers(self) -> AnimationProviders;
}

/// Shared animation providers used by component update paths.
///
/// Components should read providers from this context instead of storing their
/// own provider slots. This keeps one animation theme attached to the
/// [`crate::component::ComponentContext`] used for the current update.
#[derive(Clone)]
pub struct AnimationContext {
    providers: AnimationProviders,
}

impl AnimationContext {
    /// Creates an animation context from an explicit provider group.
    #[must_use]
    pub const fn new(providers: AnimationProviders) -> Self {
        Self { providers }
    }

    /// Creates an animation context from a named provider set.
    #[must_use]
    pub fn from_provider_set(provider_set: impl MotionProviderSet) -> Self {
        Self::new(provider_set.into_animation_providers())
    }

    /// Creates the default Adwaita-like animation context.
    #[must_use]
    pub fn adwaita() -> Self {
        Self::from_provider_set(AdwaitaMotionProviders)
    }

    /// Returns the complete provider group.
    #[must_use]
    pub const fn providers(&self) -> &AnimationProviders {
        &self.providers
    }

    /// Returns the mutable complete provider group.
    pub fn providers_mut(&mut self) -> &mut AnimationProviders {
        &mut self.providers
    }

    /// Returns the button animation provider slot.
    #[must_use]
    pub const fn button(&self) -> &ButtonAnimationProviderSlot {
        self.providers.button()
    }

    /// Returns the mutable button animation provider slot.
    pub fn button_mut(&mut self) -> &mut ButtonAnimationProviderSlot {
        self.providers.button_mut()
    }

    /// Returns the surface animation provider slot.
    #[must_use]
    pub const fn surface(&self) -> &SurfaceAnimationProviderSlot {
        self.providers.surface()
    }

    /// Returns the mutable surface animation provider slot.
    pub fn surface_mut(&mut self) -> &mut SurfaceAnimationProviderSlot {
        self.providers.surface_mut()
    }

    /// Returns this context with a different button animation provider.
    #[must_use]
    pub fn with_button_provider(mut self, provider: impl ButtonAnimationProvider) -> Self {
        self.set_button_provider(provider);
        self
    }

    /// Replaces the button animation provider.
    pub fn set_button_provider(&mut self, provider: impl ButtonAnimationProvider) {
        self.providers.set_button_provider(provider);
    }

    /// Returns this context with a different surface animation provider.
    #[must_use]
    pub fn with_surface_provider(mut self, provider: impl SurfaceAnimationProvider) -> Self {
        self.set_surface_provider(provider);
        self
    }

    /// Replaces the surface animation provider.
    pub fn set_surface_provider(&mut self, provider: impl SurfaceAnimationProvider) {
        self.providers.set_surface_provider(provider);
    }
}

/// Concrete component animation provider slots.
#[derive(Clone)]
pub struct AnimationProviders {
    button: ButtonAnimationProviderSlot,
    surface: SurfaceAnimationProviderSlot,
}

impl AnimationProviders {
    /// Creates an animation provider group from explicit component slots.
    #[must_use]
    pub const fn new(
        button: ButtonAnimationProviderSlot,
        surface: SurfaceAnimationProviderSlot,
    ) -> Self {
        Self { button, surface }
    }

    /// Creates the default Adwaita-like provider group.
    #[must_use]
    pub fn adwaita() -> Self {
        AdwaitaMotionProviders.into_animation_providers()
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

    /// Returns this provider group with a different button animation provider.
    #[must_use]
    pub fn with_button_provider(mut self, provider: impl ButtonAnimationProvider) -> Self {
        self.set_button_provider(provider);
        self
    }

    /// Replaces the button animation provider.
    pub fn set_button_provider(&mut self, provider: impl ButtonAnimationProvider) {
        self.button = ButtonAnimationProviderSlot::new(provider);
    }

    /// Returns this provider group with a different surface animation provider.
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

impl MotionProviderSet for AnimationProviders {
    fn into_animation_providers(self) -> AnimationProviders {
        self
    }
}

impl Default for AnimationProviders {
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

#[cfg(test)]
mod tests {
    use crate::{
        button::{ButtonMotion, ButtonMotionTransition, ButtonMotionTrigger},
        motions::{AdwaitaMotionProviders, AnimationContext, AnimationProviders},
        surface::{SurfaceMotion, SurfaceMotionTransition, SurfaceMotionTrigger},
        theme::ThemePack,
    };

    #[test]
    fn adwaita_motion_providers_populate_animation_context() {
        let context = AnimationContext::from_provider_set(AdwaitaMotionProviders);
        let theme = ThemePack::adwaita();

        let button_motion = ButtonMotion {
            tokens: theme.button.standard_filled.idle,
            focus_ring_alpha: 0.0,
            focus_ring_width: 0.0,
        };
        let surface_motion = SurfaceMotion {
            tokens: theme.surface.raised.idle,
            elevation: 1.0,
        };

        let _button_animation = context.button().build(&ButtonMotionTransition {
            from: button_motion,
            to: button_motion,
            trigger: ButtonMotionTrigger::HoverEnter,
        });
        let _surface_animation = context.surface().build(&SurfaceMotionTransition {
            from: surface_motion,
            to: surface_motion,
            trigger: SurfaceMotionTrigger::HoverEnter,
        });
    }

    #[test]
    fn animation_providers_can_be_used_as_a_provider_set() {
        let context = AnimationContext::from_provider_set(AnimationProviders::adwaita());
        let theme = ThemePack::adwaita();
        let surface_motion = SurfaceMotion {
            tokens: theme.surface.regular.idle,
            elevation: 0.0,
        };

        let _surface_animation = context.surface().build(&SurfaceMotionTransition {
            from: surface_motion,
            to: surface_motion,
            trigger: SurfaceMotionTrigger::Sync,
        });
    }
}
