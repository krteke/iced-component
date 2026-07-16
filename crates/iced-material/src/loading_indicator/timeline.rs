use iced::time::{Duration, Instant};

const ROTATION_DURATION: Duration = Duration::from_millis(4_666);

#[derive(Clone, Copy, Debug)]
pub(super) struct LoadingTimeline {
    started_at: Option<Instant>,
    elapsed: Duration,
}

impl LoadingTimeline {
    pub(super) const fn new() -> Self {
        Self {
            started_at: None,
            elapsed: Duration::ZERO,
        }
    }

    pub(super) fn advance(&mut self, now: Instant) {
        let started_at = *self.started_at.get_or_insert(now);
        self.elapsed = now.saturating_duration_since(started_at);
    }

    pub(super) fn reset(&mut self) {
        self.started_at = None;
        self.elapsed = Duration::ZERO;
    }

    pub(super) fn phase(&self) -> f32 {
        (self.elapsed.as_secs_f32() / ROTATION_DURATION.as_secs_f32()).rem_euclid(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::{LoadingTimeline, ROTATION_DURATION};
    use iced::time::{Duration, Instant};

    #[test]
    fn phase_matches_the_material_global_rotation_period() {
        let start = Instant::now();
        let mut timeline = LoadingTimeline::new();
        timeline.advance(start);
        timeline.advance(start + ROTATION_DURATION / 2);

        assert!((timeline.phase() - 0.5).abs() < 0.001);

        timeline.advance(start + ROTATION_DURATION + Duration::from_millis(1));
        assert!(timeline.phase() < 0.001);
    }
}
