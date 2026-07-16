//! Rounded polygon construction and corner smoothing.

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

mod corner;

use core::f32::consts::TAU;

use iced::Point;

use super::{cubic::*, math::*};
use corner::PolygonCorner;

#[derive(Debug, Clone, PartialEq)]
pub(super) struct RoundedPolygon {
    pub(super) features: Vec<Feature>,
    pub(super) center: Point,
    pub(super) cubics: Vec<Cubic>,
}

impl RoundedPolygon {
    pub(super) fn from_features(features: Vec<Feature>, center: Point) -> Self {
        let cubics = polygon_cubics(&features, center);

        Self {
            features,
            center,
            cubics,
        }
    }

    pub(super) fn from_vertices(
        vertices: &[Point],
        per_vertex_rounding: &[CornerRounding],
        center: Option<Point>,
    ) -> Self {
        assert!(vertices.len() >= 3);
        assert_eq!(vertices.len(), per_vertex_rounding.len());

        let rounded_corners: Vec<PolygonCorner> = (0..vertices.len())
            .map(|index| {
                PolygonCorner::new(
                    vertices[(index + vertices.len() - 1) % vertices.len()],
                    vertices[index],
                    vertices[(index + 1) % vertices.len()],
                    per_vertex_rounding[index],
                )
            })
            .collect();
        let cut_adjusts: Vec<(f32, f32)> = (0..vertices.len())
            .map(|index| {
                let expected_round_cut = rounded_corners[index].expected_round_cut
                    + rounded_corners[(index + 1) % vertices.len()].expected_round_cut;
                let expected_cut = rounded_corners[index].expected_cut()
                    + rounded_corners[(index + 1) % vertices.len()].expected_cut();
                let side_size = point_distance(point_sub(
                    vertices[index],
                    vertices[(index + 1) % vertices.len()],
                ));

                if expected_round_cut > side_size {
                    (side_size / expected_round_cut, 0.0)
                } else if expected_cut > side_size {
                    (
                        1.0,
                        (side_size - expected_round_cut) / (expected_cut - expected_round_cut),
                    )
                } else {
                    (1.0, 1.0)
                }
            })
            .collect();
        let corners: Vec<Vec<Cubic>> = (0..vertices.len())
            .map(|index| {
                let (round_cut_ratio0, cut_ratio0) =
                    cut_adjusts[(index + vertices.len() - 1) % vertices.len()];
                let (round_cut_ratio1, cut_ratio1) = cut_adjusts[index];
                let allowed_cut0 = rounded_corners[index].expected_round_cut * round_cut_ratio0
                    + (rounded_corners[index].expected_cut()
                        - rounded_corners[index].expected_round_cut)
                        * cut_ratio0;
                let allowed_cut1 = rounded_corners[index].expected_round_cut * round_cut_ratio1
                    + (rounded_corners[index].expected_cut()
                        - rounded_corners[index].expected_round_cut)
                        * cut_ratio1;

                rounded_corners[index].get_cubics(allowed_cut0, allowed_cut1)
            })
            .collect();
        let mut features = Vec::with_capacity(vertices.len() * 2);

        for index in 0..vertices.len() {
            let previous = vertices[(index + vertices.len() - 1) % vertices.len()];
            let current = vertices[index];
            let next = vertices[(index + 1) % vertices.len()];
            let convex = convex(previous, current, next);

            features.push(Feature::Corner {
                cubics: corners[index].clone(),
                convex,
            });
            features.push(Feature::Edge(vec![Cubic::straight_line(
                corners[index].last().unwrap().anchor1_x(),
                corners[index].last().unwrap().anchor1_y(),
                corners[(index + 1) % vertices.len()]
                    .first()
                    .unwrap()
                    .anchor0_x(),
                corners[(index + 1) % vertices.len()]
                    .first()
                    .unwrap()
                    .anchor0_y(),
            )]));
        }

        Self::from_features(
            features,
            center.unwrap_or_else(|| calculate_center(vertices)),
        )
    }

    pub(super) fn transformed(&self, f: impl Fn(Point) -> Point + Copy) -> Self {
        Self::from_features(
            self.features
                .iter()
                .map(|feature| feature.transformed(f))
                .collect(),
            f(self.center),
        )
    }

    pub(super) fn normalized(&self) -> Self {
        let bounds = self.calculate_bounds(true);
        let width = bounds_width(bounds);
        let height = bounds_height(bounds);
        let side = width.max(height);

        if side < DISTANCE_EPSILON {
            return self.clone();
        }

        let offset_x = (side - width) / 2.0 - bounds[0];
        let offset_y = (side - height) / 2.0 - bounds[1];

        self.transformed(|point| {
            Point::new((point.x + offset_x) / side, (point.y + offset_y) / side)
        })
    }

