use std::time::Instant;

use crate::math::{Float, FloatRange, Vec3};

pub mod nurbs;

pub trait CurveFunction {
    fn min_u(&self) -> Float;
    fn max_u(&self) -> Float;

    fn point(&self, u: Float) -> Vec3;

    fn create(&self, u_res: usize) -> Vec<Vec3> {
        let mut points: Vec<Vec3> = Vec::new();

        let start = Instant::now();
        for u in FloatRange::new(self.min_u(), self.max_u(), u_res) {
            points.push(self.point(u));
        }
        let end = Instant::now();

        println!("{} points in {}μs", points.len(), (end - start).as_micros());

        points
    }
}
