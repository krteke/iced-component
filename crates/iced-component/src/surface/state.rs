use crate::surface::{SurfaceInteraction, SurfaceStyleState};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(super) struct SurfaceState {
    hovered: bool,
}

impl SurfaceState {
    pub(super) fn apply(&mut self, interaction: SurfaceInteraction) {
        match interaction {
            SurfaceInteraction::HoverEnter => self.hovered = true,
            SurfaceInteraction::HoverExit => self.hovered = false,
        }
    }

    pub(super) const fn is_hovered(self) -> bool {
        self.hovered
    }

    pub(super) const fn style_state(self) -> SurfaceStyleState {
        if self.hovered {
            SurfaceStyleState::Hovered
        } else {
            SurfaceStyleState::Idle
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SurfaceState;
    use crate::surface::{SurfaceInteraction, SurfaceStyleState};

    #[test]
    fn apply_updates_hover_state() {
        let mut state = SurfaceState::default();

        state.apply(SurfaceInteraction::HoverEnter);
        assert!(state.is_hovered());
        assert_eq!(state.style_state(), SurfaceStyleState::Hovered);

        state.apply(SurfaceInteraction::HoverExit);
        assert!(!state.is_hovered());
        assert_eq!(state.style_state(), SurfaceStyleState::Idle);
    }
}
