use space::{ESpace, EVec2, EVector, HSpace, HSpace2, HSpace3, HVec2, HVec3, HVector};

use crate::{
    bezier_curve::BezierCurve,
    math::{
        b_spline::{curve_derivative_control_points, curve_derivatives_1, curve_derivatives_2},
        knot_vector::KnotVector,
        nurbs::{curve_decompose, curve_derivatives, curve_point},
    },
};

#[derive(Debug)]
pub struct ClosestResult<E: EVector> {
    pub u: f64,
    pub closest_point: E,
    pub distance: f64,
    pub iterations: usize,
}

#[derive(Debug)]
pub struct NurbsCurve<H: HVector> {
    control_points: Vec<H>,
    knot_vector: KnotVector,
}
impl<H: HVector> NurbsCurve<H> {
    pub fn new(control_points: Vec<H>, knot_vector: KnotVector) -> Self {
        Self {
            control_points,
            knot_vector,
        }
    }

    pub fn control_points(&self) -> &[H] {
        &self.control_points
    }

    pub fn knot_vector(&self) -> &KnotVector {
        &self.knot_vector
    }

    pub fn decompose(&self) -> Vec<BezierCurve<H>> {
        curve_decompose(
            &self
                .control_points
                .iter()
                .map(|p| H::weight(p))
                .collect::<Vec<_>>(),
            self.degree(),
            &self.knot_vector,
        )
        .into_iter()
        .map(|pts| BezierCurve::new(pts.iter().map(|p| H::unweight(*p)).collect()))
        .collect()
    }

    pub fn find_closest(
        &self,
        point: H::Projected,
        u_guess: f64,
        max_iter: usize,
    ) -> Option<ClosestResult<H::Projected>> {
        const TOL: f64 = 0.0000001;
        let mut u = u_guess;
        for i in 0..max_iter {
            let ders = self.derivatives(u, 2);

            let vec_to_actual = ders[0] - point;
            let dot_err = ders[1].dot(&vec_to_actual);

            if dot_err.abs() <= TOL {
                return Some(ClosestResult {
                    u,
                    closest_point: ders[0],
                    distance: vec_to_actual.magnitude(),
                    iterations: i,
                });
            } else {
                let numerator = ders[1].dot(&vec_to_actual);
                let denominator = ders[2].dot(&vec_to_actual) + ders[1].magnitude2();
                u -= numerator / denominator;
            }
        }

        None
    }

    pub fn u_at_control_point(&self, index: usize) -> f64 {
        let num_ctrl_pts = self.control_points.len();
        assert!(
            index < num_ctrl_pts,
            "Invalid control point index {index} ({num_ctrl_pts} control points)"
        );

        let end = self.degree() + index;
        let knot_before = self.knot_vector[end - 1];
        let knot_after = self.knot_vector[end];

        (knot_before + knot_after) / 2.0
    }

    pub fn derivative_curve(&self, der: usize) -> Self {
        let degree = self.degree();
        assert!(
            der <= degree,
            "Derivative must not be greater than degree (degree = {}, derivative = {})",
            degree,
            der
        );

        let cpts = curve_derivative_control_points(
            &self
                .control_points
                .iter()
                .map(|cp| cp.weight())
                .collect::<Vec<_>>(),
            degree,
            &self.knot_vector,
            0,
            self.control_points.len() - 1,
            der,
        );

        Self::new(
            cpts.into_iter()
                .last()
                .unwrap()
                .into_iter()
                .take(self.control_points.len() - der)
                .map(H::cast_from_weighted)
                .collect(),
            self.knot_vector
                .iter()
                .skip(der)
                .take(self.knot_vector.len() - 2 * der)
                .cloned()
                .collect(),
        )
    }

    pub fn degree(&self) -> usize {
        self.knot_vector.len() - self.control_points.len() - 1
    }

    pub fn point(&self, u: f64) -> H::Projected {
        curve_point(&self.control_points, self.degree(), &self.knot_vector, u)
    }

    pub fn derivatives(&self, u: f64, num_ders: usize) -> Vec<H::Projected> {
        let ders = curve_derivatives_1(
            &self
                .control_points
                .iter()
                .map(|p| p.weight())
                .collect::<Vec<_>>(),
            self.degree(),
            &self.knot_vector,
            num_ders,
            u,
        )
        .into_iter()
        .map(H::cast_from_weighted)
        .collect::<Vec<_>>();

        curve_derivatives(&ders, num_ders)
    }

    pub fn derivative(&self, u: f64, der: usize) -> H::Projected {
        let ders = curve_derivatives_2(
            &self
                .control_points
                .iter()
                .map(|p| p.weight())
                .collect::<Vec<_>>(),
            self.degree(),
            &self.knot_vector,
            der,
            u,
        )
        .into_iter()
        .map(H::cast_from_weighted)
        .collect::<Vec<_>>();

        curve_derivatives(&ders, der)[der]
    }

