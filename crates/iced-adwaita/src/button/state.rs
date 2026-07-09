use iced_component_core::component::StyleChange;

use crate::button::{ButtonEvent, ButtonStyleState};

/// Style synchronization reason for a button.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ButtonSync {
    /// Manual synchronization request.
    Manual,
    /// Synchronization after a style revision change.
    StyleChanged(StyleChange),
}

/// Component-level button state and synchronization signals.
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

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct ButtonState {
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

impl ButtonState {
    pub(crate) fn apply(&mut self, signal: ButtonSignal) {
        match signal {
            ButtonSignal::HoverEnter => self.pointer = PointerState::Hovered,
            ButtonSignal::HoverExit => {
                self.pointer = PointerState::Outside;
                self.pressed = false;
            }
            ButtonSignal::PressDown => {
                if !self.disabled {
                    self.pressed = true;
                }
            }
            ButtonSignal::PressUp => self.pressed = false,
            ButtonSignal::Focus => self.focused = true,
            ButtonSignal::Blur => self.focused = false,
            ButtonSignal::SetDisabled(disabled) => self.set_disabled(disabled),
            ButtonSignal::Sync(_) => {}
        }
    }

    pub(crate) fn apply_event<Action>(&mut self, event: ButtonEvent<Action>) -> Option<Action> {
        match event {
            ButtonEvent::Signal(signal) => {
                self.apply(signal);
                None
            }
            ButtonEvent::Pressed(action) => {
                let disabled = self.disabled;
                self.pressed = false;
                (!disabled).then_some(action)
            }
        }
    }

    pub(crate) const fn is_disabled(self) -> bool {
        self.disabled
    }

    pub(crate) const fn is_focused(self) -> bool {
        self.focused
    }

    pub(crate) const fn style_state(self) -> ButtonStyleState {
        if self.disabled {
            ButtonStyleState::Disabled
        } else if self.pressed {
            ButtonStyleState::Pressed
        } else if matches!(self.pointer, PointerState::Hovered) {
            ButtonStyleState::Hovered
        } else {
            ButtonStyleState::Idle
        }
    }

    fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
        if disabled {
            self.pointer = PointerState::Outside;
            self.pressed = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::button::{ButtonEvent, ButtonSignal, ButtonStyleState};

    use super::ButtonState;

    #[test]
    fn disabling_clears_transient_interaction_state() {
        let mut state = ButtonState::default();

        state.apply(ButtonSignal::HoverEnter);
        state.apply(ButtonSignal::PressDown);
        state.apply(ButtonSignal::SetDisabled(true));

        assert!(state.is_disabled());
        assert_eq!(state.style_state(), ButtonStyleState::Disabled);
    }

    #[test]
    fn disabled_pressed_event_does_not_emit_action() {
        let mut state = ButtonState::default();

        state.apply(ButtonSignal::SetDisabled(true));

        assert_eq!(state.apply_event(ButtonEvent::Pressed("save")), None);
    }
}
