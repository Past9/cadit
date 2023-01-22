use crate::{ESpace, ESpace2, ESpace3, EVec2, EVec3, EVector, HVec1, HVec2, HVec3, HVector, TOL};
use std::fmt::Debug;

pub trait ELine: Debug + Clone {
    type Space: ESpace;
    type Point: EVector<Space = Self::Space>;

    fn dist_to_point(&self, point: &Self::Point) -> f64;
    fn contains_point(&self, point: &Self::Point) -> bool;
}

/// An infinite line in 2D Euclidean space
#[derive(Debug, Clone)]
pub struct ELine2 {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}
impl ELine2 {
    pub fn from_pos_and_dir(pos: EVec2, dir: EVec2) -> Self {
        let dir = dir.normalize();

        let a = dir.y;
        let b = -dir.x;
        let c = dir.x * pos.y - dir.y * pos.x;

        Self { a, b, c }
    }
}
impl ELine for ELine2 {
    type Space = ESpace2;
    type Point = EVec2;

    fn dist_to_point(&self, point: &Self::Point) -> f64 {
        (self.a * point.x + self.b * point.y + self.c).abs()
            / (self.a.powi(2) + self.b.powi(2)).sqrt()
    }

    fn contains_point(&self, point: &Self::Point) -> bool {
        let eval = self.a * point.x + self.b * point.y + self.c;
        eval.abs() <= TOL
    }
}
impl MakeImplicit for ELine2 {
    type Input = HVec2;
    type Output = HVec1;

    fn make_implicit(&self, control_point: &Self::Input) -> Self::Output {
        HVec1 {
            x: control_point.x * self.a + control_point.y * self.b + self.c,
            h: control_point.h,
        }
    }
}

/// An infinite line in 3D Euclidean space
#[derive(Debug, Clone)]
pub struct ELine3 {
    // Plane 1
    pub a1: f64,
    pub b1: f64,
    pub c1: f64,
    pub d1: f64,

    // Plane 2
    pub a2: f64,
    pub b2: f64,
    pub c2: f64,
    pub d2: f64,
}
impl ELine3 {
    pub fn from_pos_and_dir(pos: EVec3, dir: EVec3) -> Self {
        let dir = dir.normalize();

        let x0 = pos.x;
        let y0 = pos.y;
        let z0 = pos.z;

        let a = dir.x;
        let b = dir.y;
        let c = dir.z;

        /*
        let a1 = -b;
        let b1 = a;
        let c1 = 0.0;
        let d1 = (b * x0) - (a * y0);

        let a2 = c;
        let b2 = 0.0;
        let c2 = a;
        let d2 = (c * x0) - (a * z0);
        */

        let a1 = -b * c;
        let b1 = a * c;
        let c1 = 0.0;
        let d1 = (b * c * x0) - (a * c * y0);

        let a2 = -b * c;
        let b2 = 0.0;
        let c2 = a * b;
        let d2 = (b * c * x0) - (a * b * z0);

        Self {
            a1,
            b1,
            c1,
            d1,
            a2,
            b2,
            c2,
            d2,
        }
    }
}
impl ELine for ELine3 {
    type Space = ESpace3;
    type Point = EVec3;

    fn dist_to_point(&self, point: &Self::Point) -> f64 {
        let dist1 = (self.a1 * point.x + self.b1 * point.y + self.c1 * point.z + self.d1).abs()
            / (self.a1.powi(2) + self.b1.powi(2) + self.c1.powi(2)).sqrt();

        let dist2 = (self.a2 * point.x + self.b2 * point.y + self.c2 * point.z + self.d2).abs()
            / (self.a2.powi(2) + self.b2.powi(2) + self.c2.powi(2)).sqrt();

        EVec2::new(dist1, dist2).magnitude()
    }

    fn contains_point(&self, point: &Self::Point) -> bool {
        // Check if point lies in first plane
        let eval = self.a1 * point.x + self.b1 * point.y + self.c1 * point.z + self.d1;
        if eval.abs() > TOL {
            return false;
        }

        // Check if point lies in second plane
        let eval = self.a2 * point.x + self.b2 * point.y + self.c2 * point.z + self.d2;
        if eval.abs() > TOL {
            return false;
        }

        // If point lies in both planes, it's on the line
        true
    }
}
impl MakeImplicit for ELine3 {
    type Input = HVec3;
    type Output = HVec2;

    fn make_implicit(&self, control_point: &Self::Input) -> Self::Output {
        HVec2 {
            x: control_point.x * self.a1
                + control_point.y * self.b1
                + control_point.z * self.c1
                + self.d1,
            y: control_point.x * self.a2
                + control_point.y * self.b2
                + control_point.z * self.c2
                + self.d2,
            h: control_point.h,
        }
    }
}

/// Trait that enables taking a control point from a rational (the control point
/// is in homogeneous space) parametric spline and converting it into a coefficient
/// for an implicit version of that spline in the next lowest homogeneous space. This
/// implicit spline lies along the X-axis of that space with its "control points"
/// (coefficients) evenly spaced from `x == 0.0` to `x == 1.0`. Points on the resulting
/// spline will have the same distance (in homogeneous space) from the X-axis as the
/// original control points have from the line. Useful for evaluating error tolerance
/// between rendering primitives and true splines during tesselation.
pub trait MakeImplicit<L: ELine = Self> {
    type Input: HVector<Space = <<L as ELine>::Space as ESpace>::Homogeneous>;
    type Output: HVector<Space = <<<L as ELine>::Space as ESpace>::Lower as ESpace>::Homogeneous>;

    fn make_implicit(&self, control_point: &Self::Input) -> Self::Output;
}
