use crate::{
    control_points::{ControlMesh, ControlPolygon},
    knots::KnotVector,
    math::{
        b_spline::surface_derivatives_2,
        nurbs::{self, insert_surface_knots},
        Float, FloatRange, HPoint3, Point3, Vec4,
    },
};

use super::SurfaceFunction;

#[derive(Clone, Copy, Debug)]
pub enum SurfaceDirection {
    U,
    V,
}
impl SurfaceDirection {
    pub fn name(&self) -> String {
        match self {
            SurfaceDirection::U => "U",
            SurfaceDirection::V => "V",
        }
        .into()
    }
}

pub struct NurbsSurface {
    pub control_points: ControlMesh,
    pub knot_vector_u: KnotVector,
    pub knot_vector_v: KnotVector,
    pub degree_u: usize,
    pub degree_v: usize,
}
impl NurbsSurface {
    pub fn new(
        control_points: ControlMesh,
        knot_vector_u: KnotVector,
        knot_vector_v: KnotVector,
        degree_u: usize,
        degree_v: usize,
    ) -> Self {
        assert_eq!(
            knot_vector_u.len(),
            control_points.len() + degree_u + 1,
            "Incorrect degree ({}), knot vector length ({}), or number of control points ({}) for dimension U",
            degree_u,
            knot_vector_u.len(),
            control_points.len()
        );

        assert_eq!(
            knot_vector_v.len(),
            control_points[0].len() + degree_v + 1,
            "Incorrect degree ({}), knot vector length ({}), or number of control points ({}) for dimension V",
            degree_v,
            knot_vector_v.len(),
            control_points[0].len()
        );

        Self {
            control_points: control_points,
            knot_vector_u: knot_vector_u,
            knot_vector_v: knot_vector_v,
            degree_u,
            degree_v,
        }
    }

    pub fn split_at(&self, position: Float, direction: SurfaceDirection) -> (Self, Self) {
        match direction {
            SurfaceDirection::U => {
                let knot_multiplicity = self.knot_vector_u.find_multiplicity(position);
                let num_insertions = self.degree_u - knot_multiplicity;
                let split = self.insert_knots(num_insertions, position, direction);

                if let Some(knot_index) = split.knot_vector_u.find_index(position) {
                    let knot_span = self.knot_vector_u.find_span(
                        self.degree_u,
                        self.control_points.len(),
                        position,
                    );

                    let kv_len = knot_index + split.degree_u;
                    let cp_len = kv_len - split.degree_u - 1;

                    let surf1 = Self::new(
                        ControlMesh::from_slice(&split.control_points[..cp_len]),
                        KnotVector::from_slice(&split.knot_vector_u[..kv_len]),
                        split.knot_vector_v.clone(),
                        split.degree_u,
                        split.degree_v,
                    );

                    let surf2 = Self::new(
                        ControlMesh::from_slice(&split.control_points[knot_span + 1..]),
                        KnotVector::from_slice(&split.knot_vector_u[knot_index..]),
                        split.knot_vector_v.clone(),
                        split.degree_u,
                        split.degree_v,
                    );

                    (surf1, surf2)
                } else {
                    panic!(
                        "Could not find knots at {} position {} after splitting surface",
                        direction.name(),
                        position
                    );
                }
            }
            SurfaceDirection::V => {
                let knot_multiplicity = self.knot_vector_v.find_multiplicity(position);
                let num_insertions = self.degree_v - knot_multiplicity;
                let split = self.insert_knots(num_insertions, position, direction);

                if let Some(knot_index) = split.knot_vector_v.find_index(position) {
                    let knot_span = self.knot_vector_v.find_span(
                        self.degree_v,
                        self.control_points.len(),
                        position,
                    );

                    let kv_len = knot_index + split.degree_v;
                    let cp_len = kv_len - split.degree_v - 1;
                    let surf1 = Self::new(
                        ControlMesh::from_iter(
                            split
                                .control_points
                                .iter()
                                .map(|row| ControlPolygon::from_slice(&row[..cp_len])),
                        ),
                        split.knot_vector_u.clone(),
                        KnotVector::from_slice(&split.knot_vector_v[..kv_len]),
                        split.degree_u,
                        split.degree_v,
                    );

                    let surf2 = Self::new(
                        ControlMesh::from_iter(
                            split
                                .control_points
                                .iter()
                                .map(|row| ControlPolygon::from_slice(&row[knot_span + 1..])),
                        ),
                        split.knot_vector_u.clone(),
                        KnotVector::from_slice(&split.knot_vector_v[knot_index..]),
                        split.degree_u,
                        split.degree_v,
                    );

                    (surf1, surf2)
                } else {
                    panic!(
                        "Could not find knots at {} position {} after splitting surface",
                        direction.name(),
                        position
                    );
                }
            }
        }
    }

