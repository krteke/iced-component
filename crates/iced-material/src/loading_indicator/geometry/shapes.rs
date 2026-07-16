//! Material expressive loading shape definitions.

#![allow(clippy::cast_precision_loss)]

use core::f32::consts::{FRAC_PI_2, FRAC_PI_4, TAU};

use iced::Point;

use super::{
    cubic::CornerRounding,
    math::{point_distance, point_sub, rotate_point, rotate_point_around},
    polygon::{RoundedPolygon, rounded_polygon_circle, rounded_polygon_star},
};

pub(super) fn indeterminate_loading_polygons() -> Vec<RoundedPolygon> {
    vec![
        material_soft_burst(),
        material_cookie9(),
        material_pentagon(),
        material_pill(),
        material_sunny(),
        material_cookie4(),
        material_oval(),
    ]
}

pub(super) fn determinate_loading_polygons() -> Vec<RoundedPolygon> {
    vec![
        material_circle().transformed(|point| rotate_point(point, TAU / 20.0)),
        material_soft_burst(),
    ]
}

fn material_circle() -> RoundedPolygon {
    rounded_polygon_circle(10, 1.0, Point::ORIGIN).normalized()
}

fn material_oval() -> RoundedPolygon {
    rounded_polygon_circle(8, 1.0, Point::ORIGIN)
        .transformed(|point| Point::new(point.x, point.y * 0.64))
        .transformed(|point| rotate_point(point, -FRAC_PI_4))
        .normalized()
}

fn material_pill() -> RoundedPolygon {
    custom_material_polygon(
        &[
            ShapeVertex::new(0.961, 0.039, CornerRounding::new(0.426)),
            ShapeVertex::new(1.001, 0.428, CornerRounding::UNROUNDED),
            ShapeVertex::new(1.000, 0.609, CornerRounding::new(1.0)),
        ],
        2,
        true,
    )
    .normalized()
}

fn material_pentagon() -> RoundedPolygon {
    custom_material_polygon(
        &[
            ShapeVertex::new(0.500, -0.009, CornerRounding::new(0.172)),
            ShapeVertex::new(1.030, 0.365, CornerRounding::new(0.164)),
            ShapeVertex::new(0.828, 0.970, CornerRounding::new(0.169)),
        ],
        1,
        true,
    )
    .normalized()
}

fn material_sunny() -> RoundedPolygon {
    rounded_polygon_star(8, 1.0, 0.8, CornerRounding::new(0.15), Point::ORIGIN).normalized()
}

fn material_cookie4() -> RoundedPolygon {
    custom_material_polygon(
        &[
            ShapeVertex::new(1.237, 1.236, CornerRounding::new(0.258)),
            ShapeVertex::new(0.500, 0.918, CornerRounding::new(0.233)),
        ],
        4,
        false,
    )
    .normalized()
}

fn material_cookie9() -> RoundedPolygon {
    rounded_polygon_star(9, 1.0, 0.8, CornerRounding::new(0.5), Point::ORIGIN)
        .transformed(|point| rotate_point(point, -FRAC_PI_2))
        .normalized()
}

fn material_soft_burst() -> RoundedPolygon {
    custom_material_polygon(
        &[
            ShapeVertex::new(0.193, 0.277, CornerRounding::new(0.053)),
            ShapeVertex::new(0.176, 0.055, CornerRounding::new(0.053)),
        ],
        10,
        false,
    )
    .normalized()
}

struct ShapeVertex {
    point: Point,
    rounding: CornerRounding,
}

impl ShapeVertex {
    fn new(x: f32, y: f32, rounding: CornerRounding) -> Self {
        Self {
            point: Point::new(x, y),
            rounding,
        }
    }
}

fn custom_material_polygon(points: &[ShapeVertex], reps: usize, mirroring: bool) -> RoundedPolygon {
    let center = Point::new(0.5, 0.5);
    let repeated = repeat_material_vertices(points, reps, center, mirroring);
    let vertices: Vec<Point> = repeated.iter().map(|vertex| vertex.point).collect();
    let roundings: Vec<CornerRounding> = repeated.iter().map(|vertex| vertex.rounding).collect();

    RoundedPolygon::from_vertices(&vertices, &roundings, Some(center))
}

fn repeat_material_vertices(
    points: &[ShapeVertex],
    reps: usize,
    center: Point,
    mirroring: bool,
) -> Vec<ShapeVertex> {
    if mirroring {
        let angles: Vec<f32> = points
            .iter()
            .map(|vertex| (vertex.point.y - center.y).atan2(vertex.point.x - center.x))
            .collect();
        let distances: Vec<f32> = points
            .iter()
            .map(|vertex| point_distance(point_sub(vertex.point, center)))
            .collect();
        let actual_reps = reps * 2;
        let section_angle = TAU / actual_reps as f32;
        let mut vertices = Vec::with_capacity(points.len() * actual_reps);

        for rep in 0..actual_reps {
            for index in 0..points.len() {
                let source = if rep % 2 == 0 {
                    index
                } else {
                    points.len() - 1 - index
                };

                if source > 0 || rep % 2 == 0 {
                    let angle = section_angle * rep as f32
                        + if rep % 2 == 0 {
                            angles[source]
                        } else {
                            section_angle - angles[source] + 2.0 * angles[0]
                        };

                    vertices.push(ShapeVertex::new(
                        center.x + angle.cos() * distances[source],
                        center.y + angle.sin() * distances[source],
                        points[source].rounding,
                    ));
                }
            }
        }

        vertices
    } else {
        let mut vertices = Vec::with_capacity(points.len() * reps);

        for index in 0..points.len() * reps {
            let source = index % points.len();
            let rep = index / points.len();
            let point =
                rotate_point_around(points[source].point, center, rep as f32 * TAU / reps as f32);

            vertices.push(ShapeVertex {
                point,
                rounding: points[source].rounding,
            });
        }

        vertices
    }
}
