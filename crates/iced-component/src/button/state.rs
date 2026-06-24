use crate::button::{ButtonMotion, ButtonStyleState};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(super) struct ButtonState {
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
    pub(super) fn hover_enter(&mut self) {
        self.pointer = PointerState::Hovered;
    }

    pub(super) fn hover_exit(&mut self) {
        self.pointer = PointerState::Outside;
        self.pressed = false;
    }

    pub(super) fn press_down(&mut self) {
        if !self.disabled {
            self.pressed = true;
        }
    }

    pub(super) fn press_up(&mut self) {
        self.pressed = false;
    }

    pub(super) fn focus(&mut self) {
        self.focused = true;
    }

    pub(super) fn blur(&mut self) {
        self.focused = false;
    }

    pub(super) fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
        if disabled {
            self.pointer = PointerState::Outside;
            self.pressed = false;
        }
    }

    pub(super) const fn is_disabled(self) -> bool {
        self.disabled
    }

    pub(super) const fn is_focused(self) -> bool {
        self.focused
    }

    pub(super) const fn style_state(self) -> ButtonStyleState {
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

    pub(super) const fn target_motion(self) -> ButtonMotion {
        match self.style_state() {
            ButtonStyleState::Disabled => ButtonMotion::disabled(self.focused),
            ButtonStyleState::Pressed => ButtonMotion::pressed(self.focused),
            ButtonStyleState::Hovered => ButtonMotion::hovered(self.focused),
            ButtonStyleState::Idle => ButtonMotion::idle_with_focus(self.focused),
        }
    }
}