    pub fn dist_func(&self, point: H::Projected, u: f64) -> f64 {
        let der = self.derivative(u, 1);
        let curve_point = self.point(u);

        der.dot(&(curve_point - point))
    }

    pub fn tangent(&self, u: f64) -> H::Projected {
        self.derivative(u, 1).normalize()
    }

    pub fn min_u(&self) -> f64 {
        self.knot_vector[0]
    }

    pub fn max_u(&self) -> f64 {
        self.knot_vector[self.knot_vector.len() - 1]
    }
}
impl NurbsCurve<HVec2> {
    pub fn normal(&self, u: f64) -> EVec2 {
        let tan = self.tangent(u);
        EVec2::new(tan.y, -tan.x)
    }

    pub fn example_quarter_circle() -> Self {
        Self::new(
            Vec::from([
                HVec2::new(-1.0, 0.0, 1.0),
                HVec2::new(-1.0, -1.0, 2.0_f64.sqrt() / 2.0),
                HVec2::new(0.0, -1.0, 1.0),
            ]),
            KnotVector::new([0.0, 0.0, 0.0, 1.0, 1.0, 1.0]),
        )
    }

    pub fn example_half_circle() -> Self {
        Self::new(
            Vec::from([
                HVec2::new(-1.0, 0.0, 1.0),
                HVec2::new(-1.0, -1.0, 2.0_f64.sqrt() / 2.0),
                HVec2::new(0.0, -1.0, 1.0),
                HVec2::new(1.0, -1.0, 2.0_f64.sqrt() / 2.0),
                HVec2::new(1.0, 0.0, 1.0),
            ]),
            KnotVector::new([0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0]),
        )
    }

    pub fn example_circle() -> Self {
        Self::new(
            Vec::from([
                HVec2::new(-1.0, 0.0, 1.0),
                HVec2::new(-1.0, -1.0, 2.0_f64.sqrt() / 2.0),
                HVec2::new(0.0, -1.0, 1.0),
                HVec2::new(1.0, -1.0, 2.0_f64.sqrt() / 2.0),
                HVec2::new(1.0, 0.0, 1.0),
                HVec2::new(1.0, 1.0, 2.0_f64.sqrt() / 2.0),
                HVec2::new(0.0, 1.0, 1.0),
                HVec2::new(-1.0, 1.0, 2.0_f64.sqrt() / 2.0),
                HVec2::new(-1.0, 0.0, 1.0),
            ]),
            KnotVector::new([
                0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0,
            ]),
        )
    }
}
impl NurbsCurve<HVec3> {
    pub fn example_quarter_circle() -> Self {
        Self::new(
            Vec::from([
                HVec3::new(-1.0, 0.0, 0.0, 1.0),
                HVec3::new(-1.0, -1.0, 0.0, 2.0_f64.sqrt() / 2.0),
                HVec3::new(0.0, -1.0, 0.0, 1.0),
            ]),
            KnotVector::new([0.0, 0.0, 0.0, 1.0, 1.0, 1.0]),
        )
    }

    pub fn example_half_circle() -> Self {
        Self::new(
            Vec::from([
                HVec3::new(-1.0, 0.0, 0.0, 1.0),
                HVec3::new(-1.0, -1.0, 0.0, 2.0_f64.sqrt() / 2.0),
                HVec3::new(0.0, -1.0, 0.0, 1.0),
                HVec3::new(1.0, -1.0, 0.0, 2.0_f64.sqrt() / 2.0),
                HVec3::new(1.0, 0.0, 0.0, 1.0),
            ]),
            KnotVector::new([0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0]),
        )
    }

    pub fn example_circle() -> Self {
        Self::new(
            Vec::from([
                HVec3::new(-1.0, 0.0, 0.0, 1.0),
                HVec3::new(-1.0, -1.0, 0.0, 2.0_f64.sqrt() / 2.0),
                HVec3::new(0.0, -1.0, 0.0, 1.0),
                HVec3::new(1.0, -1.0, 0.0, 2.0_f64.sqrt() / 2.0),
                HVec3::new(1.0, 0.0, 0.0, 1.0),
                HVec3::new(1.0, 1.0, 0.0, 2.0_f64.sqrt() / 2.0),
                HVec3::new(0.0, 1.0, 0.0, 1.0),
                HVec3::new(-1.0, 1.0, 0.0, 2.0_f64.sqrt() / 2.0),
                HVec3::new(-1.0, 0.0, 0.0, 1.0),
            ]),
            KnotVector::new([
                0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0,
            ]),
        )
    }
}
