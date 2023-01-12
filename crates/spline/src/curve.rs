use crate::math::{
    knot_vector::KnotVector,
    nurbs::{
        curve_derivative_control_points, curve_derivatives_1, curve_derivatives_2, curve_point,
    },
    HPoint, Point,
};

type ControlPoints = Vec<HPoint>;

pub struct Curve {
    control_points: ControlPoints,
    knot_vector: KnotVector,
}
impl Curve {
    pub fn new(control_points: ControlPoints, knot_vector: KnotVector) -> Self {
        Self {
            control_points,
            knot_vector,
        }
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
                .map(|cp| cp.to_wpoint())
                .collect::<Vec<_>>(),
            degree,
            &self.knot_vector,
            0,
            self.control_points.len() - 1,
            der,
        );

        println!("CPTS {:#?}", cpts);

        Self::new(
            cpts.into_iter()
                .last()
                .unwrap()
                .into_iter()
                .take(self.control_points.len() - der)
                .map(|pt| pt.to_unweighted())
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

    pub fn point(&self, u: f64) -> Point {
        curve_point(&self.control_points, self.degree(), &self.knot_vector, u)
    }

    pub fn derivative(&self, u: f64, der: usize) -> Point {
        curve_derivatives_1(
            &self.control_points,
            self.degree(),
            &self.knot_vector,
            der,
            u,
        )[der]
            .cartesian()
    }

    pub fn min_u(&self) -> f64 {
        self.knot_vector[0]
    }

    pub fn max_u(&self) -> f64 {
        self.knot_vector[self.knot_vector.len() - 1]
    }

    pub fn example_quarter_circle() -> Self {
        Self::new(
            ControlPoints::from([
                HPoint::new(-1.0, 0.0, 0.0, 1.0),
                HPoint::new(-1.0, -1.0, 0.0, 2.0_f64.sqrt() / 2.0),
                HPoint::new(0.0, -1.0, 0.0, 1.0),
            ]),
            KnotVector::new([0.0, 0.0, 0.0, 1.0, 1.0, 1.0]),
        )
    }

    pub fn example_half_circle() -> Self {
        Self::new(
            ControlPoints::from([
                HPoint::new(-1.0, 0.0, 0.0, 1.0),
                HPoint::new(-1.0, -1.0, 0.0, 2.0_f64.sqrt() / 2.0),
                HPoint::new(0.0, -1.0, 0.0, 1.0),
                HPoint::new(1.0, -1.0, 0.0, 2.0_f64.sqrt() / 2.0),
                HPoint::new(1.0, 0.0, 0.0, 1.0),
            ]),
            KnotVector::new([0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0]),
        )
    }

    pub fn example_circle() -> Self {
        Self::new(
            ControlPoints::from([
                HPoint::new(-1.0, 0.0, 0.0, 1.0),
                HPoint::new(-1.0, -1.0, 0.0, 2.0_f64.sqrt() / 2.0),
                HPoint::new(0.0, -1.0, 0.0, 1.0),
                HPoint::new(1.0, -1.0, 0.0, 2.0_f64.sqrt() / 2.0),
                HPoint::new(1.0, 0.0, 0.0, 1.0),
                HPoint::new(1.0, 1.0, 0.0, 2.0_f64.sqrt() / 2.0),
                HPoint::new(0.0, 1.0, 0.0, 1.0),
                HPoint::new(-1.0, 1.0, 0.0, 2.0_f64.sqrt() / 2.0),
                HPoint::new(-1.0, 0.0, 0.0, 1.0),
            ]),
            KnotVector::new([
                0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0,
            ]),
        )
    }
}