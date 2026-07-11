//! Generic adapter for theme-native button implementations.

#[cfg(all(feature = "adwaita", feature = "material"))]
mod defaults;
#[cfg(all(feature = "adwaita", feature = "material"))]
mod style;
#[cfg(all(test, feature = "adwaita", feature = "material"))]
mod tests;

use iced::Element;
use iced_component_core::anim::MotionError;

use crate::{
    backend::{ButtonBackend, ButtonViewBackend},
    context::{AdapterUpdateCx, AdapterViewCx, BackendSelection},
};

pub use iced_component_core::component::button::{ButtonEvent, ButtonOutcome, ButtonSignal};
#[cfg(all(feature = "adwaita", feature = "material"))]
pub use style::ButtonStyle;

/// Stateful button adapter backed by exactly two selected theme libraries.
#[derive(Debug)]
pub struct AdaptiveButton<A, B>
where
    A: ButtonBackend,
    B: ButtonBackend,
{
    first: A::Button,
    second: B::Button,
}

impl<A, B> AdaptiveButton<A, B>
where
    A: ButtonBackend,
    B: ButtonBackend,
{
    /// Creates an adapter from fully configured concrete buttons.
    #[must_use]
    pub const fn from_backends(first: A::Button, second: B::Button) -> Self {
        Self { first, second }
    }

    /// Returns this button with a disabled initial state in both backends.
    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.first = A::disabled(self.first, disabled);
        self.second = B::disabled(self.second, disabled);
        self
    }

    /// Registers both selected backends for direct runtime switching.
    pub fn register(&mut self, cx: &mut AdapterUpdateCx<'_, A, B>) {
        A::register(&mut self.first, &mut cx.first());
        B::register(&mut self.second, &mut cx.second());
    }

    /// Synchronizes both concrete buttons with their retained contexts.
    pub fn sync(&mut self, cx: &mut AdapterUpdateCx<'_, A, B>) -> Result<bool, MotionError> {
        let first = A::sync(&mut self.first, &mut cx.first())?;
        let second = B::sync(&mut self.second, &mut cx.second())?;
        Ok(first || second)
    }

    /// Enables or disables both concrete buttons.
    pub fn set_disabled(
        &mut self,
        disabled: bool,
        cx: &mut AdapterUpdateCx<'_, A, B>,
    ) -> Result<bool, MotionError> {
        let first = A::set_disabled(&mut self.first, disabled, &mut cx.first())?;
        let second = B::set_disabled(&mut self.second, disabled, &mut cx.second())?;
        Ok(first || second)
    }

    /// Applies one rendered event to both backends and returns the active result.
    pub fn update_event(
        &mut self,
        event: ButtonEvent,
        cx: &mut AdapterUpdateCx<'_, A, B>,
    ) -> Result<ButtonOutcome, MotionError> {
        let selection = cx.selection();
        let first = A::update_event(&mut self.first, event, &mut cx.first())?;
        let second = B::update_event(&mut self.second, event, &mut cx.second())?;

        Ok(match selection {
            BackendSelection::First => first,
            BackendSelection::Second => second,
        })
    }

    /// Builds a view using the currently selected backend.
    #[must_use]
    pub fn view<'a, Message>(
        &'a self,
        cx: &AdapterViewCx<'_, A, B>,
    ) -> AdaptiveButtonView<'a, Message, A, B>
    where
        Message: Clone + 'a,
    {
        let backend = match cx.selection() {
            BackendSelection::First => {
                AdaptiveButtonViewBackend::First(A::view(&self.first, &cx.first()))
            }
            BackendSelection::Second => {
                AdaptiveButtonViewBackend::Second(B::view(&self.second, &cx.second()))
            }
        };
        AdaptiveButtonView { backend }
    }

    /// Returns the first concrete themed button.
    #[must_use]
    pub const fn first(&self) -> &A::Button {
        &self.first
    }

    /// Returns the mutable first concrete themed button.
    pub fn first_mut(&mut self) -> &mut A::Button {
        &mut self.first
    }

    /// Returns the second concrete themed button.
    #[must_use]
    pub const fn second(&self) -> &B::Button {
        &self.second
    }

    /// Returns the mutable second concrete themed button.
    pub fn second_mut(&mut self) -> &mut B::Button {
        &mut self.second
    }

    /// Returns whether both selected backend slots are registered.
    #[must_use]
    pub fn is_registered(&self) -> bool {
        A::is_registered(&self.first) && B::is_registered(&self.second)
    }
}

/// Temporary view adapter for a generic [`AdaptiveButton`].
pub struct AdaptiveButtonView<'a, Message, A, B>
where
    A: ButtonBackend + 'a,
    B: ButtonBackend + 'a,
    Message: Clone + 'a,
{
    backend: AdaptiveButtonViewBackend<A::View<'a, Message>, B::View<'a, Message>>,
}

enum AdaptiveButtonViewBackend<A, B> {
    First(A),
    Second(B),
}

impl<'a, Message, A, B> AdaptiveButtonView<'a, Message, A, B>
where
    A: ButtonBackend + 'a,
    B: ButtonBackend + 'a,
    Message: Clone + 'a,
{
    /// Replaces the content rendered by the selected backend.
    #[must_use]
    pub fn content(self, content: impl Into<Element<'a, Message>>) -> Self {
        let content = content.into();
        let backend = match self.backend {
            AdaptiveButtonViewBackend::First(button) => {
                AdaptiveButtonViewBackend::First(button.content(content))
            }
            AdaptiveButtonViewBackend::Second(button) => {
                AdaptiveButtonViewBackend::Second(button.content(content))
            }
        };
        Self { backend }
    }

    /// Maps rendered button events into application messages.
    #[must_use]
    pub fn on_event<F>(self, mapper: F) -> Self
    where
        F: Fn(ButtonEvent) -> Message + 'a,
    {
        let backend = match self.backend {
            AdaptiveButtonViewBackend::First(button) => {
                AdaptiveButtonViewBackend::First(button.on_event(mapper))
            }
            AdaptiveButtonViewBackend::Second(button) => {
                AdaptiveButtonViewBackend::Second(button.on_event(mapper))
            }
        };
        Self { backend }
    }
}

impl<'a, Message, A, B> From<AdaptiveButtonView<'a, Message, A, B>> for Element<'a, Message>
where
    A: ButtonBackend + 'a,
    B: ButtonBackend + 'a,
    Message: Clone + 'a,
{
    fn from(view: AdaptiveButtonView<'a, Message, A, B>) -> Self {
        match view.backend {
            AdaptiveButtonViewBackend::First(button) => button.into(),
            AdaptiveButtonViewBackend::Second(button) => button.into(),
        }
    }
}

#[cfg(all(feature = "adwaita", feature = "material"))]
pub use defaults::{Button, ButtonView};