    pub fn insert_knots(
        &self,
        num_new_knots: usize,
        position: Float,
        direction: SurfaceDirection,
    ) -> Self {
        let (new_knot_vector_u, new_knot_vector_v, new_control_points) = insert_surface_knots(
            self.degree_u,
            self.degree_v,
            direction,
            &self.knot_vector_u,
            &self.knot_vector_v,
            num_new_knots,
            position,
            &self.control_points.weight(),
        );

        Self::new(
            new_control_points.to_unweighted(),
            new_knot_vector_u,
            new_knot_vector_v,
            self.degree_u,
            self.degree_v,
        )
    }

    pub fn example_warped_square() -> Self {
        Self::new(
            ControlMesh::new([
                ControlPolygon::new([
                    HPoint3::new(-2.0, -1.0, -2.0, 1.0),
                    HPoint3::new(2.0, 1.0, -2.0, 1.0),
                ]),
                ControlPolygon::new([
                    HPoint3::new(-2.0, 1.0, 2.0, 1.0),
                    HPoint3::new(2.0, -1.0, 2.0, 1.0),
                ]),
            ]),
            KnotVector::new([0.0, 0.0, 1.0, 1.0]),
            KnotVector::new([0.0, 0.0, 1.0, 1.0]),
            1,
            1,
        )
    }

    pub fn example_1() -> Self {
        let w = 0.25;
        Self::new(
            ControlMesh::new([
                ControlPolygon::new([
                    HPoint3::new(-3.0, 2.0, -3.0, 1.0),
                    HPoint3::new(-1.0, 2.0, -3.0, w),
                    HPoint3::new(1.0, 2.0, -3.0, w),
                    HPoint3::new(3.0, 2.0, -3.0, 1.0),
                ]),
                ControlPolygon::new([
                    HPoint3::new(-3.0, 2.0, -1.0, w),
                    HPoint3::new(-1.0, -2.0, -1.0, 1.0),
                    HPoint3::new(1.0, -2.0, -1.0, 1.0),
                    HPoint3::new(3.0, 2.0, -1.0, w),
                ]),
                ControlPolygon::new([
                    HPoint3::new(-3.0, 2.0, 1.0, w),
                    HPoint3::new(-1.0, -2.0, 1.0, 1.0),
                    HPoint3::new(1.0, -2.0, 1.0, 1.0),
                    HPoint3::new(3.0, 2.0, 1.0, w),
                ]),
                ControlPolygon::new([
                    HPoint3::new(-3.0, 2.0, 3.0, 1.0),
                    HPoint3::new(-1.0, 2.0, 3.0, w),
                    HPoint3::new(1.0, 2.0, 3.0, w),
                    HPoint3::new(3.0, 2.0, 3.0, 1.0),
                ]),
                ControlPolygon::new([
                    HPoint3::new(-3.0, 2.0, 5.0, 1.0),
                    HPoint3::new(-1.0, 2.0, 5.0, w),
                    HPoint3::new(1.0, 2.0, 5.0, w),
                    HPoint3::new(3.0, 2.0, 5.0, 1.0),
                ]),
            ]),
            KnotVector::new([0.0, 0.0, 0.0, 0.5, 1.0, 2.0, 2.0, 2.0]),
            KnotVector::new([0.0, 0.0, 0.0, 1.0, 2.0, 2.0, 2.0]),
            2,
            2,
        )
    }

    pub fn points(&self, res_u: usize, res_v: usize) -> Vec<Vec<Point3>> {
        let mut points = Vec::new();
        for u in FloatRange::new(self.min_u(), self.max_u(), res_u) {
            let v_points = FloatRange::new(self.min_v(), self.max_v(), res_v)
                .map(|v| self.point(u, v))
                .collect::<Vec<_>>();
            points.push(v_points);
        }

        points
    }
}
impl SurfaceFunction for NurbsSurface {
    fn min_u(&self) -> Float {
        self.knot_vector_u[0]
    }

    fn max_u(&self) -> Float {
        self.knot_vector_u[self.knot_vector_u.len() - 1]
    }

    fn min_v(&self) -> Float {
        self.knot_vector_v[0]
    }

    fn max_v(&self) -> Float {
        self.knot_vector_v[self.knot_vector_v.len() - 1]
    }

    fn point(&self, u: Float, v: Float) -> Point3 {
        let first = false;

        if first {
            nurbs::surface_point(
                &self.control_points,
                self.degree_u,
                self.degree_v,
                &self.knot_vector_u,
                &self.knot_vector_v,
                u,
                v,
            )
        } else {
            let num_derivatives = 0;

            let weighted_derivatives = surface_derivatives_2(
                &self.control_points.weight(),
                self.degree_u,
                self.degree_v,
                &self.knot_vector_u,
                &self.knot_vector_v,
                num_derivatives,
                u,
                v,
            );

            let derivatives = nurbs::surface_derivatives(&weighted_derivatives, num_derivatives);

            derivatives[0][0]
        }
    }
}
