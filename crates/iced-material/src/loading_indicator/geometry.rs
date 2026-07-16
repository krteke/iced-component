//! `AndroidX` Material shape morph geometry.
//!
//! Ported from `material-ui-rs` 0.4.3 (MIT), which follows the `AndroidX`
//! Material Shapes feature-mapping algorithm.

mod cubic;
mod math;
mod morph;
mod polygon;
mod shapes;

use core::f32::consts::{FRAC_PI_2, TAU};
use std::sync::OnceLock;

use iced::Point;
use iced_widget::canvas::Path;

use cubic::Cubic;
use math::{
    bounds_center, bounds_height, bounds_width, cubics_bounds, point_add, point_sub,
    rotate_point_around,
};
use morph::Morph;
use polygon::RoundedPolygon;
use shapes::{determinate_loading_polygons, indeterminate_loading_polygons};

const MORPH_INTERVAL_MS: f32 = 650.0;
const GLOBAL_ROTATION_DURATION_MS: f32 = 4_666.0;
const MORPH_SPRING_DAMPING_RATIO: f32 = 0.6;
const MORPH_SPRING_STIFFNESS: f32 = 200.0;
const ACTIVE_INDICATOR_SCALE: f32 = 38.0 / 48.0;

struct ShapeSequence {
    morphs: Vec<Morph>,
    scale: f32,
}

/// Precomputes both shape sequences outside a frame draw when possible.
pub(super) fn prepare() {
    let _ = indeterminate_sequence();
    let _ = determinate_sequence();
}

pub(super) fn loading_shape_path(center: Point, side: f32, phase: f32) -> Path {
    let phase = phase.rem_euclid(1.0);
    let sequence = indeterminate_sequence();
    #[allow(clippy::cast_precision_loss)]
    let morph_count = sequence.morphs.len() as f32;
    let morph_position =
        (phase * GLOBAL_ROTATION_DURATION_MS / MORPH_INTERVAL_MS).rem_euclid(morph_count);
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let from_index = morph_position.floor() as usize;
    let local_progress = morph_position.fract();
    let morph_progress = loading_spring_progress(local_progress);
    #[allow(clippy::cast_precision_loss)]
    let rotation = phase * TAU + (from_index as f32 + 1.0 + morph_progress) * FRAC_PI_2;

    morphed_loading_shape_path(
        &sequence.morphs[from_index],
        center,
        side,
        sequence.scale,
        morph_progress,
        rotation,
    )
}

pub(super) fn determinate_loading_shape_path(center: Point, side: f32, progress: f32) -> Path {
    let progress = progress.clamp(0.0, 1.0);
    let sequence = determinate_sequence();
    let rotation = -progress * core::f32::consts::PI;

    morphed_loading_shape_path(
        &sequence.morphs[0],
        center,
        side,
        sequence.scale,
        progress,
        rotation,
    )
}

fn indeterminate_sequence() -> &'static ShapeSequence {
    static SEQUENCE: OnceLock<ShapeSequence> = OnceLock::new();

    SEQUENCE.get_or_init(|| {
        let polygons = indeterminate_loading_polygons();
        build_sequence(&polygons, true)
    })
}

fn determinate_sequence() -> &'static ShapeSequence {
    static SEQUENCE: OnceLock<ShapeSequence> = OnceLock::new();

    SEQUENCE.get_or_init(|| {
        let polygons = determinate_loading_polygons();
        build_sequence(&polygons, false)
    })
}

fn build_sequence(polygons: &[RoundedPolygon], circular: bool) -> ShapeSequence {
    let scale = loading_shape_scale(polygons);
    let mut morphs = Vec::with_capacity(polygons.len());

    for index in 0..polygons.len() {
        if index + 1 < polygons.len() {
            morphs.push(Morph::new(
                polygons[index].normalized(),
                polygons[index + 1].normalized(),
            ));
        } else if circular {
            morphs.push(Morph::new(
                polygons[index].normalized(),
                polygons[0].normalized(),
            ));
        }
    }

    ShapeSequence { morphs, scale }
}

fn morphed_loading_shape_path(
    morph: &Morph,
    center: Point,
    side: f32,
    scale: f32,
    morph_progress: f32,
    rotation: f32,
) -> Path {
    let cubics = morph.as_cubics(morph_progress);

    processed_cubic_path(&cubics, center, side, scale, rotation)
}