    pub(super) fn calculate_bounds(&self, approximate: bool) -> [f32; 4] {
        cubics_bounds(&self.cubics, approximate)
    }

    pub(super) fn calculate_max_bounds(&self) -> [f32; 4] {
        let mut max_dist_squared = 0.0_f32;

        for cubic in &self.cubics {
            let anchor_distance = distance_squared(
                cubic.anchor0_x() - self.center.x,
                cubic.anchor0_y() - self.center.y,
            );
            let middle = cubic.point_on_curve(0.5);
            let middle_distance =
                distance_squared(middle.x - self.center.x, middle.y - self.center.y);

            max_dist_squared = max_dist_squared.max(anchor_distance.max(middle_distance));
        }

        let distance = max_dist_squared.sqrt();

        [
            self.center.x - distance,
            self.center.y - distance,
            self.center.x + distance,
            self.center.y + distance,
        ]
    }
}

pub(super) fn rounded_polygon_circle(
    num_vertices: usize,
    radius: f32,
    center: Point,
) -> RoundedPolygon {
    let theta = std::f32::consts::PI / num_vertices as f32;
    let polygon_radius = radius / theta.cos();
    let vertices = vertices_from_num_verts(num_vertices, polygon_radius, center);
    let roundings = vec![CornerRounding::new(radius); num_vertices];

    RoundedPolygon::from_vertices(&vertices, &roundings, Some(center))
}

pub(super) fn rounded_polygon_star(
    num_vertices_per_radius: usize,
    radius: f32,
    inner_radius: f32,
    rounding: CornerRounding,
    center: Point,
) -> RoundedPolygon {
    assert!(radius > 0.0 && inner_radius > 0.0 && inner_radius < radius);

    let vertices =
        star_vertices_from_num_verts(num_vertices_per_radius, radius, inner_radius, center);
    let roundings = vec![rounding; vertices.len()];

    RoundedPolygon::from_vertices(&vertices, &roundings, Some(center))
}

pub(super) fn vertices_from_num_verts(
    num_vertices: usize,
    radius: f32,
    center: Point,
) -> Vec<Point> {
    (0..num_vertices)
        .map(|index| radial_to_cartesian(radius, TAU / num_vertices as f32 * index as f32, center))
        .collect()
}

pub(super) fn star_vertices_from_num_verts(
    num_vertices_per_radius: usize,
    radius: f32,
    inner_radius: f32,
    center: Point,
) -> Vec<Point> {
    let mut vertices = Vec::with_capacity(num_vertices_per_radius * 2);

    for index in 0..num_vertices_per_radius {
        vertices.push(radial_to_cartesian(
            radius,
            TAU / num_vertices_per_radius as f32 * index as f32,
            center,
        ));
        vertices.push(radial_to_cartesian(
            inner_radius,
            std::f32::consts::PI / num_vertices_per_radius as f32 * (2 * index + 1) as f32,
            center,
        ));
    }

    vertices
}

pub(super) fn polygon_cubics(features: &[Feature], center: Point) -> Vec<Cubic> {
    let mut cubics = Vec::new();
    let mut first_cubic = None;
    let mut last_cubic: Option<Cubic> = None;
    let mut first_feature_split_start = None;
    let mut first_feature_split_end = None;

    if !features.is_empty() && features[0].cubics().len() == 3 {
        let (start, end) = features[0].cubics()[1].split(0.5);
        first_feature_split_start = Some(vec![features[0].cubics()[0], start]);
        first_feature_split_end = Some(vec![end, features[0].cubics()[2]]);
    }

    for index in 0..=features.len() {
        let feature_cubics: Option<&[Cubic]> = if index == 0 {
            first_feature_split_end
                .as_deref()
                .or(Some(features[0].cubics()))
        } else if index == features.len() {
            first_feature_split_start.as_deref()
        } else {
            Some(features[index].cubics())
        };

        let Some(feature_cubics) = feature_cubics else {
            break;
        };

        for cubic in feature_cubics {
            if !cubic.zero_length() {
                if let Some(last) = last_cubic.take() {
                    cubics.push(last);
                }

                last_cubic = Some(*cubic);
                let _ = first_cubic.get_or_insert(*cubic);
            } else if let Some(last) = last_cubic.as_mut() {
                last.points[6] = cubic.anchor1_x();
                last.points[7] = cubic.anchor1_y();
            }
        }
    }

    if let (Some(last), Some(first)) = (last_cubic, first_cubic) {
        cubics.push(Cubic::new(
            Point::new(last.anchor0_x(), last.anchor0_y()),
            Point::new(last.control0_x(), last.control0_y()),
            Point::new(last.control1_x(), last.control1_y()),
            Point::new(first.anchor0_x(), first.anchor0_y()),
        ));
    } else {
        cubics.push(Cubic::new(center, center, center, center));
    }

    cubics
}
