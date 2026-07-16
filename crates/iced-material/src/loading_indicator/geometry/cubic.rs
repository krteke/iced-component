//! Cubic curves and polygon feature primitives.

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

use iced::Point;

use super::math::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct CornerRounding {
    pub(super) radius: f32,
    pub(super) smoothing: f32,
}

impl CornerRounding {
    pub(super) const UNROUNDED: Self = Self {
        radius: 0.0,
        smoothing: 0.0,
    };

    pub(super) const fn new(radius: f32) -> Self {
        Self {
            radius,
            smoothing: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Cubic {
    pub(super) points: [f32; 8],
}

impl Cubic {
    pub(super) fn new(anchor0: Point, control0: Point, control1: Point, anchor1: Point) -> Self {
        Self {
            points: [
                anchor0.x, anchor0.y, control0.x, control0.y, control1.x, control1.y, anchor1.x,
                anchor1.y,
            ],
        }
    }

    pub(super) fn from_points(
        anchor0: Point,
        control0: Point,
        control1: Point,
        anchor1: Point,
    ) -> Self {
        Self::new(anchor0, control0, control1, anchor1)
    }

    pub(super) fn straight_line(x0: f32, y0: f32, x1: f32, y1: f32) -> Self {
        Self::new(
            Point::new(x0, y0),
            Point::new(lerp(x0, x1, 1.0 / 3.0), lerp(y0, y1, 1.0 / 3.0)),
            Point::new(lerp(x0, x1, 2.0 / 3.0), lerp(y0, y1, 2.0 / 3.0)),
            Point::new(x1, y1),
        )
    }

    pub(super) fn circular_arc(
        center_x: f32,
        center_y: f32,
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
    ) -> Self {
        let p0d = direction_vector(x0 - center_x, y0 - center_y);
        let p1d = direction_vector(x1 - center_x, y1 - center_y);
        let rotated_p0 = rotate90(p0d);
        let rotated_p1 = rotate90(p1d);
        let clockwise = point_dot(rotated_p0, Point::new(x1 - center_x, y1 - center_y)) >= 0.0;
        let cosa = point_dot(p0d, p1d);

        if cosa > 0.999 {
            return Self::straight_line(x0, y0, x1, y1);
        }

        let k = distance_components(x0 - center_x, y0 - center_y) * 4.0 / 3.0
            * ((2.0 * (1.0 - cosa)).sqrt() - (1.0 - cosa * cosa).sqrt())
            / (1.0 - cosa)
            * if clockwise { 1.0 } else { -1.0 };

        Self::new(
            Point::new(x0, y0),
            Point::new(x0 + rotated_p0.x * k, y0 + rotated_p0.y * k),
            Point::new(x1 - rotated_p1.x * k, y1 - rotated_p1.y * k),
            Point::new(x1, y1),
        )
    }

    pub(super) fn anchor0_x(&self) -> f32 {
        self.points[0]
    }

    pub(super) fn anchor0_y(&self) -> f32 {
        self.points[1]
    }

    pub(super) fn control0_x(&self) -> f32 {
        self.points[2]
    }

    pub(super) fn control0_y(&self) -> f32 {
        self.points[3]
    }

    pub(super) fn control1_x(&self) -> f32 {
        self.points[4]
    }

    pub(super) fn control1_y(&self) -> f32 {
        self.points[5]
    }

    pub(super) fn anchor1_x(&self) -> f32 {
        self.points[6]
    }

    pub(super) fn anchor1_y(&self) -> f32 {
        self.points[7]
    }

    pub(super) fn point_on_curve(&self, t: f32) -> Point {
        let u = 1.0 - t;

        Point::new(
            self.anchor0_x() * (u * u * u)
                + self.control0_x() * (3.0 * t * u * u)
                + self.control1_x() * (3.0 * t * t * u)
                + self.anchor1_x() * (t * t * t),
            self.anchor0_y() * (u * u * u)
                + self.control0_y() * (3.0 * t * u * u)
                + self.control1_y() * (3.0 * t * t * u)
                + self.anchor1_y() * (t * t * t),
        )
    }

    pub(super) fn split(&self, t: f32) -> (Self, Self) {
        let u = 1.0 - t;
        let point_on_curve = self.point_on_curve(t);

        (
            Self::new(
                Point::new(self.anchor0_x(), self.anchor0_y()),
                Point::new(
                    self.anchor0_x() * u + self.control0_x() * t,
                    self.anchor0_y() * u + self.control0_y() * t,
                ),
                Point::new(
                    self.anchor0_x() * (u * u)
                        + self.control0_x() * (2.0 * u * t)
                        + self.control1_x() * (t * t),
                    self.anchor0_y() * (u * u)
                        + self.control0_y() * (2.0 * u * t)
                        + self.control1_y() * (t * t),
                ),
                point_on_curve,
            ),
            Self::new(
                point_on_curve,
                Point::new(
                    self.control0_x() * (u * u)
                        + self.control1_x() * (2.0 * u * t)
                        + self.anchor1_x() * (t * t),
                    self.control0_y() * (u * u)
                        + self.control1_y() * (2.0 * u * t)
                        + self.anchor1_y() * (t * t),
                ),
                Point::new(
                    self.control1_x() * u + self.anchor1_x() * t,
                    self.control1_y() * u + self.anchor1_y() * t,
                ),
                Point::new(self.anchor1_x(), self.anchor1_y()),
            ),
        )
    }

    pub(super) fn reverse(&self) -> Self {
        Self::new(
            Point::new(self.anchor1_x(), self.anchor1_y()),
            Point::new(self.control1_x(), self.control1_y()),
            Point::new(self.control0_x(), self.control0_y()),
            Point::new(self.anchor0_x(), self.anchor0_y()),
        )
    }

    pub(super) fn transformed(&self, mut f: impl FnMut(Point) -> Point) -> Self {
        Self::from_points(
            f(Point::new(self.anchor0_x(), self.anchor0_y())),
            f(Point::new(self.control0_x(), self.control0_y())),
            f(Point::new(self.control1_x(), self.control1_y())),
            f(Point::new(self.anchor1_x(), self.anchor1_y())),
        )
    }

    pub(super) fn zero_length(&self) -> bool {
        (self.anchor0_x() - self.anchor1_x()).abs() < DISTANCE_EPSILON
            && (self.anchor0_y() - self.anchor1_y()).abs() < DISTANCE_EPSILON
    }

    pub(super) fn calculate_bounds(&self, approximate: bool) -> [f32; 4] {
        if self.zero_length() {
            return [
                self.anchor0_x(),
                self.anchor0_y(),
                self.anchor0_x(),
                self.anchor0_y(),
            ];
        }

        let mut min_x = self.anchor0_x().min(self.anchor1_x());
        let mut min_y = self.anchor0_y().min(self.anchor1_y());
        let mut max_x = self.anchor0_x().max(self.anchor1_x());
        let mut max_y = self.anchor0_y().max(self.anchor1_y());

        if approximate {
            return [
                min_x.min(self.control0_x().min(self.control1_x())),
                min_y.min(self.control0_y().min(self.control1_y())),
                max_x.max(self.control0_x().max(self.control1_x())),
                max_y.max(self.control0_y().max(self.control1_y())),
            ];
        }

        update_cubic_bounds_axis(
            self.anchor0_x(),
            self.control0_x(),
            self.control1_x(),
            self.anchor1_x(),
            |t| self.point_on_curve(t).x,
            &mut min_x,
            &mut max_x,
        );
        update_cubic_bounds_axis(
            self.anchor0_y(),
            self.control0_y(),
            self.control1_y(),
            self.anchor1_y(),
            |t| self.point_on_curve(t).y,
            &mut min_y,
            &mut max_y,
        );

        [min_x, min_y, max_x, max_y]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(super) enum Feature {
    Edge(Vec<Cubic>),
    Corner { cubics: Vec<Cubic>, convex: bool },
}

impl Feature {
    pub(super) fn cubics(&self) -> &[Cubic] {
        match self {
            Self::Edge(cubics) | Self::Corner { cubics, .. } => cubics,
        }
    }

    pub(super) fn transformed(&self, f: impl Fn(Point) -> Point + Copy) -> Self {
        match self {
            Self::Edge(cubics) => {
                Self::Edge(cubics.iter().map(|cubic| cubic.transformed(f)).collect())
            }
            Self::Corner { cubics, convex } => Self::Corner {
                cubics: cubics.iter().map(|cubic| cubic.transformed(f)).collect(),
                convex: *convex,
            },
        }
    }

    pub(super) fn is_corner(&self) -> bool {
        matches!(self, Self::Corner { .. })
    }

    pub(super) fn is_convex_corner(&self) -> bool {
        matches!(self, Self::Corner { convex: true, .. })
    }

    pub(super) fn is_concave_corner(&self) -> bool {
        matches!(self, Self::Corner { convex: false, .. })
    }
}
