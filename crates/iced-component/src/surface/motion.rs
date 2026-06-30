use aura_anim::prelude::Animatable;

use crate::theme::SurfaceRole;

/// Animatable visual values for themed surfaces.
#[derive(Clone, Copy, Debug, PartialEq, Animatable)]
pub struct SurfaceMotion {
    /// Shadow/elevation multiplier.
    pub elevation: f32,
    /// Background opacity multiplier.
    pub bg_alpha: f32,
    /// Border opacity multiplier.
    pub border_alpha: f32,
    /// Border width in pixels.
    pub border_width: f32,
    /// Corner radius multiplier.
    pub radius_scale: f32,
    /// Shadow opacity multiplier.
    pub shadow_alpha: f32,
    /// Shadow blur multiplier.
    pub shadow_blur: f32,
}

impl SurfaceMotion {
    pub(super) const fn for_role(role: SurfaceRole, hovered: bool) -> Self {
        let elevation = match role {
            SurfaceRole::Background | SurfaceRole::Regular => 0.0,
            SurfaceRole::Raised if hovered => 1.15,
            SurfaceRole::Raised => 1.0,
        };

        Self {
            elevation,
            bg_alpha: 1.0,
            border_alpha: if hovered { 1.0 } else { 0.82 },
            border_width: 1.0,
            radius_scale: if hovered { 1.02 } else { 1.0 },
            shadow_alpha: if hovered { 1.0 } else { 0.92 },
            shadow_blur: if hovered { 1.06 } else { 1.0 },
        }
    }
}