fn loading_spring_progress(progress: f32) -> f32 {
    let seconds = progress.clamp(0.0, 1.0) * MORPH_INTERVAL_MS / 1_000.0;
    let natural_frequency = MORPH_SPRING_STIFFNESS.sqrt();
    let damped_frequency = natural_frequency * (1.0 - MORPH_SPRING_DAMPING_RATIO.powi(2)).sqrt();
    let envelope = (-MORPH_SPRING_DAMPING_RATIO * natural_frequency * seconds).exp();
    let phase = damped_frequency * seconds;
    let response = 1.0
        - envelope
            * (phase.cos()
                + MORPH_SPRING_DAMPING_RATIO / (1.0 - MORPH_SPRING_DAMPING_RATIO.powi(2)).sqrt()
                    * phase.sin());

    response.clamp(0.0, 1.0)
}

fn processed_cubic_path(
    cubics: &[Cubic],
    center: Point,
    side: f32,
    scale: f32,
    rotation: f32,
) -> Path {
    if cubics.is_empty() {
        return Path::new(|_| {});
    }

    let scale = side * scale;
    let source_center = bounds_center(cubics_bounds(cubics, false));
    let translation = point_sub(
        center,
        Point::new(source_center.x * scale, source_center.y * scale),
    );
    let transform = |point: Point| {
        rotate_point_around(
            point_add(Point::new(point.x * scale, point.y * scale), translation),
            center,
            rotation,
        )
    };

    Path::new(|path| {
        path.move_to(transform(Point::new(
            cubics[0].anchor0_x(),
            cubics[0].anchor0_y(),
        )));
        for cubic in cubics {
            path.bezier_curve_to(
                transform(Point::new(cubic.control0_x(), cubic.control0_y())),
                transform(Point::new(cubic.control1_x(), cubic.control1_y())),
                transform(Point::new(cubic.anchor1_x(), cubic.anchor1_y())),
            );
        }
        path.close();
    })
}

fn loading_shape_scale(polygons: &[RoundedPolygon]) -> f32 {
    polygons.iter().fold(1.0_f32, |scale, polygon| {
        let bounds = polygon.calculate_bounds(true);
        let max_bounds = polygon.calculate_max_bounds();
        let scale_x = bounds_width(bounds) / bounds_width(max_bounds);
        let scale_y = bounds_height(bounds) / bounds_height(max_bounds);

        scale.min(scale_x.max(scale_y))
    }) * ACTIVE_INDICATOR_SCALE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indeterminate_sequence_matches_androidx_material_shapes() {
        let polygons = indeterminate_loading_polygons();
        let corners: Vec<_> = polygons
            .iter()
            .map(|polygon| {
                polygon
                    .features
                    .iter()
                    .filter(|feature| feature.is_corner())
                    .count()
            })
            .collect();

        assert_eq!(corners, vec![20, 18, 5, 10, 16, 8, 8]);
    }

    #[test]
    fn shape_sequences_are_cached() {
        assert!(core::ptr::eq(
            indeterminate_sequence(),
            indeterminate_sequence()
        ));
        assert!(core::ptr::eq(
            determinate_sequence(),
            determinate_sequence()
        ));
    }

    #[test]
    fn cached_morphs_produce_finite_closed_outlines() {
        for morph in &indeterminate_sequence().morphs {
            let cubics = morph.as_cubics(0.37);

            assert!(!cubics.is_empty());
            assert!(
                cubics
                    .iter()
                    .flat_map(|cubic| cubic.points)
                    .all(f32::is_finite)
            );
            for pair in cubics.windows(2) {
                assert_close(pair[0].anchor1_x(), pair[1].anchor0_x());
                assert_close(pair[0].anchor1_y(), pair[1].anchor0_y());
            }
            assert_close(cubics.last().unwrap().anchor1_x(), cubics[0].anchor0_x());
            assert_close(cubics.last().unwrap().anchor1_y(), cubics[0].anchor0_y());
        }
    }

    #[test]
    fn spring_reaches_each_shape_before_the_next_interval() {
        assert!((loading_spring_progress(0.0) - 0.0).abs() < 0.001);
        assert!(loading_spring_progress(0.5) > 0.8);
        assert!(loading_spring_progress(1.0) > 0.99);
    }

    #[test]
    fn rotation_safe_scale_stays_inside_the_container() {
        let scale = loading_shape_scale(&indeterminate_loading_polygons());

        assert!(scale > 0.0);
        assert!(scale < ACTIVE_INDICATOR_SCALE);
    }

    fn assert_close(actual: f32, expected: f32) {
        assert!((actual - expected).abs() < 0.001);
    }
}
