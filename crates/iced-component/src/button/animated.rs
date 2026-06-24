use crate::button::{ButtonResolvedStyle, ButtonStyleState, ButtonVariant, motion::ButtonMotion};

/// Read-only button state consumed by rendering code.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ButtonSnapshot {
    /// Button visual variant.
    pub variant: ButtonVariant,
    /// Interaction state used to resolve component style.
    pub style_state: ButtonStyleState,
    /// Resolved theme style for the current interaction state.
    pub style: ButtonResolvedStyle,
    /// Current animated motion values.
    pub motion: ButtonMotion,
    /// Whether focus visuals are active.
    pub focused: bool,
    /// Whether the button is disabled.
    pub disabled: bool,
}

/// Button interaction message handled by [`AnimatedButton`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ButtonInteraction {
    /// Pointer entered the button.
    HoverEnter,
    /// Pointer left the button.
    HoverExit,
    /// Pointer pressed the button.
    PressDown,
    /// Pointer released the button.
    PressUp,
    /// Keyboard focus entered the button.
    Focus,
    /// Keyboard focus left the button.
    Blur,
    /// Enables or disables the button.
    SetDisabled(bool),
}

/// Button event that can carry an application action on release.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ButtonEvent<Action> {
    /// Internal interaction that only updates button state.
    Interaction(ButtonInteraction),
    /// Release event that first resets pressed state, then yields an action.
    Pressed(Action),
}
