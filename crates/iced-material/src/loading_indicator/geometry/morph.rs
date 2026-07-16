//! Feature-aware polygon morph matching.

#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::float_cmp,
    clippy::manual_midpoint,
    clippy::needless_pass_by_value,
    clippy::unused_self,
    clippy::wildcard_imports
)]

mod mapping;
mod measured;

use iced::Point;

use super::{cubic::*, math::*, polygon::RoundedPolygon};
use mapping::feature_mapper;
use measured::MeasuredPolygon;

#[derive(Debug, Clone)]
pub(super) struct Morph {
    pairs: Vec<(Cubic, Cubic)>,
}

impl Morph {
    pub(super) fn new(start: RoundedPolygon, end: RoundedPolygon) -> Self {
        Self {
            pairs: match_polygons(&start, &end),
        }
    }

    pub(super) fn as_cubics(&self, progress: f32) -> Vec<Cubic> {
        let mut cubics = Vec::with_capacity(self.pairs.len());
        let mut first_cubic = None;
        let mut last_cubic = None;

        for (start, end) in &self.pairs {
            let cubic = Cubic {
                points: std::array::from_fn(|index| {
                    lerp(start.points[index], end.points[index], progress)
                }),
            };

            let _ = first_cubic.get_or_insert(cubic);
            if let Some(last) = last_cubic.take() {
                cubics.push(last);
            }
            last_cubic = Some(cubic);
        }

        if let (Some(last), Some(first)) = (last_cubic, first_cubic) {
            cubics.push(Cubic::new(
                Point::new(last.anchor0_x(), last.anchor0_y()),
                Point::new(last.control0_x(), last.control0_y()),
                Point::new(last.control1_x(), last.control1_y()),
                Point::new(first.anchor0_x(), first.anchor0_y()),
            ));
        }

        cubics
    }
}

fn match_polygons(start: &RoundedPolygon, end: &RoundedPolygon) -> Vec<(Cubic, Cubic)> {
    let measured_start = MeasuredPolygon::measure(start);
    let measured_end = MeasuredPolygon::measure(end);
    let mapper = feature_mapper(&measured_start.features, &measured_end.features);
    let end_cut_point = mapper.map(0.0);
    let shifted_start = measured_start;
    let shifted_end = measured_end.cut_and_shift(end_cut_point);
    let mut pairs = Vec::new();
    let mut start_index = 0;
    let mut end_index = 0;
    let mut start_cubic = shifted_start.cubics.get(start_index).cloned();
    start_index += 1;
    let mut end_cubic = shifted_end.cubics.get(end_index).cloned();
    end_index += 1;

    while let (Some(start), Some(end)) = (start_cubic.clone(), end_cubic.clone()) {
        let start_end_progress = if start_index == shifted_start.cubics.len() {
            1.0
        } else {
            start.end_outline_progress
        };
        let end_end_progress = if end_index == shifted_end.cubics.len() {
            1.0
        } else {
            mapper.map_back(positive_modulo(
                end.end_outline_progress + end_cut_point,
                1.0,
            ))
        };
        let min_progress = start_end_progress.min(end_end_progress);
        let (start_segment, new_start) = if start_end_progress > min_progress + ANGLE_EPSILON {
            let (segment, remainder) = start.cut_at_progress(min_progress);
            (segment, Some(remainder))
        } else {
            let next = shifted_start.cubics.get(start_index).cloned();
            start_index += 1;
            (start, next)
        };
        let (end_segment, new_end) = if end_end_progress > min_progress + ANGLE_EPSILON {
            let (segment, remainder) = end.cut_at_progress(positive_modulo(
                mapper.map(min_progress) - end_cut_point,
                1.0,
            ));
            (segment, Some(remainder))
        } else {
            let next = shifted_end.cubics.get(end_index).cloned();
            end_index += 1;
            (end, next)
        };

        pairs.push((start_segment.cubic, end_segment.cubic));
        start_cubic = new_start;
        end_cubic = new_end;
    }

    assert!(start_cubic.is_none() && end_cubic.is_none());
    pairs
}
