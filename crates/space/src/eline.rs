use crate::{ESpace, ESpace2, ESpace3, HVec1, HVec2, HVec3, HVector};

pub trait ELine<S: ESpace> {}

/// An infinite line in 2D Euclidean space
pub struct ELine2 {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}
impl ImplicitifyControlPoint<ESpace2, HVec2, HVec1> for ELine2 {
    fn implicitify_control_point(&self, control_point: HVec2) -> HVec1 {
        HVec1 {
            x: control_point.x * self.a + control_point.y * self.b + self.c,
            h: control_point.h,
        }
    }
}

/// An infinite line in 3D Euclidean space
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
impl ImplicitifyControlPoint<ESpace3, HVec3, HVec2> for ELine3 {
    fn implicitify_control_point(&self, control_point: HVec3) -> HVec2 {
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
pub trait ImplicitifyControlPoint<
    S: ESpace,
    TControlPoint: HVector<S::Homogeneous>,
    TOutput: HVector<<S::Lower as ESpace>::Homogeneous>,
>
{
    fn implicitify_control_point(&self, control_point: TControlPoint) -> TOutput;
}
