//! Generic adapter for theme-native loading indicators.

#[cfg(all(feature = "adwaita", feature = "material"))]
mod defaults;
#[cfg(all(test, feature = "adwaita", feature = "material"))]
mod tests;

use iced::{Element, time::Instant};
use iced_component_core::anim::MotionError;

use crate::{
    backend::loading_indicator::LoadingIndicatorBackend,
    context::{AdapterUpdateCx, AdapterViewCx, BackendSelection},
};

/// Loading indicator backed by exactly two selected theme libraries.
#[derive(Debug)]
pub struct AdaptiveLoadingIndicator<A, B>
where
    A: LoadingIndicatorBackend,
    B: LoadingIndicatorBackend,
{
    first: A::LoadingIndicator,
    second: B::LoadingIndicator,
}

impl<A, B> AdaptiveLoadingIndicator<A, B>
where
    A: LoadingIndicatorBackend,
    B: LoadingIndicatorBackend,
{
    /// Creates an adapter from fully configured concrete loading indicators.
    #[must_use]
    pub const fn from_backends(first: A::LoadingIndicator, second: B::LoadingIndicator) -> Self {
        Self { first, second }
    }

    /// Returns this adapter with one common square size.
    #[must_use]
    pub fn size(mut self, size: f32) -> Self {
        self.first = A::size(self.first, size);
        self.second = B::size(self.second, size);
        self
    }

    /// Sets one common square size on both concrete components.
    pub fn set_size(&mut self, size: f32) {
        A::set_size(&mut self.first, size);
        B::set_size(&mut self.second, size);
    }

    /// Clears the common square size on both concrete components.
    pub fn clear_size(&mut self) {
        A::clear_size(&mut self.first);
        B::clear_size(&mut self.second);
    }

    /// Returns the shared explicit size when both backends agree.
    #[must_use]
    pub fn explicit_size(&self) -> Option<f32> {
        let first = A::explicit_size(&self.first);
        if first == B::explicit_size(&self.second) {
            first
        } else {
            None
        }
    }

    /// Explicitly registers component-owned motion for both backends.
    pub fn register(&mut self, cx: &mut AdapterUpdateCx<'_, A, B>) {
        A::register(&mut self.first, &mut cx.first());
        B::register(&mut self.second, &mut cx.second());
    }

    /// Synchronizes both backends with their current theme revisions.
    pub fn sync(&mut self, cx: &mut AdapterUpdateCx<'_, A, B>) -> Result<bool, MotionError> {
        let first = A::sync(&mut self.first, &mut cx.first())?;
        let second = B::sync(&mut self.second, &mut cx.second())?;
        Ok(first || second)
    }

    /// Returns whether all required backend motion has been registered.
    #[must_use]
    pub fn is_registered(&self) -> bool {
        A::is_registered(&self.first) && B::is_registered(&self.second)
    }

    /// Advances both backend timelines to preserve direct switching.
    pub fn advance(&mut self, now: Instant) {
        A::advance(&mut self.first, now);
        B::advance(&mut self.second, now);
    }

    /// Restarts both backend timelines.
    pub fn reset(&mut self) {
        A::reset(&mut self.first);
        B::reset(&mut self.second);
    }

    /// Builds the currently selected theme-native loading indicator.
    #[must_use]
    pub fn view<Message>(&self, cx: &AdapterViewCx<'_, A, B>) -> Element<'static, Message>
    where
        Message: 'static,
    {
        match cx.selection() {
            BackendSelection::First => A::view(&self.first, &cx.first()),
            BackendSelection::Second => B::view(&self.second, &cx.second()),
        }
    }

    /// Returns the first concrete loading indicator.
    #[must_use]
    pub const fn first(&self) -> &A::LoadingIndicator {
        &self.first
    }

    /// Returns the mutable first concrete loading indicator.
    pub fn first_mut(&mut self) -> &mut A::LoadingIndicator {
        &mut self.first
    }

    /// Returns the second concrete loading indicator.
    #[must_use]
    pub const fn second(&self) -> &B::LoadingIndicator {
        &self.second
    }

    /// Returns the mutable second concrete loading indicator.
    pub fn second_mut(&mut self) -> &mut B::LoadingIndicator {
        &mut self.second
    }
}

#[cfg(all(feature = "adwaita", feature = "material"))]
pub use defaults::LoadingIndicator;
