use iced_component_core::component::button::ButtonInteractionState;

use crate::button::ButtonStyleState;

pub(crate) trait ButtonStateExt {
    fn style_state(self) -> ButtonStyleState;
}

impl ButtonStateExt for ButtonInteractionState {
    fn style_state(self) -> ButtonStyleState {
        if self.is_disabled() {
            ButtonStyleState::Disabled
        } else if self.is_pressed() {
            ButtonStyleState::Pressed
        } else if self.is_hovered() {
            ButtonStyleState::Hovered
        } else {
            ButtonStyleState::Idle
        }
    }
}

#[cfg(test)]
mod tests {
    use iced_component_core::component::button::{ButtonInteractionState, ButtonSignal};

    use super::ButtonStateExt;

    #[test]
    fn focus_does_not_change_the_profile_style_state() {
        let mut state = ButtonInteractionState::new();

        state.apply(ButtonSignal::Focus);

        assert_eq!(state.style_state(), crate::button::ButtonStyleState::Idle);
    }
}
