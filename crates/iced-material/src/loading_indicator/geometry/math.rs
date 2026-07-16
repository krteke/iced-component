//! Geometry measurement and point math.

#![allow(
    clippy::cast_precision_loss,
    clippy::float_cmp,
    clippy::manual_midpoint
)]

use iced::Point;

use super::cubic::Cubic;

pub(super) fn progress_in_range(progress: f32, from: f32, to: f32) -> bool {
    if to >= from {
        (from..=to).contains(&progress)
    } else {
        progress >= from || progress <= to
    }
}

pub(super) fn progress_distance(first: f32, second: f32) -> f32 {
    let distance = (first - second).abs();

    distance.min(1.0 - distance)
}

pub(super) fn measure_cubic(cubic: Cubic) -> f32 {
    closest_progress_to(cubic, f32::INFINITY).1
}

pub(super) fn find_cubic_cut_point(cubic: Cubic, measure: f32) -> f32 {
    closest_progress_to(cubic, measure).0
}

pub(super) fn closest_progress_to(cubic: Cubic, threshold: f32) -> (f32, f32) {
    const SEGMENTS: usize = 3;
    let mut total = 0.0;
    let mut remainder = threshold;
    let mut previous = Point::new(cubic.anchor0_x(), cubic.anchor0_y());

    for index in 1..=SEGMENTS {
        let progress = index as f32 / SEGMENTS as f32;
        let point = cubic.point_on_curve(progress);
        let segment = point_distance(point_sub(point, previous));

        if segment >= remainder {
            return (
                progress - (1.0 - remainder / segment) / SEGMENTS as f32,
                threshold,
            );
        }

        remainder -= segment;
        total += segment;
        previous = point;
    }

    (1.0, total)
}

pub(super) fn update_cubic_bounds_axis(
    anchor0: f32,
    control0: f32,
    control1: f32,
    anchor1: f32,
    point: impl Fn(f32) -> f32,
    min_value: &mut f32,
    max_value: &mut f32,
) {
    let a = -anchor0 + 3.0 * control0 - 3.0 * control1 + anchor1;
    let b = 2.0 * anchor0 - 4.0 * control0 + 2.0 * control1;
    let c = -anchor0 + control0;

    if a.abs() < DISTANCE_EPSILON {
        if b != 0.0 {
            let t = 2.0 * c / (-2.0 * b);
            update_bounds_with_curve_point(t, &point, min_value, max_value);
        }
    } else {
        let discriminant = b * b - 4.0 * a * c;

        if discriminant >= 0.0 {
            update_bounds_with_curve_point(
                (-b + discriminant.sqrt()) / (2.0 * a),
                &point,
                min_value,
                max_value,
            );
            update_bounds_with_curve_point(
                (-b - discriminant.sqrt()) / (2.0 * a),
                &point,
                min_value,
                max_value,
            );
        }
    }
}

pub(super) fn update_bounds_with_curve_point(
    t: f32,
    point: &impl Fn(f32) -> f32,
    min_value: &mut f32,
    max_value: &mut f32,
) {
    if (0.0..=1.0).contains(&t) {
        let value = point(t);
        *min_value = min_value.min(value);
        *max_value = max_value.max(value);
    }
}

pub(super) fn cubics_bounds(cubics: &[Cubic], approximate: bool) -> [f32; 4] {
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;

    for cubic in cubics {
        let bounds = cubic.calculate_bounds(approximate);

        min_x = min_x.min(bounds[0]);
        min_y = min_y.min(bounds[1]);
        max_x = max_x.max(bounds[2]);
        max_y = max_y.max(bounds[3]);
    }

    [min_x, min_y, max_x, max_y]
}

pub(super) fn bounds_width(bounds: [f32; 4]) -> f32 {
    bounds[2] - bounds[0]
}

pub(super) fn bounds_height(bounds: [f32; 4]) -> f32 {
    bounds[3] - bounds[1]
}

pub(super) fn bounds_center(bounds: [f32; 4]) -> Point {
    Point::new((bounds[0] + bounds[2]) / 2.0, (bounds[1] + bounds[3]) / 2.0)
}

pub(super) fn calculate_center(vertices: &[Point]) -> Point {
    let sum = vertices
        .iter()
        .fold(Point::ORIGIN, |sum, point| point_add(sum, *point));

    point_scale(sum, 1.0 / vertices.len() as f32)
}

pub(super) fn radial_to_cartesian(radius: f32, angle: f32, center: Point) -> Point {
    point_add(
        point_scale(direction_vector_from_angle(angle), radius),
        center,
    )
}

pub(super) fn convex(previous: Point, current: Point, next: Point) -> bool {
    point_clockwise(point_sub(current, previous), point_sub(next, current))
}

pub(super) fn rotate_point(point: Point, rotation: f32) -> Point {
    let cos = rotation.cos();
    let sin = rotation.sin();

    Point::new(point.x * cos - point.y * sin, point.x * sin + point.y * cos)
}

pub(super) fn rotate_point_around(point: Point, center: Point, rotation: f32) -> Point {
    point_add(rotate_point(point_sub(point, center), rotation), center)
}

pub(super) fn point_add(first: Point, second: Point) -> Point {
    Point::new(first.x + second.x, first.y + second.y)
}

pub(super) fn point_sub(first: Point, second: Point) -> Point {
    Point::new(first.x - second.x, first.y - second.y)
}

pub(super) fn point_scale(point: Point, scale: f32) -> Point {
    Point::new(point.x * scale, point.y * scale)
}

pub(super) fn point_lerp(first: Point, second: Point, progress: f32) -> Point {
    Point::new(
        lerp(first.x, second.x, progress),
        lerp(first.y, second.y, progress),
    )
}

pub(super) fn point_distance(point: Point) -> f32 {
    distance_components(point.x, point.y)
}

pub(super) fn point_direction(point: Point) -> Point {
    let distance = point_distance(point);

    assert!(distance > 0.0);
    point_scale(point, 1.0 / distance)
}

pub(super) fn point_dot(first: Point, second: Point) -> f32 {
    first.x * second.x + first.y * second.y
}

pub(super) fn point_clockwise(first: Point, second: Point) -> bool {
    first.x * second.y - first.y * second.x > 0.0
}

pub(super) fn rotate90(point: Point) -> Point {
    Point::new(-point.y, point.x)
}

pub(super) fn direction_vector(x: f32, y: f32) -> Point {
    let distance = distance_components(x, y);

    assert!(distance > 0.0);
    Point::new(x / distance, y / distance)
}

pub(super) fn direction_vector_from_angle(angle: f32) -> Point {
    Point::new(angle.cos(), angle.sin())
}

pub(super) fn distance_components(x: f32, y: f32) -> f32 {
    (x * x + y * y).sqrt()
}

pub(super) fn distance_squared(x: f32, y: f32) -> f32 {
    x * x + y * y
}

pub(super) fn distance_squared_point(point: Point) -> f32 {
    distance_squared(point.x, point.y)
}

pub(super) fn square(value: f32) -> f32 {
    value * value
}

pub(super) fn lerp(start: f32, end: f32, progress: f32) -> f32 {
    (1.0 - progress) * start + progress * end
}

pub(super) fn positive_modulo(value: f32, modulus: f32) -> f32 {
    (value % modulus + modulus) % modulus
}

pub(super) const DISTANCE_EPSILON: f32 = 1e-4;
pub(super) const ANGLE_EPSILON: f32 = 1e-6;
