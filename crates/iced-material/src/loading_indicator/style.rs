use spectrum_theme::Color;

use iced_component_core::anim::MotionError;

use crate::{context::ViewCx, theme::tokens::LoadingIndicatorTokens};

use super::{LoadingIndicator, LoadingIndicatorMode, LoadingIndicatorMotion};

/// Optional instance-level loading indicator colors.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LoadingIndicatorStyle {
    /// Uncontained active shape color.
    pub active: Option<Color>,
    /// Contained circular background color.
    pub container: Option<Color>,
    /// Active shape color when contained.
    pub contained_active: Option<Color>,
}

impl LoadingIndicatorStyle {
    /// Creates an empty style override.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            active: None,
            container: None,
            contained_active: None,
        }
    }
}

/// Resolved visual inputs for one loading indicator.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LoadingIndicatorVisual {
    /// Square allocation size in logical pixels.
    pub size: f32,
    /// Uncontained active shape color.
    pub active: Color,
    /// Contained circular background color.
    pub container: Color,
    /// Active shape color when contained.
    pub contained_active: Color,
}

impl LoadingIndicatorVisual {
    pub(super) fn resolve(
        tokens: LoadingIndicatorTokens,
        size: Option<f32>,
        style: LoadingIndicatorStyle,
        motion: LoadingIndicatorMotion,
    ) -> Self {
        Self {
            size: size.unwrap_or_else(|| tokens.size.value()).max(1.0),
            active: style.active.unwrap_or_else(|| motion.active()),
            container: style.container.unwrap_or_else(|| motion.container()),
            contained_active: style
                .contained_active
                .unwrap_or_else(|| motion.contained_active()),
        }
    }
}

/// Read-only loading indicator state consumed by the Canvas view.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LoadingIndicatorSnapshot {
    /// Current progress mode.
    pub mode: LoadingIndicatorMode,
    /// Whether the circular color container is enabled.
    pub contained: bool,
    /// Current indeterminate timeline phase.
    pub phase: f32,
    /// Fully resolved visual inputs.
    pub visual: LoadingIndicatorVisual,
}

impl LoadingIndicatorSnapshot {
    pub(super) fn resolve(
        indicator: &LoadingIndicator,
        cx: &ViewCx<'_>,
    ) -> Result<Self, MotionError> {
        let tokens = cx.theme().pack().loading_indicator;
        let motion = indicator.motion_for_view(cx)?;

        Ok(Self {
            mode: indicator.mode(),
            contained: indicator.is_contained(),
            phase: indicator.phase(),
            visual: LoadingIndicatorVisual::resolve(
                tokens,
                indicator.explicit_size(),
                indicator.style(),
                motion,
            ),
        })
    }
}
