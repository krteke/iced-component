use aura_anim_core::timing::{Duration, Easing, Timing};

use super::MotionPreferences;

/// Named duration buckets shared by component interactions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MotionSpeed {
    /// No animation.
    Instant,
    /// Small state changes such as hover and press feedback.
    Fast,
    /// Default component state transitions.
    Normal,
    /// Larger transitions such as panel enter or exit.
    Slow,
}

/// Motion token values shared by animated components.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MotionTokens {
    /// Duration for fast interactions.
    pub fast: Duration,
    /// Duration for normal interactions.
    pub normal: Duration,
    /// Duration for slow interactions.
    pub slow: Duration,
}

/// Unresolved transition intent.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MotionTransition {
    /// Duration bucket to use.
    pub speed: MotionSpeed,
    /// Easing curve to use.
    pub easing: Easing,
}

impl MotionTokens {
    /// Returns the duration for a speed bucket.
    #[must_use]
    pub fn duration(self, speed: MotionSpeed) -> Duration {
        match speed {
            MotionSpeed::Instant => Duration::ZERO,
            MotionSpeed::Fast => self.fast,
            MotionSpeed::Normal => self.normal,
            MotionSpeed::Slow => self.slow,
        }
    }

    /// Resolves a transition into `aura-anim` timing.
    #[must_use]
    pub fn timing(self, transition: MotionTransition, preferences: &MotionPreferences) -> Timing {
        let duration = if preferences.reduce_motion() {
            Duration::ZERO
        } else {
            self.duration(transition.speed)
        };

        Timing::new(duration.as_millis()).with_easing(transition.easing)
    }
}

impl Default for MotionTokens {
    fn default() -> Self {
        Self {
            fast: Duration::from_millis(120.0),
            normal: Duration::from_millis(180.0),
            slow: Duration::from_millis(240.0),
        }
    }
}

impl MotionTransition {
    /// Creates a transition intent from a speed bucket and easing curve.
    #[must_use]
    pub const fn new(speed: MotionSpeed, easing: Easing) -> Self {
        Self { speed, easing }
    }

    /// Default transition for ordinary component state changes.
    #[must_use]
    pub const fn standard() -> Self {
        Self::new(MotionSpeed::Normal, Easing::EaseOut)
    }
}

#[cfg(test)]
mod tests {
    use aura_anim_core::timing::{Duration, Easing};

    use crate::motion::MotionPreferences;

    use super::{MotionSpeed, MotionTokens, MotionTransition};

    #[test]
    fn tokens_resolve_to_aura_timing_when_motion_is_allowed() {
        let preferences = MotionPreferences::default();
        let transition = MotionTransition::new(MotionSpeed::Fast, Easing::EaseOut);

        let timing = MotionTokens::default().timing(transition, &preferences);

        assert_eq!(timing.duration(), Duration::from_millis(120.0));
        assert_eq!(timing.easing(), Easing::EaseOut);
    }

    #[test]
    fn tokens_resolve_to_zero_duration_when_motion_is_reduced() {
        let (preferences, _controller) = MotionPreferences::new(true);
        let transition = MotionTransition::new(MotionSpeed::Slow, Easing::EaseInOut);

        let timing = MotionTokens::default().timing(transition, &preferences);

        assert_eq!(timing.duration(), Duration::ZERO);
        assert_eq!(timing.easing(), Easing::EaseInOut);
    }
}
