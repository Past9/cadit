use cgmath::{vec4, Vector3, Vector4};

use crate::{
    control_points::ControlPolygon,
    knots::KnotVector,
    math::{
        b_spline::curve_derivatives_2,
        nurbs::{self, insert_curve_knots, refine_curve},
    },
};

use super::CurveFunction;

pub struct NurbsCurve {
    /// Points that the curve interpolates between
    pub control_points: ControlPolygon<Vector4<f64>>,

    /// Knot vector
    pub knot_vector: KnotVector,

    /// Degree of curve
    degree: usize,
}
impl NurbsCurve {
    pub fn new(
        control_points: ControlPolygon<Vector4<f64>>,
        knot_vector: KnotVector,
        degree: usize,
    ) -> Self {
        assert_eq!(
            knot_vector.len(),
            control_points.len() + degree + 1,
            "Incorrect degree, knot vector length, or number of control points"
        );

        Self {
            control_points: control_points,
            knot_vector: knot_vector,
            degree,
        }
    }

    pub fn refine(&self, new_knots: KnotVector) -> Self {
        let (new_knot_vector, new_control_points) = refine_curve(
            self.degree,
            &self.knot_vector,
            new_knots,
            &self.control_points.to_weighted(),
        );

        Self::new(
            new_control_points.to_unweighted(),
            new_knot_vector,
            self.degree,
        )
    }

    pub fn refine_midpoints(&self) -> Self {
        let mut new_knots = Vec::new();
        let mut last_knot = self.knot_vector.first();
        for knot in self.knot_vector.iter().skip(1) {
            if *knot > last_knot {
                new_knots.push((knot - last_knot) / 2.0 + last_knot);
            }
            last_knot = *knot;
        }

        self.refine(KnotVector::from_vec(new_knots))

        /*
        let (new_knot_vector, new_control_points) = refine_curve(
            self.degree,
            &self.knot_vector,
            KnotVector::from_slice(&new_knots),
            &self.control_points.to_weighted(),
        );

        Self::new(
            new_control_points.to_unweighted(),
            new_knot_vector,
            self.degree,
        )
        */
    }

    pub fn insert_knots(&self, num_new_knots: usize, u: f64) -> Self {
        let (new_knot_vector, new_control_points) = insert_curve_knots(
            self.degree,
            &self.knot_vector,
            num_new_knots,
            u,
            &self.control_points.to_weighted(),
        );

        Self::new(
            new_control_points.to_unweighted(),
            new_knot_vector,
            self.degree,
        )
    }

    pub fn example_refinement() -> Self {
        Self::new(
            ControlPolygon::new([
                vec4(-1.0, 1.0, 0.0, 1.0),
                vec4(-1.0, 0.0, 0.0, 3.0),
                vec4(0.0, -1.0, 0.0, 1.0),
                vec4(1.0, 0.0, 0.0, 3.0),
                vec4(1.0, 1.0, 0.0, 1.0),
            ]),
            KnotVector::new([0.0, 0.0, 0.0, 0.0, 2.0, 3.0, 3.0, 3.0, 3.0]),
            3,
        )
    }

    pub fn example_1() -> Self {
        Self::new(
            ControlPolygon::new([
                vec4(-1.0, 1.0, 0.0, 1.0),
                vec4(-1.0, 0.0, 0.0, 3.0),
                vec4(0.0, -1.0, 0.0, 1.0),
                vec4(1.0, 0.0, 0.0, 3.0),
                vec4(1.0, 1.0, 0.0, 1.0),
            ]),
            KnotVector::new([0.0, 0.0, 0.0, 0.0, 2.0, 3.0, 3.0, 3.0, 3.0]),
            3,
        )
    }

    pub fn example_circle() -> Self {
        Self::new(
            ControlPolygon::new([
                vec4(0.0, -1.0, 0.0, 1.0),
                vec4(-1.0, -1.0, 0.0, 2.0.sqrt() / 2.0),
                vec4(-1.0, 0.0, 0.0, 1.0),
                vec4(-1.0, 1.0, 0.0, 2.0.sqrt() / 2.0),
                vec4(0.0, 1.0, 0.0, 1.0),
                vec4(1.0, 1.0, 0.0, 2.0.sqrt() / 2.0),
                vec4(1.0, 0.0, 0.0, 1.0),
                vec4(1.0, -1.0, 0.0, 2.0.sqrt() / 2.0),
                vec4(0.0, -1.0, 0.0, 1.0),
            ]),
            KnotVector::new([
                0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0,
            ]),
            2,
        )
    }
}
impl CurveFunction for NurbsCurve {
    fn min_u(&self) -> f64 {
        self.knot_vector[0]
    }

    fn max_u(&self) -> f64 {
        self.knot_vector[self.knot_vector.len() - 1]
    }

    fn point(&self, u: f64) -> Vector3<f64> {
        let first = false;
        if first {
            nurbs::curve_point(&self.control_points, self.degree, &self.knot_vector, u)
        } else {
            let num_derivatives = 2;

            let weighted_derivatives = curve_derivatives_2(
                &self.control_points.to_weighted(),
                self.degree,
                &self.knot_vector,
                num_derivatives,
                u,
            );

            let derivatives = nurbs::curve_derivatives(&weighted_derivatives, num_derivatives);

            derivatives[0]
        }
    }
}
