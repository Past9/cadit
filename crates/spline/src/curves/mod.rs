use std::time::Instant;

use cgmath::Vector3;

use crate::math::FloatRange;

pub mod nurbs;

pub trait CurveFunction {
    fn min_u(&self) -> f64;
    fn max_u(&self) -> f64;

    fn point(&self, u: f64) -> Vector3<f64>;

    fn create(&self, u_res: usize) -> Vec<Vector3<f64>> {
        let mut points: Vec<Vector3<f64>> = Vec::new();

        let start = Instant::now();
        for u in FloatRange::new(self.min_u(), self.max_u(), u_res) {
            points.push(self.point(u));
        }
        let end = Instant::now();

        println!("{} points in {}μs", points.len(), (end - start).as_micros());

        points
    }
}
