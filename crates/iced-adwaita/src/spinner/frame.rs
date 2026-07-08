use core::f32::consts::PI;
use iced::time::Duration;
use std::f32::consts::TAU;

const START_ANGLE: f32 = PI * 0.35;
const SPIN_DURATION_MS: f32 = 1_200.0;
const MIN_ARC_LENGTH: f32 = PI * 0.015;
const MAX_ARC_LENGTH: f32 = PI * 0.9;
const IDLE_DISTANCE: f32 = PI * 0.9;
const OVERLAP_DISTANCE: f32 = PI * 0.7;
const EXTEND_DISTANCE: f32 = PI * 1.1;
const CONTRACT_DISTANCE: f32 = PI * 1.35;
const SHAPE_PERIOD: f32 = IDLE_DISTANCE + EXTEND_DISTANCE + CONTRACT_DISTANCE - OVERLAP_DISTANCE;
const STATIC_PROGRESS: f32 = EXTEND_DISTANCE - OVERLAP_DISTANCE * 0.5;

/// One sampled Adwaita spinner frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpinnerFrame {
    /// Clockwise angle where the visible arc starts, in radians.
    pub arc_start_radians: f32,
    /// Clockwise visible arc sweep, in radians.
    pub sweep_radians: f32,
}

impl SpinnerFrame {
    /// Samples the official Adwaita spinner timeline at `elapsed`.
    #[must_use]
    pub fn from_elapsed(elapsed: Duration) -> Self {
        Self::from_progress(progress_from_elapsed(elapsed))
    }

    /// Returns the static paintable frame Adwaita uses when no animation exists.
    #[must_use]
    pub fn static_frame() -> Self {
        Self::from_progress(STATIC_PROGRESS)
    }

    fn from_progress(progress: f32) -> Self {
        let base = progress.rem_euclid(TAU * 53.0);
        let start = normalize(base + arc_start(base) + START_ANGLE);
        let end = normalize(base + arc_end(base) + START_ANGLE);

        Self {
            arc_start_radians: end,
            sweep_radians: clockwise_distance(end, start),
        }
    }
}

fn arc_start(progress: f32) -> f32 {
    let angle = progress.rem_euclid(SHAPE_PERIOD);
    let t = if angle > EXTEND_DISTANCE {
        1.0
    } else {
        ease_in_out_sine(angle / EXTEND_DISTANCE)
    };

    lerp(MIN_ARC_LENGTH, MAX_ARC_LENGTH, t) - angle * MAX_ARC_LENGTH / SHAPE_PERIOD
}

fn arc_end(progress: f32) -> f32 {
    let angle = progress.rem_euclid(SHAPE_PERIOD);
    let t = if angle < EXTEND_DISTANCE - OVERLAP_DISTANCE {
        0.0
    } else if angle > SHAPE_PERIOD - IDLE_DISTANCE {
        1.0
    } else {
        ease_in_out_sine((angle - EXTEND_DISTANCE + OVERLAP_DISTANCE) / CONTRACT_DISTANCE)
    };

    lerp(0.0, MAX_ARC_LENGTH - MIN_ARC_LENGTH, t) - angle * MAX_ARC_LENGTH / SHAPE_PERIOD
}

fn progress_from_elapsed(elapsed: Duration) -> f32 {
    elapsed.as_secs_f32() * TAU * 1_000.0 / SPIN_DURATION_MS
}

fn normalize(angle: f32) -> f32 {
    angle.rem_euclid(TAU)
}

fn clockwise_distance(from: f32, to: f32) -> f32 {
    (to - from).rem_euclid(TAU)
}

fn ease_in_out_sine(t: f32) -> f32 {
    -(f32::cos(core::f32::consts::PI * t) - 1.0) * 0.5
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

#[cfg(test)]
mod tests {
    use super::{MAX_ARC_LENGTH, MIN_ARC_LENGTH, SpinnerFrame, TAU, progress_from_elapsed};
    use iced::time::Duration;

    #[test]
    fn frame_loops_without_rotation_jump_after_full_adwaita_cycle() {
        let start = SpinnerFrame::from_elapsed(Duration::ZERO);
        let end = SpinnerFrame::from_elapsed(Duration::from_millis(63_600));

        assert_close(start.arc_start_radians, end.arc_start_radians);
        assert_close(start.sweep_radians, end.sweep_radians);
    }

    #[test]
    fn rotation_period_is_twelve_hundred_milliseconds() {
        let rotated = progress_from_elapsed(Duration::from_millis(1_200));

        assert_close(rotated.rem_euclid(TAU), 0.0);
    }

    #[test]
    fn shape_period_is_fifteen_hundred_ninety_milliseconds() {
        let start = SpinnerFrame::from_elapsed(Duration::ZERO);
        let shaped = SpinnerFrame::from_elapsed(Duration::from_millis(1_590));

        assert_close(start.sweep_radians, shaped.sweep_radians);
    }

    #[test]
    fn sweep_stays_inside_official_arc_range() {
        for millis in (0..63_600).step_by(137) {
            let frame = SpinnerFrame::from_elapsed(Duration::from_millis(millis));

            assert!(frame.sweep_radians >= MIN_ARC_LENGTH - 0.001);
            assert!(frame.sweep_radians <= MAX_ARC_LENGTH + 0.001);
        }
    }

    fn assert_close(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < 0.001,
            "{actual} was not close to {expected}"
        );
    }
}
