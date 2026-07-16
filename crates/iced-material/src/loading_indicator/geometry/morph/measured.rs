//! Arc-length measurement and outline cutting for polygon morphs.

use super::super::{cubic::*, math::*, polygon::RoundedPolygon};

#[derive(Debug, Clone)]
pub(super) struct ProgressableFeature {
    pub(super) progress: f32,
    pub(super) feature: Feature,
}

#[derive(Debug, Clone)]
pub(super) struct MeasuredCubic {
    pub(super) cubic: Cubic,
    start_outline_progress: f32,
    pub(super) end_outline_progress: f32,
}

impl MeasuredCubic {
    pub(super) fn cut_at_progress(&self, cut_outline_progress: f32) -> (Self, Self) {
        let bounded_cut_outline_progress =
            cut_outline_progress.clamp(self.start_outline_progress, self.end_outline_progress);
        let outline_progress_size = self.end_outline_progress - self.start_outline_progress;
        let progress_from_start = bounded_cut_outline_progress - self.start_outline_progress;
        let relative_progress = progress_from_start / outline_progress_size;
        let measured_size = measure_cubic(self.cubic);
        let t = find_cubic_cut_point(self.cubic, relative_progress * measured_size);
        let (first, second) = self.cubic.split(t);

        (
            Self {
                cubic: first,
                start_outline_progress: self.start_outline_progress,
                end_outline_progress: bounded_cut_outline_progress,
            },
            Self {
                cubic: second,
                start_outline_progress: bounded_cut_outline_progress,
                end_outline_progress: self.end_outline_progress,
            },
        )
    }
}

#[derive(Debug, Clone)]
pub(super) struct MeasuredPolygon {
    pub(super) features: Vec<ProgressableFeature>,
    pub(super) cubics: Vec<MeasuredCubic>,
}

impl MeasuredPolygon {
    fn new(
        features: Vec<ProgressableFeature>,
        cubics: Vec<Cubic>,
        outline_progress: Vec<f32>,
    ) -> Self {
        assert_eq!(outline_progress.len(), cubics.len() + 1);
        assert!((outline_progress[0] - 0.0).abs() < DISTANCE_EPSILON);
        assert!((outline_progress[outline_progress.len() - 1] - 1.0).abs() < DISTANCE_EPSILON);

        let mut measured_cubics = Vec::new();
        let mut start_outline_progress = 0.0;

        for index in 0..cubics.len() {
            if outline_progress[index + 1] - outline_progress[index] > DISTANCE_EPSILON {
                measured_cubics.push(MeasuredCubic {
                    cubic: cubics[index],
                    start_outline_progress,
                    end_outline_progress: outline_progress[index + 1],
                });
                start_outline_progress = outline_progress[index + 1];
            }
        }

        if let Some(last) = measured_cubics.last_mut() {
            last.end_outline_progress = 1.0;
        }

        Self {
            features,
            cubics: measured_cubics,
        }
    }

    pub(super) fn measure(polygon: &RoundedPolygon) -> Self {
        let mut cubics = Vec::new();
        let mut feature_to_cubic = Vec::new();

        for feature in &polygon.features {
            for (cubic_index, cubic) in feature.cubics().iter().enumerate() {
                if feature.is_corner() && cubic_index == feature.cubics().len() / 2 {
                    feature_to_cubic.push((feature.clone(), cubics.len()));
                }
                cubics.push(*cubic);
            }
        }

        let mut measures = Vec::with_capacity(cubics.len() + 1);
        let mut total = 0.0;
        measures.push(total);

        for cubic in &cubics {
            total += measure_cubic(*cubic);
            measures.push(total);
        }

        let outline_progress: Vec<f32> = measures.iter().map(|measure| measure / total).collect();
        let features = feature_to_cubic
            .into_iter()
            .map(|(feature, index)| ProgressableFeature {
                progress: positive_modulo(
                    (outline_progress[index] + outline_progress[index + 1]) / 2.0,
                    1.0,
                ),
                feature,
            })
            .collect();

        Self::new(features, cubics, outline_progress)
    }

    pub(super) fn cut_and_shift(&self, cutting_point: f32) -> Self {
        assert!((0.0..=1.0).contains(&cutting_point));

        if cutting_point < DISTANCE_EPSILON {
            return self.clone();
        }

        let target_index = self
            .cubics
            .iter()
            .position(|cubic| {
                cutting_point >= cubic.start_outline_progress
                    && cutting_point <= cubic.end_outline_progress
            })
            .unwrap_or(self.cubics.len() - 1);
        let target = &self.cubics[target_index];
        let (first, second) = target.cut_at_progress(cutting_point);
        let mut cubics = Vec::with_capacity(self.cubics.len() + 1);

        cubics.push(second.cubic);
        for index in 1..self.cubics.len() {
            cubics.push(self.cubics[(index + target_index) % self.cubics.len()].cubic);
        }
        cubics.push(first.cubic);

        let mut outline_progress = Vec::with_capacity(self.cubics.len() + 2);

        for index in 0..self.cubics.len() + 2 {
            outline_progress.push(match index {
                0 => 0.0,
                n if n == self.cubics.len() + 1 => 1.0,
                _ => {
                    let cubic_index = (target_index + index - 1) % self.cubics.len();
                    positive_modulo(
                        self.cubics[cubic_index].end_outline_progress - cutting_point,
                        1.0,
                    )
                }
            });
        }

        let features = self
            .features
            .iter()
            .map(|feature| ProgressableFeature {
                progress: positive_modulo(feature.progress - cutting_point, 1.0),
                feature: feature.feature.clone(),
            })
            .collect();

        Self::new(features, cubics, outline_progress)
    }
}
