use iced::time::{Duration, Instant};

use super::{SpinnerCadence, SpinnerSample};

/// Playback state for a spinner timeline.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SpinnerPlayback {
    /// The timeline advances when it receives frame timestamps.
    #[default]
    Running,
    /// The timeline retains its current sample until resumed.
    Paused,
    /// The timeline is reset and does not advance until started again.
    Stopped,
}

/// Stateful clock for an indeterminate spinner.
#[derive(Clone, Copy, Debug)]
pub struct SpinnerTimeline {
    playback: SpinnerPlayback,
    elapsed: Duration,
    last_tick: Option<Instant>,
}

impl SpinnerTimeline {
    /// Creates a timeline that starts accumulating on its first frame timestamp.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            playback: SpinnerPlayback::Running,
            elapsed: Duration::ZERO,
            last_tick: None,
        }
    }

    /// Returns the current playback state.
    #[must_use]
    pub const fn playback(&self) -> SpinnerPlayback {
        self.playback
    }

    /// Returns the accumulated active time.
    #[must_use]
    pub const fn elapsed(&self) -> Duration {
        self.elapsed
    }

    /// Starts a fresh active timeline.
    pub fn start(&mut self) {
        self.playback = SpinnerPlayback::Running;
        self.elapsed = Duration::ZERO;
        self.last_tick = None;
    }

    /// Stops and clears the timeline.
    pub fn stop(&mut self) {
        self.playback = SpinnerPlayback::Stopped;
        self.elapsed = Duration::ZERO;
        self.last_tick = None;
    }

    /// Freezes the current sample at `now`.
    pub fn pause(&mut self, now: Instant) -> bool {
        if self.playback != SpinnerPlayback::Running {
            return false;
        }

        let _ = self.advance(now);
        self.playback = SpinnerPlayback::Paused;
        self.last_tick = None;
        true
    }

    /// Continues a paused timeline without rewinding its sample.
    pub fn resume(&mut self) -> bool {
        if self.playback != SpinnerPlayback::Paused {
            return false;
        }

        self.playback = SpinnerPlayback::Running;
        self.last_tick = None;
        true
    }

    /// Resets elapsed time while preserving the playback state.
    pub fn reset(&mut self) {
        self.elapsed = Duration::ZERO;
        self.last_tick = None;
    }

    /// Advances the running timeline to `now`.
    pub fn advance(&mut self, now: Instant) -> bool {
        if self.playback != SpinnerPlayback::Running {
            return false;
        }

        let Some(previous) = self.last_tick.replace(now) else {
            return false;
        };
        let elapsed = now.saturating_duration_since(previous);

        if elapsed.is_zero() {
            return false;
        }

        self.elapsed = self.elapsed.saturating_add(elapsed);
        true
    }

    /// Samples the active cadence at the current elapsed time.
    #[must_use]
    pub fn sample(&self, cadence: SpinnerCadence) -> SpinnerSample {
        cadence.sample(self.elapsed)
    }
}

impl Default for SpinnerTimeline {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use iced::time::{Duration, Instant};

    use super::{SpinnerPlayback, SpinnerTimeline};
    use crate::spinner::SpinnerCadence;

    #[test]
    fn running_timeline_accumulates_frame_deltas() {
        let start = Instant::now();
        let mut timeline = SpinnerTimeline::new();

        assert!(!timeline.advance(start));
        assert!(timeline.advance(start + Duration::from_millis(40)));
        assert_eq!(timeline.elapsed(), Duration::from_millis(40));
    }

    #[test]
    fn pausing_preserves_the_current_sample() {
        let start = Instant::now();
        let mut timeline = SpinnerTimeline::new();
        let cadence = SpinnerCadence::default();
        let _ = timeline.advance(start);
        let _ = timeline.advance(start + Duration::from_millis(200));
        let before = timeline.sample(cadence);

        assert!(timeline.pause(start + Duration::from_millis(200)));
        assert!(!timeline.advance(start + Duration::from_secs(3)));
        assert_eq!(timeline.playback(), SpinnerPlayback::Paused);
        assert_eq!(timeline.sample(cadence), before);
    }

    #[test]
    fn resuming_keeps_elapsed_time_and_waits_for_a_new_tick() {
        let start = Instant::now();
        let mut timeline = SpinnerTimeline::new();
        let _ = timeline.advance(start);
        let _ = timeline.advance(start + Duration::from_millis(100));
        let _ = timeline.pause(start + Duration::from_millis(100));

        assert!(timeline.resume());
        assert!(!timeline.advance(start + Duration::from_secs(1)));
        assert!(timeline.advance(start + Duration::from_secs(1) + Duration::from_millis(50)));
        assert_eq!(timeline.elapsed(), Duration::from_millis(150));
    }

    #[test]
    fn stopping_clears_elapsed_time() {
        let start = Instant::now();
        let mut timeline = SpinnerTimeline::new();
        let _ = timeline.advance(start);
        let _ = timeline.advance(start + Duration::from_millis(100));

        timeline.stop();

        assert_eq!(timeline.playback(), SpinnerPlayback::Stopped);
        assert_eq!(timeline.elapsed(), Duration::ZERO);
    }
}
