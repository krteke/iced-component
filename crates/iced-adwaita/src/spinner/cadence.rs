use core::f32::consts::{FRAC_PI_2, TAU};

use iced::time::Duration;

use super::SpinnerSample;

/// Declarative cadence for a fixed-length indeterminate spinner.
///
/// This independent adwaita-like preset deliberately uses a stable arc instead
/// of imitating an undocumented sweep choreography. It is not an implementation
/// of, or endorsed by, GNOME or libadwaita.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpinnerCadence {
    revolution: Duration,
    sweep_radians: f32,
    phase_offset_radians: f32,
}

impl SpinnerCadence {
    /// Creates a fixed-arc cadence from one orbit period, sweep, and offset.
    #[must_use]
    pub const fn new(revolution: Duration, sweep_radians: f32, phase_offset_radians: f32) -> Self {
        Self {
            revolution,
            sweep_radians,
            phase_offset_radians,
        }
    }

    /// Returns the duration of one complete orbit.
    #[must_use]
    pub const fn revolution(&self) -> Duration {
        self.revolution
    }

    /// Returns the fixed visible sweep in radians.
    #[must_use]
    pub const fn sweep_radians(&self) -> f32 {
        self.sweep_radians
    }

    /// Returns the orbit offset in radians.
    #[must_use]
    pub const fn phase_offset_radians(&self) -> f32 {
        self.phase_offset_radians
    }

    pub(super) fn sample(&self, elapsed: Duration) -> SpinnerSample {
        let orbit = fraction(elapsed, self.revolution);
        let sweep = self.sweep_radians.clamp(0.0, TAU);
        let start = orbit * TAU + self.phase_offset_radians - sweep * 0.5;

        SpinnerSample::new(start, sweep)
    }
}

impl Default for SpinnerCadence {
    fn default() -> Self {
        Self::new(Duration::from_millis(1_240), TAU * 0.26, -FRAC_PI_2)
    }
}

fn fraction(elapsed: Duration, period: Duration) -> f32 {
    let seconds = period.as_secs_f32();

    if seconds <= f32::EPSILON {
        0.0
    } else {
        (elapsed.as_secs_f32() / seconds).rem_euclid(1.0)
    }
}

#[cfg(test)]
mod tests {
    use iced::time::Duration;

    use super::SpinnerCadence;

    #[test]
    fn sweep_is_constant_throughout_the_orbit() {
        let cadence = SpinnerCadence::default();
        let expected = cadence.sweep_radians();

        for millis in (0..4_000).step_by(19) {
            assert_close(
                cadence.sample(Duration::from_millis(millis)).sweep_radians,
                expected,
            );
        }
    }

    #[test]
    fn a_complete_orbit_returns_to_the_same_arc_start() {
        let cadence = SpinnerCadence::default();

        assert_close(
            cadence.sample(Duration::ZERO).arc_start_radians,
            cadence.sample(cadence.revolution()).arc_start_radians,
        );
    }

    #[test]
    fn invalid_sweeps_are_clamped_to_a_full_or_empty_arc() {
        let full = SpinnerCadence::new(Duration::from_secs(1), 10.0, 0.0);
        let empty = SpinnerCadence::new(Duration::from_secs(1), -1.0, 0.0);

        assert_close(
            full.sample(Duration::ZERO).sweep_radians,
            core::f32::consts::TAU,
        );
        assert_close(empty.sample(Duration::ZERO).sweep_radians, 0.0);
    }

    fn assert_close(actual: f32, expected: f32) {
        assert!((actual - expected).abs() < 0.001);
    }
}
