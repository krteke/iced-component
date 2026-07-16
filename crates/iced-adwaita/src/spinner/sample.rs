use core::f32::consts::TAU;

/// Render-ready arc geometry sampled from a [`SpinnerTimeline`](super::SpinnerTimeline).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpinnerSample {
    /// Clockwise angle where the visible arc begins, in radians.
    pub arc_start_radians: f32,
    /// Visible arc sweep, in radians.
    pub sweep_radians: f32,
}

impl SpinnerSample {
    pub(super) fn new(arc_start_radians: f32, sweep_radians: f32) -> Self {
        Self {
            arc_start_radians: arc_start_radians.rem_euclid(TAU),
            sweep_radians: sweep_radians.clamp(0.0, TAU),
        }
    }
}
