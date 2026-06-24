use aura_anim_core::{
    SpringConfig,
    timing::{Delay, Duration, Timing},
};

use super::MotionPreferences;
use crate::motion::{MotionSpring, MotionSpringTokens, transition::MotionTransition};

/// Named duration buckets shared by component interactions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MotionSpeed {
    /// No animation.
    Instant,
    /// Tiny feedback changes such as pressed-state settling.
    Micro,
    /// Small state changes such as hover and press feedback.
    Fast,
    /// Default component state transitions.
    Normal,
    /// Larger transitions such as panel enter or exit.
    Slow,
    /// Long transitions for large layout or presence changes.
    Slower,
}

/// Motion token values shared by animated components.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MotionTokens {
    /// Duration for tiny direct feedback.
    pub micro: Duration,
    /// Duration for fast interactions.
    pub fast: Duration,
    /// Duration for normal interactions.
    pub normal: Duration,
    /// Duration for slow interactions.
    pub slow: Duration,
    /// Duration for larger presence or layout transitions.
    pub slower: Duration,
    /// Default transition for direct component interaction feedback.
    pub interaction: MotionTransition,
    /// Physics-based motion presets.
    pub spring: MotionSpringTokens,
}

impl MotionTokens {
    /// Returns the duration for a speed bucket.
    #[must_use]
    pub fn duration(self, speed: MotionSpeed) -> Duration {
        match speed {
            MotionSpeed::Instant => Duration::ZERO,
            MotionSpeed::Micro => self.micro,
            MotionSpeed::Fast => self.fast,
            MotionSpeed::Normal => self.normal,
            MotionSpeed::Slow => self.slow,
            MotionSpeed::Slower => self.slower,
        }
    }

    /// Returns the spring configuration for a physics preset.
    #[must_use]
    pub fn spring(self, preset: MotionSpring) -> SpringConfig {
        match preset {
            MotionSpring::Soft => self.spring.soft,
            MotionSpring::Balanced => self.spring.balanced,
            MotionSpring::Firm => self.spring.firm,
            MotionSpring::Snappy => self.spring.snappy,
        }
    }

    /// Resolves a transition into `aura-anim` timing.
    #[must_use]
    pub fn timing(self, transition: MotionTransition, preferences: &MotionPreferences) -> Timing {
        let reduce = preferences.reduce_motion() && transition.follow_reduce_motion;
        let duration = if reduce {
            Duration::ZERO
        } else {
            self.duration(transition.speed)
        };
        let delay = if reduce {
            Delay::ZERO
        } else {
            transition.delay
        };

        Timing::new(duration.as_millis())
            .with_delay(delay)
            .with_direction(transition.direction)
            .with_easing(transition.easing)
            .with_iterations(transition.iterations)
    }
}

impl Default for MotionTokens {
    fn default() -> Self {
        Self {
            micro: Duration::from_millis(80.0),
            fast: Duration::from_millis(120.0),
            normal: Duration::from_millis(200.0),
            slow: Duration::from_millis(260.0),
            slower: Duration::from_millis(320.0),
            interaction: MotionTransition::standard(),
            spring: MotionSpringTokens::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use aura_anim_core::timing::{Delay, Direction, Duration, Easing, IterationCount};

    use crate::motion::{MotionPreferences, MotionSpring, MotionSpringTokens};

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
    fn default_interaction_transition_uses_normal_feedback_timing() {
        let preferences = MotionPreferences::default();
        let timing =
            MotionTokens::default().timing(MotionTokens::default().interaction, &preferences);

        assert_eq!(timing.duration(), Duration::from_millis(200.0));
        assert_eq!(timing.easing(), Easing::EaseOut);
    }

    #[test]
    fn tokens_resolve_full_timed_transition_surface() {
        let preferences = MotionPreferences::default();
        let transition = MotionTransition::new(MotionSpeed::Slower, Easing::EaseInOut)
            .with_delay(Delay::from_millis(30.0))
            .with_direction(Direction::Alternate)
            .with_iterations(3);

        let timing = MotionTokens::default().timing(transition, &preferences);

        assert_eq!(timing.duration(), Duration::from_millis(320.0));
        assert_eq!(timing.delay(), Delay::from_millis(30.0));
        assert_eq!(timing.direction(), Direction::Alternate);
        assert_eq!(timing.iterations(), IterationCount::count(3));
    }

    #[test]
    fn tokens_resolve_to_zero_duration_when_motion_is_reduced() {
        let (preferences, _controller) = MotionPreferences::new(true);
        let transition = MotionTransition::new(MotionSpeed::Slow, Easing::EaseInOut)
            .with_delay(Delay::from_millis(50.0));

        let timing = MotionTokens::default().timing(transition, &preferences);

        assert_eq!(timing.duration(), Duration::ZERO);
        assert_eq!(timing.delay(), Delay::ZERO);
        assert_eq!(timing.easing(), Easing::EaseInOut);
    }

    #[test]
    fn transitions_can_opt_out_of_reduce_motion_skipping() {
        let (preferences, _controller) = MotionPreferences::new(true);
        let transition =
            MotionTransition::new(MotionSpeed::Micro, Easing::Linear).follow_reduce_motion(false);

        let timing = MotionTokens::default().timing(transition, &preferences);

        assert_eq!(timing.duration(), Duration::from_millis(80.0));
    }

    #[test]
    fn spring_presets_resolve_to_generic_physics_tokens() {
        let tokens = MotionTokens::default();

        assert_eq!(
            tokens.spring(MotionSpring::Balanced),
            MotionSpringTokens::default().balanced
        );
        assert!(
            tokens.spring(MotionSpring::Snappy).stiffness
                > tokens.spring(MotionSpring::Soft).stiffness
        );
    }
}
