//! Theme-independent button interaction support.

use super::StyleChange;

/// Why a button visual target is being synchronized.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ButtonSync {
    /// Explicit application-requested synchronization.
    Manual,
    /// Synchronization after a context style revision change.
    StyleChanged(StyleChange),
}

/// Logical pointer coordinates relative to a component's bounds.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PointerPosition {
    /// Horizontal offset in logical pixels.
    pub x: f32,
    /// Vertical offset in logical pixels.
    pub y: f32,
}

impl PointerPosition {
    /// Creates a logical pointer position.
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// Theme-independent button interaction input.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ButtonSignal {
    /// Pointer entered the button.
    HoverEnter,
    /// Pointer left the button.
    HoverExit,
    /// Pointer pressed the button.
    PressDown,
    /// Pointer pressed the button at a logical position relative to its bounds.
    PressDownAt(PointerPosition),
    /// Pointer released the button.
    PressUp,
    /// Keyboard focus entered the button.
    Focus,
    /// Keyboard focus left the button.
    Blur,
    /// Button enabled or disabled state changed.
    SetDisabled(bool),
    /// Synchronizes the visual target without changing interaction state.
    Sync(ButtonSync),
}

/// Button interaction input emitted by a rendered button.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ButtonEvent {
    /// A state or style synchronization signal.
    Signal(ButtonSignal),
    /// A pointer press completed inside the button.
    Pressed,
}

/// Result of applying a rendered button event.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ButtonOutcome {
    /// The event did not activate the button.
    #[default]
    None,
    /// The button accepted a completed press.
    Activated,
}

impl ButtonOutcome {
    /// Returns whether the event activated the button.
    #[must_use]
    pub const fn is_activated(self) -> bool {
        matches!(self, Self::Activated)
    }
}

/// Theme-independent persistent button interaction state.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ButtonInteractionState {
    pointer: PointerState,
    pressed: bool,
    focused: bool,
    disabled: bool,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum PointerState {
    #[default]
    Outside,
    Hovered,
}

impl ButtonInteractionState {
    /// Creates an idle, enabled button interaction state.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            pointer: PointerState::Outside,
            pressed: false,
            focused: false,
            disabled: false,
        }
    }

    /// Applies an interaction signal.
    pub fn apply(&mut self, signal: ButtonSignal) {
        match signal {
            ButtonSignal::HoverEnter => self.pointer = PointerState::Hovered,
            ButtonSignal::HoverExit => {
                self.pointer = PointerState::Outside;
                self.pressed = false;
            }
            ButtonSignal::PressDown | ButtonSignal::PressDownAt(_) if !self.disabled => {
                self.pressed = true;
            }
            ButtonSignal::PressDown | ButtonSignal::PressDownAt(_) | ButtonSignal::Sync(_) => {}
            ButtonSignal::PressUp => self.pressed = false,
            ButtonSignal::Focus => self.focused = true,
            ButtonSignal::Blur => self.focused = false,
            ButtonSignal::SetDisabled(disabled) => self.set_disabled(disabled),
        }
    }

    /// Applies a rendered event and reports a completed enabled activation.
    pub fn apply_event(&mut self, event: ButtonEvent) -> ButtonOutcome {
        match event {
            ButtonEvent::Signal(signal) => {
                self.apply(signal);
                ButtonOutcome::None
            }
            ButtonEvent::Pressed => {
                self.pressed = false;
                if self.disabled {
                    ButtonOutcome::None
                } else {
                    ButtonOutcome::Activated
                }
            }
        }
    }

    /// Returns whether the pointer is hovering this button.
    #[must_use]
    pub const fn is_hovered(self) -> bool {
        matches!(self.pointer, PointerState::Hovered)
    }

    /// Returns whether this button is currently pressed.
    #[must_use]
    pub const fn is_pressed(self) -> bool {
        self.pressed
    }

    /// Returns whether this button has keyboard focus.
    #[must_use]
    pub const fn is_focused(self) -> bool {
        self.focused
    }

    /// Returns whether this button is disabled.
    #[must_use]
    pub const fn is_disabled(self) -> bool {
        self.disabled
    }

    fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
        if disabled {
            self.pointer = PointerState::Outside;
            self.pressed = false;
            self.focused = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ButtonEvent, ButtonInteractionState, ButtonOutcome, ButtonSignal, PointerPosition,
    };

    #[test]
    fn disabling_clears_transient_interaction_state() {
        let mut state = ButtonInteractionState::new();

        state.apply(ButtonSignal::HoverEnter);
        state.apply(ButtonSignal::PressDown);
        state.apply(ButtonSignal::Focus);
        state.apply(ButtonSignal::SetDisabled(true));

        assert!(state.is_disabled());
        assert!(!state.is_hovered());
        assert!(!state.is_pressed());
        assert!(!state.is_focused());
    }

    #[test]
    fn disabled_button_does_not_activate() {
        let mut state = ButtonInteractionState::new();

        state.apply(ButtonSignal::SetDisabled(true));

        assert_eq!(state.apply_event(ButtonEvent::Pressed), ButtonOutcome::None);
    }

    #[test]
    fn completed_press_releases_the_pressed_state() {
        let mut state = ButtonInteractionState::new();

        state.apply(ButtonSignal::PressDown);

        assert_eq!(
            state.apply_event(ButtonEvent::Pressed),
            ButtonOutcome::Activated
        );
        assert!(!state.is_pressed());
    }

    #[test]
    fn positioned_press_sets_the_pressed_state() {
        let mut state = ButtonInteractionState::new();

        state.apply(ButtonSignal::PressDownAt(PointerPosition::new(12.0, 8.0)));

        assert!(state.is_pressed());
    }
}
