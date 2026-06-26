use aura_anim::prelude::Animatable;

use crate::theme::SurfaceRole;

/// Animatable visual values for themed surfaces.
#[derive(Clone, Copy, Debug, PartialEq, Animatable)]
pub struct SurfaceMotion {
    /// Shadow/elevation multiplier.
    pub elevation: f32,
    /// Border opacity multiplier.
    pub border_alpha: f32,
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
            border_alpha: if hovered { 1.0 } else { 0.82 },
        }
    }
}
