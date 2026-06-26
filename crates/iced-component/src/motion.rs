//! Shared motion configuration used by animated components.

pub use aura_anim::prelude::{Duration, Timing};

/// Returns `timing` unchanged unless reduced motion is enabled.
///
/// Reduced timings preserve the easing curve for callers that inspect timing
/// metadata, but complete immediately.
#[must_use]
pub fn reduce_timing(timing: Timing, reduce_motion: bool) -> Timing {
    if reduce_motion {
        timing.with_duration(Duration::ZERO)
    } else {
        timing
    }
}

#[cfg(test)]
mod tests {
    use super::{Timing, reduce_timing};

    #[test]
    fn reduce_timing_preserves_regular_timing_when_disabled() {
        let timing = Timing::ease_out(160.0);

        assert_eq!(reduce_timing(timing, false), timing);
    }

    #[test]
    fn reduce_timing_zeroes_duration_when_enabled() {
        let timing = Timing::ease_out(160.0);
        let reduced = reduce_timing(timing, true);

        assert!(reduced.duration().is_zero());
        assert_eq!(reduced.easing(), timing.easing());
    }
}
