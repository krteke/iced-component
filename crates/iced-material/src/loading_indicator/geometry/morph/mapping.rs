//! Corner feature matching between two measured outlines.

use iced::Point;

use super::{
    super::{cubic::Feature, math::*},
    measured::ProgressableFeature,
};

#[derive(Debug, Clone)]
pub(super) struct DoubleMapper {
    source_values: Vec<f32>,
    target_values: Vec<f32>,
}

impl DoubleMapper {
    fn new(mappings: &[(f32, f32)]) -> Self {
        let source_values = mappings.iter().map(|mapping| mapping.0).collect();
        let target_values = mappings.iter().map(|mapping| mapping.1).collect();

        Self {
            source_values,
            target_values,
        }
    }

    pub(super) fn map(&self, progress: f32) -> f32 {
        linear_map(&self.source_values, &self.target_values, progress)
    }

    pub(super) fn map_back(&self, progress: f32) -> f32 {
        linear_map(&self.target_values, &self.source_values, progress)
    }
}

pub(super) fn feature_mapper(
    features1: &[ProgressableFeature],
    features2: &[ProgressableFeature],
) -> DoubleMapper {
    let filtered1: Vec<ProgressableFeature> = features1
        .iter()
        .filter(|feature| feature.feature.is_corner())
        .cloned()
        .collect();
    let filtered2: Vec<ProgressableFeature> = features2
        .iter()
        .filter(|feature| feature.feature.is_corner())
        .cloned()
        .collect();
    let mappings = feature_mapping(&filtered1, &filtered2);

    DoubleMapper::new(&mappings)
}

fn feature_mapping(
    features1: &[ProgressableFeature],
    features2: &[ProgressableFeature],
) -> Vec<(f32, f32)> {
    let mut distances = Vec::new();

    for (index1, feature1) in features1.iter().enumerate() {
        for (index2, feature2) in features2.iter().enumerate() {
            let distance = feature_distance_squared(&feature1.feature, &feature2.feature);

            if distance != f32::MAX {
                distances.push((distance, index1, index2));
            }
        }
    }

    distances.sort_by(|a, b| a.0.total_cmp(&b.0));

    if distances.is_empty() {
        return vec![(0.0, 0.0), (0.5, 0.5)];
    }

    if distances.len() == 1 {
        let (_, index1, index2) = distances[0];
        let f1 = features1[index1].progress;
        let f2 = features2[index2].progress;

        return vec![(f1, f2), ((f1 + 0.5) % 1.0, (f2 + 0.5) % 1.0)];
    }

    let mut helper = MappingHelper::new();

    for (_, index1, index2) in distances {
        helper.add_mapping(features1, features2, index1, index2);
    }

    helper.mapping
}

struct MappingHelper {
    mapping: Vec<(f32, f32)>,
    used1: Vec<usize>,
    used2: Vec<usize>,
}

impl MappingHelper {
    fn new() -> Self {
        Self {
            mapping: Vec::new(),
            used1: Vec::new(),
            used2: Vec::new(),
        }
    }

    fn add_mapping(
        &mut self,
        features1: &[ProgressableFeature],
        features2: &[ProgressableFeature],
        index1: usize,
        index2: usize,
    ) {
        if self.used1.contains(&index1) || self.used2.contains(&index2) {
            return;
        }

        let f1 = features1[index1].progress;
        let f2 = features2[index2].progress;
        let insertion_index = self
            .mapping
            .iter()
            .position(|mapping| mapping.0 > f1)
            .unwrap_or(self.mapping.len());
        let len = self.mapping.len();

        if len >= 1 {
            let before = self.mapping[(insertion_index + len - 1) % len];
            let after = self.mapping[insertion_index % len];

            if progress_distance(f1, before.0) < DISTANCE_EPSILON
                || progress_distance(f1, after.0) < DISTANCE_EPSILON
                || progress_distance(f2, before.1) < DISTANCE_EPSILON
                || progress_distance(f2, after.1) < DISTANCE_EPSILON
            {
                return;
            }

            if len > 1 && !progress_in_range(f2, before.1, after.1) {
                return;
            }
        }

        self.mapping.insert(insertion_index, (f1, f2));
        self.used1.push(index1);
        self.used2.push(index2);
    }
}

fn feature_distance_squared(first: &Feature, second: &Feature) -> f32 {
    if (first.is_convex_corner() && second.is_concave_corner())
        || (first.is_concave_corner() && second.is_convex_corner())
    {
        return f32::MAX;
    }

    distance_squared_point(point_sub(
        feature_representative_point(first),
        feature_representative_point(second),
    ))
}

fn feature_representative_point(feature: &Feature) -> Point {
    Point::new(
        (feature.cubics().first().unwrap().anchor0_x()
            + feature.cubics().last().unwrap().anchor1_x())
            / 2.0,
        (feature.cubics().first().unwrap().anchor0_y()
            + feature.cubics().last().unwrap().anchor1_y())
            / 2.0,
    )
}

fn linear_map(x_values: &[f32], y_values: &[f32], progress: f32) -> f32 {
    let progress = if progress >= 1.0 {
        0.0
    } else {
        positive_modulo(progress, 1.0)
    };
    let segment_start_index = (0..x_values.len())
        .find(|index| {
            progress_in_range(
                progress,
                x_values[*index],
                x_values[(*index + 1) % x_values.len()],
            )
        })
        .unwrap_or(0);
    let segment_end_index = (segment_start_index + 1) % x_values.len();
    let segment_size_x = positive_modulo(
        x_values[segment_end_index] - x_values[segment_start_index],
        1.0,
    );
    let segment_size_y = positive_modulo(
        y_values[segment_end_index] - y_values[segment_start_index],
        1.0,
    );
    let position = if segment_size_x < 0.001 {
        0.5
    } else {
        positive_modulo(progress - x_values[segment_start_index], 1.0) / segment_size_x
    };

    positive_modulo(
        y_values[segment_start_index] + segment_size_y * position,
        1.0,
    )
}
