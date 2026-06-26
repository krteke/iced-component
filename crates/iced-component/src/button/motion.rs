use aura_anim::prelude::Animatable;

/// Animatable visual values for an animated button.
#[derive(Clone, Copy, Debug, PartialEq, Animatable)]
pub struct ButtonMotion {
    /// Content scale multiplier.
    pub scale: f32,
    /// Vertical shadow offset multiplier.
    pub shadow_y: f32,
    /// Background emphasis multiplier.
    pub bg_alpha: f32,
    /// Border glow opacity.
    pub border_glow: f32,
    /// Focus ring opacity.
    pub focus_alpha: f32,
}

impl ButtonMotion {
    pub(super) const fn idle() -> Self {
        Self::idle_with_focus(false)
    }

    pub(super) const fn idle_with_focus(focused: bool) -> Self {
        Self {
            scale: 1.0,
            shadow_y: 1.0,
            bg_alpha: 1.0,
            border_glow: if focused { 1.0 } else { 0.0 },
            focus_alpha: if focused { 1.0 } else { 0.0 },
        }
    }

    pub(super) const fn hovered(focused: bool) -> Self {
        Self {
            shadow_y: 1.2,
            ..Self::idle_with_focus(focused)
        }
    }

    pub(super) const fn pressed(focused: bool) -> Self {
        Self {
            scale: 0.98,
            shadow_y: 0.35,
            bg_alpha: 0.95,
            ..Self::idle_with_focus(focused)
        }
    }

    pub(super) const fn disabled(focused: bool) -> Self {
        Self {
            scale: 1.0,
            shadow_y: 0.0,
            bg_alpha: 0.45,
            border_glow: 0.0,
            focus_alpha: if focused { 0.5 } else { 0.0 },
        }
    }
}
