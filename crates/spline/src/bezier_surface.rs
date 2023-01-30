use once_cell::unsync::OnceCell;
use space::{
    hspace::{HSpace, HSpace3},
    HVec3,
};

use crate::math::bezier::decasteljau2;

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

    pub fn degree_u(&self) -> usize {
        self.control_points.len() - 1
    }

    pub fn degree_v(&self) -> usize {
        self.control_points[0].len() - 1
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
                HVec3::new(0.0, -2.0, 0.0, 1.0),
                HVec3::new(1.0, 0.0, 0.0, 1.0),
            ]),
            Vec::from([
                HVec3::new(-1.0, 0.0, 1.0, 1.0),
                HVec3::new(0.0, 0.0, 1.0, 1.0),
                HVec3::new(1.0, 0.0, 1.0, 1.0),
            ]),
        ]))
    }
}
