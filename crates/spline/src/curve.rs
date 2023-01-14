use crate::math::{
    b_spline::{curve_derivative_control_points, curve_derivatives_2},
    knot_vector::KnotVector,
    nurbs::{curve_derivatives, curve_point},
    Homogeneous, Point, Vec2H, Vec3H, Vector,
};

pub struct ClosestResult {
    pub u: f64,
    pub closest_point: Point,
    pub dist: f64,
}

pub struct Curve<H: Homogeneous> {
    control_points: Vec<H>,
    knot_vector: KnotVector,
}
impl<H: Homogeneous> Curve<H> {
    pub fn new(control_points: Vec<H>, knot_vector: KnotVector) -> Self {
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
                .map(|cp| cp.weight())
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

        let mut ders = curve_derivatives(&ders, der);

        ders.swap_remove(1)
    }

    pub fn closest(&self, point: H::Projected, u: f64) -> f64 {
        let der = self.derivative(u, 1);
        let curve_point = self.point(u);

        der.dot(&(curve_point - point))
    }

    pub fn tangent(&self, u: f64) -> H::Projected {
        self.derivative(u, 1).normalize()
    }

    /*
    pub fn normal(&self, u: f64) -> H::Projected {
        let tan = self.tangent(u);
        Point {
            x: -tan.y,
            y: tan.x,
            z: tan.z,
        }
    }
    */

    pub fn min_u(&self) -> f64 {
        self.knot_vector[0]
    }

    pub fn max_u(&self) -> f64 {
        self.knot_vector[self.knot_vector.len() - 1]
    }

    /*
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
    */
}
impl Curve<Vec2H> {
    pub fn example_quarter_circle() -> Self {
        Self::new(
            Vec::from([
                Vec2H::new(-1.0, 0.0, 1.0),
                Vec2H::new(-1.0, -1.0, 2.0_f64.sqrt() / 2.0),
                Vec2H::new(0.0, -1.0, 1.0),
            ]),
            KnotVector::new([0.0, 0.0, 0.0, 1.0, 1.0, 1.0]),
        )
    }

    pub fn example_half_circle() -> Self {
        Self::new(
            Vec::from([
                Vec2H::new(-1.0, 0.0, 1.0),
                Vec2H::new(-1.0, -1.0, 2.0_f64.sqrt() / 2.0),
                Vec2H::new(0.0, -1.0, 1.0),
                Vec2H::new(1.0, -1.0, 2.0_f64.sqrt() / 2.0),
                Vec2H::new(1.0, 0.0, 1.0),
            ]),
            KnotVector::new([0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0]),
        )
    }

    pub fn example_circle() -> Self {
        Self::new(
            Vec::from([
                Vec2H::new(-1.0, 0.0, 1.0),
                Vec2H::new(-1.0, -1.0, 2.0_f64.sqrt() / 2.0),
                Vec2H::new(0.0, -1.0, 1.0),
                Vec2H::new(1.0, -1.0, 2.0_f64.sqrt() / 2.0),
                Vec2H::new(1.0, 0.0, 1.0),
                Vec2H::new(1.0, 1.0, 2.0_f64.sqrt() / 2.0),
                Vec2H::new(0.0, 1.0, 1.0),
                Vec2H::new(-1.0, 1.0, 2.0_f64.sqrt() / 2.0),
                Vec2H::new(-1.0, 0.0, 1.0),
            ]),
            KnotVector::new([
                0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0,
            ]),
        )
    }
}
impl Curve<Vec3H> {
    pub fn example_quarter_circle() -> Self {
        Self::new(
            Vec::from([
                Vec3H::new(-1.0, 0.0, 0.0, 1.0),
                Vec3H::new(-1.0, -1.0, 0.0, 2.0_f64.sqrt() / 2.0),
                Vec3H::new(0.0, -1.0, 0.0, 1.0),
            ]),
            KnotVector::new([0.0, 0.0, 0.0, 1.0, 1.0, 1.0]),
        )
    }

    pub fn example_half_circle() -> Self {
        Self::new(
            Vec::from([
                Vec3H::new(-1.0, 0.0, 0.0, 1.0),
                Vec3H::new(-1.0, -1.0, 0.0, 2.0_f64.sqrt() / 2.0),
                Vec3H::new(0.0, -1.0, 0.0, 1.0),
                Vec3H::new(1.0, -1.0, 0.0, 2.0_f64.sqrt() / 2.0),
                Vec3H::new(1.0, 0.0, 0.0, 1.0),
            ]),
            KnotVector::new([0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0]),
        )
    }

    pub fn example_circle() -> Self {
        Self::new(
            Vec::from([
                Vec3H::new(-1.0, 0.0, 0.0, 1.0),
                Vec3H::new(-1.0, -1.0, 0.0, 2.0_f64.sqrt() / 2.0),
                Vec3H::new(0.0, -1.0, 0.0, 1.0),
                Vec3H::new(1.0, -1.0, 0.0, 2.0_f64.sqrt() / 2.0),
                Vec3H::new(1.0, 0.0, 0.0, 1.0),
                Vec3H::new(1.0, 1.0, 0.0, 2.0_f64.sqrt() / 2.0),
                Vec3H::new(0.0, 1.0, 0.0, 1.0),
                Vec3H::new(-1.0, 1.0, 0.0, 2.0_f64.sqrt() / 2.0),
                Vec3H::new(-1.0, 0.0, 0.0, 1.0),
            ]),
            KnotVector::new([
                0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0,
            ]),
        )
    }
}
