//! Widget-local state and timing copied from the Material reference ripple.

use iced::{Point, Size};
use iced_widget::core::time::Instant;
use std::time::Duration;

const MAX_RIPPLES: usize = 10;
const BOUNDED_EXTRA_RADIUS: f32 = 10.0;
const PATTERN_RADIUS_SCALE: f32 = 2.3;
const PATTERN_ENTER_DURATION: Duration = Duration::from_millis(450);
const PATTERN_EXIT_DURATION: Duration = Duration::from_millis(375);
const PATTERN_NOISE_DURATION_MS: f32 = 7_000.0;
const PATTERN_FADE_IN_END: f32 = 0.13;
const PATTERN_FADE_OUT_START: f32 = 0.4;
const PATTERN_FADE_OUT_NOISE_END: f32 = 0.5;
const NOISE_PHASE_DURATION_DIVISOR: f32 = 214.0;

/// Persistent bounded press-ripple queue for one rendered button.
#[derive(Debug, Default)]
pub(crate) struct PressRippleState {
    active_ripple: Option<PressRipple>,
    exiting_ripples: Vec<PressRipple>,
}

impl PressRippleState {
    /// Removes every active or exiting ripple.
    pub(crate) fn clear(&mut self) {
        self.active_ripple = None;
        self.exiting_ripples.clear();
    }

    /// Starts a ripple at a point relative to the button's bounds.
    pub(crate) fn press(&mut self, origin: Point, now: Instant) {
        if let Some(mut ripple) = self.active_ripple.take() {
            ripple.exit(now);
            self.push_exiting(ripple);
        }

        self.active_ripple = Some(PressRipple::new(origin, now));
    }

    /// Moves the active ripple into the fading queue.
    pub(crate) fn release(&mut self, now: Instant) {
        if let Some(mut ripple) = self.active_ripple.take() {
            ripple.exit(now);
            self.push_exiting(ripple);
        }
    }

    /// Removes ripples whose exit phase has completed.
    pub(crate) fn prune(&mut self, now: Instant) {
        self.exiting_ripples
            .retain(|ripple| !ripple.has_finished_exit(now));
    }

    /// Returns whether another redraw is required.
    pub(crate) fn has_visible_ripples(&self, now: Instant) -> bool {
        self.ripple_opacity(now) > 0.0
    }

    /// Iterates shader-ready ripples with non-zero visual alpha.
    pub(crate) fn visible(&self, now: Instant) -> impl Iterator<Item = PatternedRipple> + '_ {
        self.active_ripple
            .iter()
            .chain(&self.exiting_ripples)
            .filter_map(move |ripple| ripple.patterned(now))
    }

    fn ripple_opacity(&self, now: Instant) -> f32 {
        self.active_ripple
            .map_or(0.0, |ripple| ripple.opacity(now))
            .max(
                self.exiting_ripples
                    .iter()
                    .map(|ripple| ripple.opacity(now))
                    .fold(0.0, f32::max),
            )
    }

    fn push_exiting(&mut self, ripple: PressRipple) {
        if self.exiting_ripples.len() >= MAX_RIPPLES {
            let _ = self.exiting_ripples.remove(0);
        }

        self.exiting_ripples.push(ripple);
    }
}

#[derive(Debug, Clone, Copy)]
struct PressRipple {
    origin: Point,
    started_at: Instant,
    exit_started_at: Option<Instant>,
    exit_delay: Duration,
}

impl PressRipple {
    fn new(origin: Point, started_at: Instant) -> Self {
        Self {
            origin,
            started_at,
            exit_started_at: None,
            exit_delay: Duration::ZERO,
        }
    }

    fn exit(&mut self, now: Instant) {
        let elapsed = now.duration_since(self.started_at);

        self.exit_started_at = Some(now);
        self.exit_delay = PATTERN_ENTER_DURATION.saturating_sub(elapsed);
    }

    fn opacity(self, now: Instant) -> f32 {
        let progress = self.patterned_progress(now);
        let fade_in = sub_progress(0.0, PATTERN_FADE_IN_END, progress);
        let fade_out = sub_progress(PATTERN_FADE_OUT_START, 1.0, progress);

        fade_in.min(1.0 - fade_out).clamp(0.0, 1.0)
    }

    fn has_finished_exit(self, now: Instant) -> bool {
        self.exit_started_at.is_some_and(|exit_started_at| {
            now.duration_since(exit_started_at) >= self.exit_delay + PATTERN_EXIT_DURATION
        })
    }

    fn patterned(self, now: Instant) -> Option<PatternedRipple> {
        let progress = self.patterned_progress(now);
        let fade_in = sub_progress(0.0, PATTERN_FADE_IN_END, progress);
        let fade_out_noise =
            sub_progress(PATTERN_FADE_OUT_START, PATTERN_FADE_OUT_NOISE_END, progress);
        let fade_out_ripple = sub_progress(PATTERN_FADE_OUT_START, 1.0, progress);
        let alpha = fade_in.min(1.0 - fade_out_noise).clamp(0.0, 1.0);
        let fade = fade_in.min(1.0 - fade_out_ripple).clamp(0.0, 1.0);

        (fade > 0.0 || alpha > 0.0).then_some(PatternedRipple {
            touch: self.origin,
            progress,
            noise_phase: noise_phase(self, now),
        })
    }

