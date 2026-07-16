//! Per-vertex corner rounding and smoothing.

use iced::Point;

use super::super::{cubic::*, math::*};

#[derive(Debug, Clone, Copy)]
pub(super) struct PolygonCorner {
    p0: Point,
    p1: Point,
    p2: Point,
    d1: Point,
    d2: Point,
    corner_radius: f32,
    smoothing: f32,
    pub(super) expected_round_cut: f32,
}

#[derive(Debug, Clone, Copy)]
struct FlankingCurve {
    actual_round_cut: f32,
    actual_smoothing_value: f32,
    corner: Point,
    side_start: Point,
    circle_segment_intersection: Point,
    other_circle_segment_intersection: Point,
    circle_center: Point,
    actual_radius: f32,
}

impl PolygonCorner {
    pub(super) fn new(p0: Point, p1: Point, p2: Point, rounding: CornerRounding) -> Self {
        let v01 = point_sub(p0, p1);
        let v21 = point_sub(p2, p1);
        let d01 = point_distance(v01);
        let d21 = point_distance(v21);

        if d01 > 0.0 && d21 > 0.0 {
            let d1 = point_scale(v01, 1.0 / d01);
            let d2 = point_scale(v21, 1.0 / d21);
            let cos_angle = point_dot(d1, d2).clamp(-1.0, 1.0);
            let sin_angle = (1.0 - square(cos_angle)).max(0.0).sqrt();
            let expected_round_cut = if sin_angle > 1e-3 {
                rounding.radius * (cos_angle + 1.0) / sin_angle
            } else {
                0.0
            };

            Self {
                p0,
                p1,
                p2,
                d1,
                d2,
                corner_radius: rounding.radius,
                smoothing: rounding.smoothing,
                expected_round_cut,
            }
        } else {
            Self {
                p0,
                p1,
                p2,
                d1: Point::ORIGIN,
                d2: Point::ORIGIN,
                corner_radius: 0.0,
                smoothing: 0.0,
                expected_round_cut: 0.0,
            }
        }
    }

    pub(super) fn expected_cut(&self) -> f32 {
        (1.0 + self.smoothing) * self.expected_round_cut
    }

    pub(super) fn get_cubics(&self, allowed_cut0: f32, allowed_cut1: f32) -> Vec<Cubic> {
        let allowed_cut = allowed_cut0.min(allowed_cut1);

        if self.expected_round_cut < DISTANCE_EPSILON
            || allowed_cut < DISTANCE_EPSILON
            || self.corner_radius < DISTANCE_EPSILON
        {
            return vec![Cubic::straight_line(
                self.p1.x, self.p1.y, self.p1.x, self.p1.y,
            )];
        }

        let actual_round_cut = allowed_cut.min(self.expected_round_cut);
        let actual_smoothing0 = self.calculate_actual_smoothing_value(allowed_cut0);
        let actual_smoothing1 = self.calculate_actual_smoothing_value(allowed_cut1);
        let actual_radius = self.corner_radius * actual_round_cut / self.expected_round_cut;
        let center_distance = (square(actual_radius) + square(actual_round_cut)).sqrt();
        let circle_center = point_add(
            self.p1,
            point_scale(
                point_direction(point_scale(point_add(self.d1, self.d2), 0.5)),
                center_distance,
            ),
        );
        let circle_intersection0 = point_add(self.p1, point_scale(self.d1, actual_round_cut));
        let circle_intersection2 = point_add(self.p1, point_scale(self.d2, actual_round_cut));
        let flanking0 = self.compute_flanking_curve(FlankingCurve {
            actual_round_cut,
            actual_smoothing_value: actual_smoothing0,
            corner: self.p1,
            side_start: self.p0,
            circle_segment_intersection: circle_intersection0,
            other_circle_segment_intersection: circle_intersection2,
            circle_center,
            actual_radius,
        });
        let flanking2 = self
            .compute_flanking_curve(FlankingCurve {
                actual_round_cut,
                actual_smoothing_value: actual_smoothing1,
                corner: self.p1,
                side_start: self.p2,
                circle_segment_intersection: circle_intersection2,
                other_circle_segment_intersection: circle_intersection0,
                circle_center,
                actual_radius,
            })
            .reverse();

        vec![
            flanking0,
            Cubic::circular_arc(
                circle_center.x,
                circle_center.y,
                flanking0.anchor1_x(),
                flanking0.anchor1_y(),
                flanking2.anchor0_x(),
                flanking2.anchor0_y(),
            ),
            flanking2,
        ]
    }

    fn calculate_actual_smoothing_value(&self, allowed_cut: f32) -> f32 {
        if allowed_cut > self.expected_cut() {
            self.smoothing
        } else if allowed_cut > self.expected_round_cut {
            self.smoothing * (allowed_cut - self.expected_round_cut)
                / (self.expected_cut() - self.expected_round_cut)
        } else {
            0.0
        }
    }

    fn compute_flanking_curve(&self, curve: FlankingCurve) -> Cubic {
        let side_direction = point_direction(point_sub(curve.side_start, curve.corner));
        let curve_start = point_add(
            curve.corner,
            point_scale(
                side_direction,
                curve.actual_round_cut * (1.0 + curve.actual_smoothing_value),
            ),
        );
        let p = point_lerp(
            curve.circle_segment_intersection,
            point_scale(
                point_add(
                    curve.circle_segment_intersection,
                    curve.other_circle_segment_intersection,
                ),
                0.5,
            ),
            curve.actual_smoothing_value,
        );
        let curve_end = point_add(
            curve.circle_center,
            point_scale(
                direction_vector(p.x - curve.circle_center.x, p.y - curve.circle_center.y),
                curve.actual_radius,
            ),
        );
        let circle_tangent = rotate90(point_sub(curve_end, curve.circle_center));
        let anchor_end =
            line_intersection(curve.side_start, side_direction, curve_end, circle_tangent)
                .unwrap_or(curve.circle_segment_intersection);
        let anchor_start = point_scale(
            point_add(curve_start, point_scale(anchor_end, 2.0)),
            1.0 / 3.0,
        );

        Cubic::from_points(curve_start, anchor_start, anchor_end, curve_end)
    }
}

fn line_intersection(p0: Point, d0: Point, p1: Point, d1: Point) -> Option<Point> {
    let rotated_d1 = rotate90(d1);
    let denominator = point_dot(d0, rotated_d1);

    if denominator.abs() < DISTANCE_EPSILON {
        return None;
    }

    let numerator = point_dot(point_sub(p1, p0), rotated_d1);

    if denominator.abs() < DISTANCE_EPSILON * numerator.abs() {
        return None;
    }

    Some(point_add(p0, point_scale(d0, numerator / denominator)))
}
