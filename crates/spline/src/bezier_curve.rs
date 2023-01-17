use std::collections::HashSet;

use crate::math::{bezier::decasteljau, FloatRange, Homogeneous, Vec2, Vec2H, Vector};

const TOL: f64 = 0.0000000001;

#[derive(Debug, Clone)]
pub struct Line2D {
    a: f64,
    b: f64,
    c: f64,
}
impl Line2D {
    pub fn from_pos_and_dir(pos: Vec2, dir: Vec2) -> Self {
        let dir = dir.normalize();

        let a = dir.y;
        let b = -dir.x;
        let c = dir.x * pos.y - dir.y * pos.x;

        Self { a, b, c }
    }

    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Self { a, b, c }
    }

    pub fn normalize(&self) -> Self {
        let vec_mag = (self.a.powi(2) + self.b.powi(2)).sqrt();
        Self {
            a: self.a / vec_mag,
            b: self.b / vec_mag,
            c: self.c,
        }
    }
}

#[derive(Debug)]
pub struct BezierCurve<H: Homogeneous> {
    control_points: Vec<H>,
}
impl<H: Homogeneous> BezierCurve<H> {
    pub fn new(control_points: Vec<H>) -> Self {
        Self { control_points }
    }

    pub fn point(&self, u: f64) -> H::Projected {
        let p = decasteljau(
            &self
                .control_points
                .iter()
                .map(|p| p.weight())
                .collect::<Vec<_>>(),
            u,
        );

        H::cast_from_weighted(p).project()
    }

    pub fn degree(&self) -> usize {
        self.control_points.len() - 1
    }

    pub fn derivative_curve(&self) -> Self {
        let mut der_points = Vec::new();
        let self_deg = self.degree() as f64;

        for i in 0..self.control_points.len() - 1 {
            der_points.push((self.control_points[i + 1] - self.control_points[i]) * self_deg);
        }

        Self::new(der_points)
    }
}
impl BezierCurve<Vec2H> {
    fn line_intersection_coefficients(&self, line: &Line2D) -> Vec<f64> {
        self.control_points
            .iter()
            .map(|pt| pt.x * line.a + pt.y * line.b + line.c)
            .collect::<Vec<_>>()
    }

    pub fn intersection_curve_plot(&self, line: &Line2D) -> Vec<Vec2> {
        let coefficients = self.line_intersection_coefficients(line);

        let mut points = Vec::new();
        for x in FloatRange::new(0.0, 1.0, 300) {
            let point = Vec2::new(x, decasteljau(&coefficients, x));
            points.push(point);
            //println!("{:?}", point);
        }

        points
    }

    fn line_intersection_nearest(
        self_coefficients: &[f64],
        der_coefficients: &[f64],
        u_guess: f64,
        max_iter: usize,
    ) -> Option<f64> {
        println!("GUESS {}", u_guess);
        let mut u = u_guess;
        for _ in 0..max_iter {
            let self_val = decasteljau(self_coefficients, u);
            let der_val = decasteljau(der_coefficients, u);

            println!("{} {}", u, self_val);

            if self_val.abs() <= TOL {
                return Some(u);
            } else {
                u -= self_val / der_val;
                if u < 0.0 {
                    u = 0.0;
                } else if u > 1.0 {
                    u = 1.0;
                }
            }
        }

        None
    }

    pub fn line_intersection_params(&self, line: &Line2D) -> Vec<f64> {
        let self_coefficients = self.line_intersection_coefficients(line);
        let der_coefficients = self.derivative_curve().line_intersection_coefficients(line);

        let mut intersections = Vec::new();

        let num_tests = self.degree();
        for i in 0..num_tests {
            let u = i as f64 / (num_tests - 1) as f64;
            if let Some(zero) =
                Self::line_intersection_nearest(&self_coefficients, &der_coefficients, u, 50)
            {
                intersections.push(zero);
            }
        }

        intersections
    }

    pub fn line_intersections(&self, line: &Line2D) -> Vec<Vec2> {
        let params = self.line_intersection_params(line);
        println!("PARAMS {:?}", params);
        let mut points = params
            .into_iter()
            .map(|u| self.point(u))
            .collect::<Vec<_>>();

        points.dedup_by(|a, b| (*a - *b).magnitude() <= TOL);

        points
    }

    pub fn line_deviations(&self, line: &Line2D) -> Vec<Vec2> {
        let params = self.derivative_curve().line_intersection_params(line);

        println!("PARAMS {:?}", params);
        let mut points = params
            .into_iter()
            .map(|u| self.point(u))
            .collect::<Vec<_>>();

        points.dedup_by(|a, b| (*a - *b).magnitude() <= TOL);

        points
    }
}
