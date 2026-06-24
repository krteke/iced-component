use crate::button::{ButtonEvent, ButtonInteraction, ButtonMotion, ButtonStyleState};

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
    pub(super) fn apply(&mut self, interaction: ButtonInteraction) {
        match interaction {
            ButtonInteraction::HoverEnter => self.hover_enter(),
            ButtonInteraction::HoverExit => self.hover_exit(),
            ButtonInteraction::PressDown => self.press_down(),
            ButtonInteraction::PressUp => self.press_up(),
            ButtonInteraction::Focus => self.focus(),
            ButtonInteraction::Blur => self.blur(),
            ButtonInteraction::SetDisabled(disabled) => self.set_disabled(disabled),
        }
    }

    pub(super) fn apply_event<Action>(&mut self, event: ButtonEvent<Action>) -> Option<Action> {
        match event {
            ButtonEvent::Interaction(interaction) => {
                self.apply(interaction);
                None
            }
            ButtonEvent::Pressed(action) => {
                let disabled = self.disabled;
                self.press_up();
                (!disabled).then_some(action)
            }
        }
    }

    fn hover_enter(&mut self) {
        self.pointer = PointerState::Hovered;
    }

    fn hover_exit(&mut self) {
        self.pointer = PointerState::Outside;
        self.pressed = false;
    }

    fn press_down(&mut self) {
        if !self.disabled {
            self.pressed = true;
        }
    }

    fn press_up(&mut self) {
        self.pressed = false;
    }

    fn focus(&mut self) {
        self.focused = true;
    }

    fn blur(&mut self) {
        self.focused = false;
    }

    fn set_disabled(&mut self, disabled: bool) {
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

#[cfg(test)]
mod tests {
    use super::ButtonState;
    use crate::button::{ButtonEvent, ButtonInteraction, ButtonStyleState};

    #[test]
    fn apply_clears_pointer_and_press_when_disabled() {
        let mut state = ButtonState::default();

        state.apply(ButtonInteraction::HoverEnter);
        state.apply(ButtonInteraction::PressDown);
        state.apply(ButtonInteraction::SetDisabled(true));
        state.apply(ButtonInteraction::PressDown);

        assert!(state.is_disabled());
        assert_eq!(state.style_state(), ButtonStyleState::Disabled);

        state.apply(ButtonInteraction::SetDisabled(false));

        assert_eq!(state.style_state(), ButtonStyleState::Idle);
    }

    #[test]
    fn apply_event_releases_and_suppresses_disabled_action() {
        let mut state = ButtonState::default();

        state.apply(ButtonInteraction::PressDown);
        assert_eq!(state.style_state(), ButtonStyleState::Pressed);
        assert_eq!(
            state.apply_event(ButtonEvent::Pressed("save")),
            Some("save")
        );
        assert_eq!(state.style_state(), ButtonStyleState::Idle);

        state.apply(ButtonInteraction::SetDisabled(true));
        assert_eq!(state.apply_event(ButtonEvent::Pressed("save")), None);
        assert_eq!(state.style_state(), ButtonStyleState::Disabled);
    }

    #[test]
    fn apply_updates_focus_state() {
        let mut state = ButtonState::default();

        state.apply(ButtonInteraction::Focus);
        assert!(state.is_focused());

        state.apply(ButtonInteraction::Blur);
        assert!(!state.is_focused());
    }
}