    fn patterned_progress(self, now: Instant) -> f32 {
        if let Some(exit_started_at) = self.exit_started_at {
            let elapsed = now.duration_since(exit_started_at);

            if elapsed <= self.exit_delay {
                self.enter_progress(now)
            } else {
                let exit_elapsed = elapsed.saturating_sub(self.exit_delay);
                lerp(
                    0.5,
                    1.0,
                    (exit_elapsed.as_secs_f32() / PATTERN_EXIT_DURATION.as_secs_f32())
                        .clamp(0.0, 1.0),
                )
            }
        } else {
            self.enter_progress(now)
        }
    }

    fn enter_progress(self, now: Instant) -> f32 {
        let progress = (now.duration_since(self.started_at).as_secs_f32()
            / PATTERN_ENTER_DURATION.as_secs_f32())
        .clamp(0.0, 1.0);

        lerp(0.0, 0.5, legacy_easing(progress))
    }
}

/// Patterned shader values whose timing matches the reference implementation.
#[derive(Debug, Clone, Copy)]
pub(crate) struct PatternedRipple {
    pub(crate) touch: Point,
    pub(crate) progress: f32,
    pub(crate) noise_phase: NoisePhase,
}

impl PatternedRipple {
    pub(crate) fn max_radius(size: Size) -> f32 {
        let half_width = size.width / 2.0;
        let half_height = size.height / 2.0;

        ((half_width * half_width + half_height * half_height).sqrt() + BOUNDED_EXTRA_RADIUS)
            * PATTERN_RADIUS_SCALE
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct NoisePhase {
    pub(crate) sparkle: f32,
    pub(crate) turbulence: f32,
}

fn noise_phase(ripple: PressRipple, now: Instant) -> NoisePhase {
    let elapsed_ms = now.duration_since(ripple.started_at).as_secs_f32() * 1_000.0;
    let max_phase = PATTERN_NOISE_DURATION_MS / NOISE_PHASE_DURATION_DIVISOR;
    let turbulence = (elapsed_ms / NOISE_PHASE_DURATION_DIVISOR).clamp(0.0, max_phase);

    NoisePhase {
        sparkle: turbulence * 0.001,
        turbulence,
    }
}

fn sub_progress(start: f32, end: f32, progress: f32) -> f32 {
    let clamped = progress.clamp(start, end);

    (clamped - start) / (end - start)
}

fn lerp(start: f32, end: f32, progress: f32) -> f32 {
    start + (end - start) * progress
}

fn legacy_easing(progress: f32) -> f32 {
    if progress <= 0.0 {
        return 0.0;
    }
    if progress >= 1.0 {
        return 1.0;
    }

    let mut start = 0.0;
    let mut end = 1.0;
    for _ in 0..20 {
        let midpoint = f32::midpoint(start, end);
        if bezier_axis(midpoint, 0.4, 0.2) < progress {
            start = midpoint;
        } else {
            end = midpoint;
        }
    }

    bezier_axis(f32::midpoint(start, end), 0.0, 1.0).clamp(0.0, 1.0)
}

fn bezier_axis(t: f32, p1: f32, p2: f32) -> f32 {
    let inverse = 1.0 - t;

    3.0 * inverse * inverse * t * p1 + 3.0 * inverse * t * t * p2 + t * t * t
}

#[cfg(test)]
mod tests {
    use iced::Point;
    use iced_widget::core::time::{Duration, Instant};

    use super::PressRippleState;

    #[test]
    fn clear_removes_an_active_ripple() {
        let now = Instant::now();
        let mut ripples = PressRippleState::default();
        ripples.press(Point::new(8.0, 6.0), now);

        let visible_at = now + Duration::from_millis(100);
        assert!(ripples.has_visible_ripples(visible_at));

        ripples.clear();

        assert!(!ripples.has_visible_ripples(visible_at));
        assert_eq!(ripples.visible(visible_at).count(), 0);
    }

    #[test]
    fn released_ripple_holds_until_the_patterned_entry_completes() {
        let start = Instant::now();
        let mut ripples = PressRippleState::default();

        ripples.press(Point::new(12.0, 8.0), start);
        ripples.release(start + Duration::from_millis(20));

        assert!(ripples.has_visible_ripples(start + Duration::from_millis(200)));
        ripples.prune(start + Duration::from_millis(826));
        assert!(!ripples.has_visible_ripples(start + Duration::from_millis(826)));
    }

    #[test]
    fn repeated_presses_keep_the_previous_ripple_exiting() {
        let start = Instant::now();
        let mut ripples = PressRippleState::default();

        ripples.press(Point::new(8.0, 8.0), start);
        ripples.press(Point::new(24.0, 8.0), start + Duration::from_millis(50));

        assert_eq!(
            ripples.visible(start + Duration::from_millis(120)).count(),
            2
        );
    }
}
