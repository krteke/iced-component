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

/// Theme-independent button interaction input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ButtonSignal {
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
    /// Button enabled or disabled state changed.
    SetDisabled(bool),
    /// Synchronizes the visual target without changing interaction state.
    Sync(ButtonSync),
}

/// Button interaction input or completed application action.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ButtonEvent<Action> {
    /// A state or style synchronization signal.
    Signal(ButtonSignal),
    /// A completed press action.
    Pressed(Action),
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
            ButtonSignal::PressDown if !self.disabled => self.pressed = true,
            ButtonSignal::PressDown | ButtonSignal::Sync(_) => {}
            ButtonSignal::PressUp => self.pressed = false,
            ButtonSignal::Focus => self.focused = true,
            ButtonSignal::Blur => self.focused = false,
            ButtonSignal::SetDisabled(disabled) => self.set_disabled(disabled),
        }
    }

    /// Applies an event and returns its application action when enabled.
    pub fn apply_event<Action>(&mut self, event: ButtonEvent<Action>) -> Option<Action> {
        match event {
            ButtonEvent::Signal(signal) => {
                self.apply(signal);
                None
            }
            ButtonEvent::Pressed(action) => {
                self.pressed = false;
                (!self.disabled).then_some(action)
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
    use super::{ButtonEvent, ButtonInteractionState, ButtonSignal};

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
    fn disabled_button_does_not_emit_actions() {
        let mut state = ButtonInteractionState::new();

        state.apply(ButtonSignal::SetDisabled(true));

        assert_eq!(state.apply_event(ButtonEvent::Pressed("save")), None);
    }

    #[test]
    fn completed_action_releases_the_pressed_state() {
        let mut state = ButtonInteractionState::new();

        state.apply(ButtonSignal::PressDown);

        assert_eq!(
            state.apply_event(ButtonEvent::Pressed("save")),
            Some("save")
        );
        assert!(!state.is_pressed());
    }
}
