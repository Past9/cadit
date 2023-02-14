use iter_tools::Itertools;
use once_cell::unsync::OnceCell;
use space::{
    hspace::{HSpace, HSpace3},
    EVec2, EVector, HVec3, HVector,
};

use crate::math::{
    bezier::{decasteljau2, newton, rational_surface_derivatives},
    FloatRange,
};

pub struct BezierSurface<H: HSpace> {
    control_points: Vec<Vec<H::Vector>>,
    weighted_control_points: OnceCell<Vec<Vec<H::WeightedVector>>>,
}
impl<H: HSpace> BezierSurface<H> {
    pub fn new(control_points: Vec<Vec<H::Vector>>) -> Self {
        Self {
            control_points,
            weighted_control_points: OnceCell::new(),
        }
    }

    pub fn weighted_control_points(&self) -> &[Vec<H::WeightedVector>] {
        self.weighted_control_points.get_or_init(|| {
            self.control_points
                .iter()
                .map(|points| {
                    points
                        .iter()
                        .map(|p| H::weight_vec(p.to_owned()))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
    }

    pub fn point(&self, u: f64, v: f64) -> H::ProjectedVector {
        let p = decasteljau2(self.weighted_control_points(), u, v);
        H::project_vec(H::cast_vec_from_weighted(p))
    }

    pub fn derivatives(&self, u: f64, v: f64, num_ders: usize) -> Vec<Vec<H::ProjectedVector>> {
        rational_surface_derivatives::<H>(&self.weighted_control_points(), num_ders, u, v)
    }

    pub fn degree_u(&self) -> usize {
        self.control_points.len() - 1
    }

    pub fn degree_v(&self) -> usize {
        self.control_points[0].len() - 1
    }

    pub fn translate(&mut self, vec: H::Vector) {
        // TODO: Use transformation matrices

        let mut new_pts = Vec::new();
        for row in self.control_points.iter_mut() {
            let mut new_row = Vec::new();
            for point in row.iter_mut() {
                new_row.push(*point + vec);
            }
            new_pts.push(new_row);
        }
        self.control_points = new_pts;
    }

    pub fn hausdorff_candidates(
        &self,
        plane: &H::EuclideanPlane,
    ) -> Vec<(EVec2, H::ProjectedVector)> {
        let try_point = |uv: EVec2| {
            newton(uv, 100, |uv| {
                let ders: Vec<Vec<H::ProjectedVector>> = rational_surface_derivatives::<H>(
                    &self.weighted_control_points(),
                    2,
                    uv.x,
                    uv.y,
                );

                let closest = H::plane_closest_to_point(plane, &ders[0][0]);
                let between = closest - ders[0][0];

                let d1_norm_u = ders[0][1].normalize();
                let d1_norm_v = ders[1][0].normalize();

                let num_u = ders[0][1].dot(&between);
                let num_v = ders[1][0].dot(&between);

                let denom_u = ders[0][2].dot(&between) + d1_norm_u.dot(&d1_norm_u);
                let denom_v = ders[2][0].dot(&between) + d1_norm_v.dot(&d1_norm_v);

                (EVec2::new(num_u, num_v), EVec2::new(denom_u, denom_v))
            })
        };

        let mut params: Vec<EVec2> = Vec::new();

        let divisions = 10;
        let initial_uvs = FloatRange::new(0.0, 1.0, divisions)
            .into_iter()
            .cartesian_product(FloatRange::new(0.0, 1.0, divisions))
            .map(|(u, v)| EVec2::new(u, v))
            .collect::<Vec<_>>();

        //let initial_uvs = vec![(0.4, 0.4)];

        for uv_initial in initial_uvs.into_iter() {
            if let Some(uv) = try_point(uv_initial) {
                params.push(uv)
            }
        }

        let mut points = Vec::new();

        points.extend(params.into_iter().map(|uv| (uv, self.point(uv.x, uv.y))));

        points
    }
}
impl BezierSurface<HSpace3> {
    pub fn example_simple() -> Self {
        Self::new(Vec::from([
            Vec::from([
                HVec3::new(-1.0, 0.0, -1.0, 1.0),
                HVec3::new(0.0, 0.0, -1.0, 1.0),
                HVec3::new(1.0, 0.0, -1.0, 1.0),
            ]),
            Vec::from([
                HVec3::new(-1.0, 0.0, 0.0, 1.0),
                HVec3::new(0.0, -2.0, 0.0, 2.0),
                HVec3::new(1.0, 0.0, 0.0, 1.0),
            ]),
            Vec::from([
                HVec3::new(-1.0, 0.0, 1.0, 1.0),
                HVec3::new(0.0, 0.0, 1.0, 1.0),
                HVec3::new(1.0, 0.0, 1.0, 1.0),
            ]),
        ]))
    }

    pub fn example_eighth_sphere() -> Self {
        let rt2 = 2.0_f64.sqrt() / 2.0;
        Self::new(Vec::from([
            Vec::from([
                HVec3::new(-1.0, 0.0, 0.0, 1.0),
                HVec3::new(-1.0, -1.0, 0.0, 2.0_f64.sqrt() / 2.0),
                HVec3::new(0.0, -1.0, 0.0, 1.0),
            ]),
            Vec::from([
                HVec3::new(-rt2, 0.0, rt2, 1.0),
                HVec3::new(-rt2, -1.0, rt2, 2.0_f64.sqrt() / 2.0),
                HVec3::new(0.0, -1.0, 0.0, 1.0),
            ]),
            Vec::from([
                HVec3::new(0.0, 0.0, 1.0, 1.0),
                HVec3::new(0.0, -1.0, 1.0, 2.0_f64.sqrt() / 2.0),
                HVec3::new(0.0, -1.0, 0.0, 1.0),
            ]),
        ]))
    }
}
