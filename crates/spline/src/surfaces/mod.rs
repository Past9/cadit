use std::time::Instant;

use crate::math::{Float, FloatRange, Vec3, Vector};

pub mod nurbs;

/*
fn make_mesh_points(points: &[Vec<Vec3>]) -> Vec<Vec<SurfacePoint>> {
    let u_len = points.len();
    let v_len = points[0].len();

    let mut surface_points = vec![
        vec![
            SurfacePoint {
                pos: Vec3::zero(),
                der_u: Vec3::zero(),
                der_v: Vec3::zero(),
                normal: Vec3::zero()
            };
            v_len
        ];
        u_len
    ];

    for i in 0..u_len {
        for j in 0..v_len {
            let pos = points[i][j];

            let (before_u, after_u) = if i == 0 {
                let der = points[i + 1][j] - points[i][j];
                (der, der)
            } else if i == u_len - 1 {
                let der = points[i][j] - points[i - 1][j];
                (der, der)
            } else {
                (
                    points[i][j] - points[i - 1][j],
                    points[i + 1][j] - points[i][j],
                )
            };

            let (before_v, after_v) = if i == 0 {
                let der = points[i][j + 1] - points[i][j];
                (der, der)
            } else if i == u_len - 1 {
                let der = points[i][j] - points[i][j - 1];
                (der, der)
            } else {
                (
                    points[i][j] - points[i][j - 1],
                    points[i][j + 1] - points[i][j],
                )
            };

            let der_u = (before_u + after_u).normalize();
            let der_v = (before_v + after_v).normalize();

            let normal = der_u.cross(&der_v).normalize();

            surface_points[i][j] = SurfacePoint {
                pos,
                der_u,
                der_v,
                normal,
            };
        }
    }

    todo!()
}
*/

#[derive(Clone, Debug)]
pub struct SurfacePoint {
    pub pos: Vec3,
    pub der_u: Vec3,
    pub der_v: Vec3,
    pub normal: Vec3,
}
impl SurfacePoint {
    pub fn new(pos: Vec3, der_u: Vec3, der_v: Vec3) -> Self {
        Self {
            pos,
            der_u,
            der_v,
            normal: der_u.normalize().cross(&der_v.normalize()).normalize(),
        }
    }
}

pub trait SurfaceFunction {
    fn min_u(&self) -> Float;
    fn max_u(&self) -> Float;
    fn min_v(&self) -> Float;
    fn max_v(&self) -> Float;

    fn point(&self, u: Float, v: Float) -> Vec3;

    fn create(&self, u_res: usize, v_res: usize) -> Vec<Vec3> {
        let mut points = Vec::new();

        let start = Instant::now();
        for u in FloatRange::new(self.min_u(), self.max_u(), u_res as usize) {
            for v in FloatRange::new(self.min_v(), self.max_v(), v_res as usize) {
                points.push(self.point(u, v));
            }
        }
        let end = Instant::now();

        println!("{} points in {}μs", points.len(), (end - start).as_micros());

        points
    }
}
