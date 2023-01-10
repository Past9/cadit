use std::time::Instant;

use crate::math::{Float, FloatRange, Point3};

pub mod nurbs;

pub trait CurveFunction {
    fn min_u(&self) -> Float;
    fn max_u(&self) -> Float;

    fn point(&self, u: Float) -> Point3;

    fn create(&self, u_res: usize) -> Vec<Point3> {
        let mut points: Vec<Point3> = Vec::new();

        let start = Instant::now();
        for u in FloatRange::new(self.min_u(), self.max_u(), u_res) {
            points.push(self.point(u));
        }
        let end = Instant::now();

        println!("{} points in {}μs", points.len(), (end - start).as_micros());

        points
    }
}
