use aura_anim::prelude::Timing;
use iced_component_core::component::button::ButtonInteractionState;

use super::ButtonStyleState;

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
            ButtonStyleState::Hover
        } else if self.is_focused() {
            ButtonStyleState::Focus
        } else {
            ButtonStyleState::Idle
        }
    }
}

pub(crate) fn interaction_timing() -> Timing {
    Timing::linear(15.0)
}

pub(crate) fn sync_timing() -> Timing {
    Timing::ease_out(200.0)
}
