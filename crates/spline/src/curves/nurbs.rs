use crate::{
    control_points::{self, ControlPolygon},
    knots::KnotVector,
    math::{
        b_spline::{curve_derivative_control_points, curve_derivatives_2},
        nurbs::{self, insert_curve_knots, refine_curve},
        Float, FloatRange, HPoint3, Point3, Vec2, Vec4, Vector,
    },
};

use super::CurveFunction;

pub struct NurbsCurve {
    /// Points that the curve interpolates between
    pub control_points: ControlPolygon,

    /// Knot vector
    pub knot_vector: KnotVector,

    /// Degree of curve
    degree: usize,
}
impl NurbsCurve {
    pub fn new(control_points: ControlPolygon, knot_vector: KnotVector, degree: usize) -> Self {
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

    /*
    pub fn derivative_curve(&self) -> NurbsCurve {
        self.derivative_curves(1).swap_remove(1)
    }

    pub fn derivative_curves(&self, num_derivates: usize) -> Vec<NurbsCurve> {
        let derivatives = curve_derivative_control_points(
            &self.control_points,
            self.degree,
            &self.knot_vector,
            0,
            self.control_points.len() - 1,
            num_derivates,
        );

        println!("DERS {:#?}", derivatives);

        derivatives
            .into_iter()
            .enumerate()
            .map(|(i, mut control_points)| {
                control_points.truncate(self.control_points.len() - i);
                Self::new(
                    control_points,
                    self.knot_vector
                        .iter()
                        .skip(i)
                        .take(self.knot_vector.len() - i * 2)
                        .cloned()
                        .collect(),
                    self.degree - i,
                )
            })
            .collect()
    }

    pub fn project_point(&self, point: Vec3, guess: f64) -> f64 {
        let der = self.derivative_curve();
        let der_pt = der.point(guess);
        let self_pt = self.point(guess);

        println!("{:?} {:?} {:?}", der_pt, self_pt, point);

        let f = der_pt.dot(&(self_pt - point));

        f
    }
    */

    pub fn refine(&self, new_knots: KnotVector) -> Self {
        let (new_knot_vector, new_control_points) = refine_curve(
            self.degree,
            &self.knot_vector,
            new_knots,
            &self.control_points.weight(),
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

    pub fn insert_knots(&self, num_new_knots: usize, u: Float) -> Self {
        let (new_knot_vector, new_control_points) = insert_curve_knots(
            self.degree,
            &self.knot_vector,
            num_new_knots,
            u,
            &self.control_points.weight(),
        );

        Self::new(
            new_control_points.to_unweighted(),
            new_knot_vector,
            self.degree,
        )
    }

    pub fn example_simple() -> Self {
        Self::new(
            ControlPolygon::new([
                HPoint3::new(-2.0, 2.0, 0.0, 1.0),
                HPoint3::new(0.0, -2.0, 0.0, 1.0),
                HPoint3::new(2.0, 2.0, 0.0, 1.0),
            ]),
            KnotVector::new([0.0, 0.0, 0.0, 1.0, 1.0, 1.0]),
            2,
        )
    }

    pub fn example_refinement() -> Self {
        Self::new(
            ControlPolygon::new([
                HPoint3::new(-1.0, 1.0, 0.0, 1.0),
                HPoint3::new(-1.0, 0.0, 0.0, 3.0),
                HPoint3::new(0.0, -1.0, 0.0, 1.0),
                HPoint3::new(1.0, 0.0, 0.0, 3.0),
                HPoint3::new(1.0, 1.0, 0.0, 1.0),
            ]),
            KnotVector::new([0.0, 0.0, 0.0, 0.0, 2.0, 3.0, 3.0, 3.0, 3.0]),
            3,
        )
    }

    pub fn example_1() -> Self {
        Self::new(
            ControlPolygon::new([
                HPoint3::new(-1.0, 1.0, 0.0, 1.0),
                HPoint3::new(-1.0, 0.0, 0.0, 3.0),
                HPoint3::new(0.0, -1.0, 0.0, 1.0),
                HPoint3::new(1.0, 0.0, 0.0, 3.0),
                HPoint3::new(1.0, 1.0, 0.0, 1.0),
            ]),
            KnotVector::new([0.0, 0.0, 0.0, 0.0, 2.0, 3.0, 3.0, 3.0, 3.0]),
            3,
        )
    }

    pub fn example_circle() -> Self {
        Self::new(
            ControlPolygon::new([
                HPoint3::new(0.0, -1.0, 0.0, 1.0),
                HPoint3::new(-1.0, -1.0, 0.0, (2.0 as Float).sqrt() / 2.0),
                HPoint3::new(-1.0, 0.0, 0.0, 1.0),
                HPoint3::new(-1.0, 1.0, 0.0, (2.0 as Float).sqrt() / 2.0),
                HPoint3::new(0.0, 1.0, 0.0, 1.0),
                HPoint3::new(1.0, 1.0, 0.0, (2.0 as Float).sqrt() / 2.0),
                HPoint3::new(1.0, 0.0, 0.0, 1.0),
                HPoint3::new(1.0, -1.0, 0.0, (2.0 as Float).sqrt() / 2.0),
                HPoint3::new(0.0, -1.0, 0.0, 1.0),
            ]),
            KnotVector::new([
                0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0,
            ]),
            2,
        )
    }

    pub fn points(&self, res_u: usize) -> Vec<Point3> {
        FloatRange::new(self.min_u(), self.max_u(), res_u)
            .map(|u| self.point(u))
            .collect::<Vec<_>>()
    }
}
impl CurveFunction for NurbsCurve {
    fn min_u(&self) -> Float {
        self.knot_vector[0]
    }

    fn max_u(&self) -> Float {
        self.knot_vector[self.knot_vector.len() - 1]
    }

    fn point(&self, u: Float) -> Point3 {
        println!(
            "POINT FN {:?} {} {:?}",
            self.control_points, self.degree, self.knot_vector
        );
        nurbs::curve_point(&self.control_points, self.degree, &self.knot_vector, u)
    }
}
